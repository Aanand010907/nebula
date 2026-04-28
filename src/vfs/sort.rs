use std::fmt;

use crate::vfs::entry::FileEntry;

/// Available sorting strategies for directory entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    Name,
    Size,
    Modified,
    Extension,
}

impl SortMode {
    /// Sort entries in-place. Directories always come first.
    pub fn sort(&self, entries: &mut [FileEntry]) {
        entries.sort_by(|a, b| {
            // Directories first, always
            let dir_ord = b.is_dir.cmp(&a.is_dir);
            if dir_ord != std::cmp::Ordering::Equal {
                return dir_ord;
            }

            match self {
                SortMode::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortMode::Size => b.size.cmp(&a.size),
                SortMode::Modified => {
                    let a_time = a.modified.unwrap_or(std::time::UNIX_EPOCH);
                    let b_time = b.modified.unwrap_or(std::time::UNIX_EPOCH);
                    b_time.cmp(&a_time) // Most recent first
                }
                SortMode::Extension => a
                    .extension
                    .to_lowercase()
                    .cmp(&b.extension.to_lowercase())
                    .then(a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            }
        });
    }

    /// Cycle to the next sort mode.
    pub fn next(self) -> Self {
        match self {
            SortMode::Name => SortMode::Size,
            SortMode::Size => SortMode::Modified,
            SortMode::Modified => SortMode::Extension,
            SortMode::Extension => SortMode::Name,
        }
    }
}

impl fmt::Display for SortMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortMode::Name => write!(f, "Name"),
            SortMode::Size => write!(f, "Size"),
            SortMode::Modified => write!(f, "Modified"),
            SortMode::Extension => write!(f, "Extension"),
        }
    }
}
