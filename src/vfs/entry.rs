use std::path::PathBuf;
use std::time::SystemTime;

use ratatui::style::Color;

use crate::theme::icons;
use crate::theme::palette::Palette;

/// Represents a single filesystem entry with all metadata needed for display.
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// Display name (basename).
    pub name: String,
    /// Full absolute path.
    pub path: PathBuf,
    /// Is this a directory?
    pub is_dir: bool,
    /// Is this a symbolic link?
    pub is_symlink: bool,
    /// Is this a broken (orphaned) symlink?
    pub is_broken_symlink: bool,
    /// Is this a hidden file (starts with '.')?
    pub is_hidden: bool,
    /// File size in bytes (0 for directories).
    pub size: u64,
    /// Last modification time.
    pub modified: Option<SystemTime>,
    /// Unix permission bits (e.g. 0o755).
    pub permissions: u32,
    /// Owner username.
    pub owner: String,
    /// Group name.
    pub group: String,
    /// Nerd Font icon glyph.
    pub icon: &'static str,
    /// Color for the icon.
    pub icon_color: Color,
    /// File extension (lowercase, without dot).
    pub extension: String,
}

impl FileEntry {
    /// Build a FileEntry from a tokio DirEntry, reading full metadata.
    /// Handles broken symlinks gracefully instead of returning an error.
    pub async fn from_dir_entry(entry: tokio::fs::DirEntry) -> anyhow::Result<Self> {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Read symlink metadata first (never follows symlinks — always succeeds for existing entries)
        let symlink_meta = tokio::fs::symlink_metadata(&path).await?;
        let is_symlink = symlink_meta.is_symlink();

        // Try to read the resolved metadata (follows symlinks)
        // If this fails for a symlink, it's a broken/orphaned symlink
        let (metadata, is_broken_symlink) = if is_symlink {
            match tokio::fs::metadata(&path).await {
                Ok(m) => (m, false),
                Err(_) => (symlink_meta.clone(), true), // Broken symlink — use symlink metadata
            }
        } else {
            (symlink_meta.clone(), false)
        };

        let is_dir = if is_broken_symlink { false } else { metadata.is_dir() };
        let size = if is_dir { 0 } else { metadata.len() };
        let modified = metadata.modified().ok();
        let is_hidden = name.starts_with('.');

        // Unix permissions
        use std::os::unix::fs::MetadataExt;
        let permissions = symlink_meta.mode();
        let uid = symlink_meta.uid();
        let gid = symlink_meta.gid();

        // Resolve owner/group names
        let owner = resolve_username(uid);
        let group = resolve_groupname(gid);

        // Extension
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        // Icon lookup — broken symlinks get a distinct icon
        let (icon, icon_color) = if is_broken_symlink {
            ("\u{f127}", Palette::RED) //  — broken link icon
        } else if is_symlink {
            icons::symlink_icon()
        } else if is_dir {
            icons::dir_icon(&name)
        } else {
            icons::file_icon(&name, &extension)
        };

        Ok(Self {
            name,
            path,
            is_dir,
            is_symlink,
            is_broken_symlink,
            is_hidden,
            size,
            modified,
            permissions,
            owner,
            group,
            icon,
            icon_color,
            extension,
        })
    }

    /// Format permissions as a rwxrwxrwx string.
    pub fn permissions_string(&self) -> String {
        let mode = self.permissions;

        let file_type = if self.is_symlink {
            'l'
        } else if self.is_dir {
            'd'
        } else {
            '-'
        };

        let mut s = String::with_capacity(10);
        s.push(file_type);
        s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
        s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
        s.push(if mode & 0o100 != 0 { 'x' } else { '-' });
        s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
        s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
        s.push(if mode & 0o010 != 0 { 'x' } else { '-' });
        s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
        s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
        s.push(if mode & 0o001 != 0 { 'x' } else { '-' });
        s
    }

    /// Human-readable file size.
    pub fn size_string(&self) -> String {
        if self.is_dir {
            String::from("—")
        } else {
            humansize::format_size(self.size, humansize::BINARY)
        }
    }

    /// Formatted modification time.
    pub fn modified_string(&self) -> String {
        match self.modified {
            Some(time) => {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                datetime.format("%b %d %H:%M").to_string()
            }
            None => String::from("—"),
        }
    }
}

/// Resolve a UID to a username, falling back to the numeric ID.
fn resolve_username(uid: u32) -> String {
    nix::unistd::User::from_uid(nix::unistd::Uid::from_raw(uid))
        .ok()
        .flatten()
        .map(|u| u.name)
        .unwrap_or_else(|| uid.to_string())
}

/// Resolve a GID to a group name, falling back to the numeric ID.
fn resolve_groupname(gid: u32) -> String {
    nix::unistd::Group::from_gid(nix::unistd::Gid::from_raw(gid))
        .ok()
        .flatten()
        .map(|g| g.name)
        .unwrap_or_else(|| gid.to_string())
}
