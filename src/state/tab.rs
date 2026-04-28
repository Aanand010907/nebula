use std::collections::HashMap;
use std::path::PathBuf;

use ratatui::widgets::ListState;

use crate::vfs::entry::FileEntry;
use crate::vfs::sort::SortMode;
use crate::ui::preview::PreviewContent;

/// Represents a single navigable tab with its own directory state.
/// Each tab maintains independent cursor positions and history.
pub struct Tab {
    /// Current working directory.
    pub current_dir: PathBuf,

    /// Entries for the parent directory (left column).
    pub parent_entries: Vec<FileEntry>,

    /// Entries for the current directory (center column).
    pub current_entries: Vec<FileEntry>,

    /// Content for the preview pane (right column).
    pub preview_content: PreviewContent,

    /// List state for parent column scrolling.
    pub parent_state: ListState,

    /// List state for current column scrolling (main cursor).
    pub current_state: ListState,

    /// Scroll offset for preview pane.
    pub preview_scroll: u16,

    /// Per-directory cursor position memory — remembers where you were.
    pub history: HashMap<PathBuf, usize>,

    /// Current sort mode.
    pub sort_mode: SortMode,
}

impl Tab {
    pub fn new(path: PathBuf) -> Self {
        let mut current_state = ListState::default();
        current_state.select(Some(0));

        Self {
            current_dir: path,
            parent_entries: Vec::new(),
            current_entries: Vec::new(),
            preview_content: PreviewContent::Loading,
            parent_state: ListState::default(),
            current_state,
            preview_scroll: 0,
            history: HashMap::new(),
            sort_mode: SortMode::Name,
        }
    }

    /// Get the currently highlighted entry.
    pub fn current_entry(&self) -> Option<&FileEntry> {
        let idx = self.current_state.selected()?;
        self.current_entries.get(idx)
    }

    /// Get the index of the currently selected item.
    pub fn cursor_index(&self) -> usize {
        self.current_state.selected().unwrap_or(0)
    }

    /// Total entries in the current directory (after filtering).
    pub fn entry_count(&self) -> usize {
        self.current_entries.len()
    }

    /// Move cursor up by one.
    pub fn move_up(&mut self) {
        if self.current_entries.is_empty() {
            return;
        }
        let current = self.cursor_index();
        let new = if current == 0 {
            self.current_entries.len().saturating_sub(1) // Wrap to bottom
        } else {
            current - 1
        };
        self.current_state.select(Some(new));
        self.preview_scroll = 0;
    }

    /// Move cursor down by one.
    pub fn move_down(&mut self) {
        if self.current_entries.is_empty() {
            return;
        }
        let current = self.cursor_index();
        let new = if current >= self.current_entries.len().saturating_sub(1) {
            0 // Wrap to top
        } else {
            current + 1
        };
        self.current_state.select(Some(new));
        self.preview_scroll = 0;
    }

    /// Jump to the first entry.
    pub fn move_top(&mut self) {
        if !self.current_entries.is_empty() {
            self.current_state.select(Some(0));
            self.preview_scroll = 0;
        }
    }

    /// Jump to the last entry.
    pub fn move_bottom(&mut self) {
        if !self.current_entries.is_empty() {
            self.current_state.select(Some(self.current_entries.len() - 1));
            self.preview_scroll = 0;
        }
    }

    /// Page up (move cursor up by ~half the visible area).
    pub fn page_up(&mut self, visible_rows: usize) {
        if self.current_entries.is_empty() {
            return;
        }
        let current = self.cursor_index();
        let jump = visible_rows / 2;
        let new = current.saturating_sub(jump);
        self.current_state.select(Some(new));
        self.preview_scroll = 0;
    }

    /// Page down (move cursor down by ~half the visible area).
    pub fn page_down(&mut self, visible_rows: usize) {
        if self.current_entries.is_empty() {
            return;
        }
        let current = self.cursor_index();
        let jump = visible_rows / 2;
        let max = self.current_entries.len().saturating_sub(1);
        let new = (current + jump).min(max);
        self.current_state.select(Some(new));
        self.preview_scroll = 0;
    }

    /// Save the current cursor position for the current directory.
    pub fn save_cursor(&mut self) {
        let idx = self.cursor_index();
        self.history.insert(self.current_dir.clone(), idx);
    }

    /// Restore a previously saved cursor position for a directory.
    pub fn restore_cursor(&mut self, path: &PathBuf) {
        if let Some(&idx) = self.history.get(path) {
            let clamped = idx.min(self.current_entries.len().saturating_sub(1));
            self.current_state.select(Some(clamped));
        } else {
            self.current_state.select(Some(0));
        }
    }

    /// Update entries for the current directory, applying filter + sort.
    /// `show_hidden` is passed from the global App state for consistency.
    pub fn set_current_entries(&mut self, mut entries: Vec<FileEntry>, show_hidden: bool) {
        if !show_hidden {
            entries.retain(|e| !e.is_hidden);
        }
        self.sort_mode.sort(&mut entries);
        self.current_entries = entries;

        // Clamp cursor
        if !self.current_entries.is_empty() {
            let idx = self.cursor_index().min(self.current_entries.len() - 1);
            self.current_state.select(Some(idx));
        } else {
            self.current_state.select(None);
        }
    }

    /// Update entries for the parent directory.
    /// `show_hidden` is passed from the global App state for consistency.
    pub fn set_parent_entries(&mut self, mut entries: Vec<FileEntry>, show_hidden: bool) {
        if !show_hidden {
            entries.retain(|e| !e.is_hidden);
        }
        self.sort_mode.sort(&mut entries);
        self.parent_entries = entries;

        // Highlight the current directory in the parent listing
        if let Some(dir_name) = self.current_dir.file_name() {
            if let Some(pos) = self.parent_entries.iter().position(|e| e.name == dir_name.to_string_lossy()) {
                self.parent_state.select(Some(pos));
            }
        }
    }
}
