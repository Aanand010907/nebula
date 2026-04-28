use std::collections::HashSet;
use std::path::PathBuf;

/// Tracks multi-selection state for Visual mode operations.
#[derive(Debug, Clone)]
pub struct Selection {
    /// Set of selected file paths.
    selected: HashSet<PathBuf>,
    /// The index where visual selection started (anchor point).
    anchor: Option<usize>,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            selected: HashSet::new(),
            anchor: None,
        }
    }

    /// Toggle a single path in the selection set.
    pub fn toggle(&mut self, path: &PathBuf) {
        if self.selected.contains(path) {
            self.selected.remove(path);
        } else {
            self.selected.insert(path.clone());
        }
    }

    /// Check if a path is selected.
    pub fn is_selected(&self, path: &PathBuf) -> bool {
        self.selected.contains(path)
    }

    /// Add a path to the selection.
    pub fn select(&mut self, path: PathBuf) {
        self.selected.insert(path);
    }

    /// Remove a path from the selection.
    pub fn deselect(&mut self, path: &PathBuf) {
        self.selected.remove(path);
    }

    /// Clear all selections.
    pub fn clear(&mut self) {
        self.selected.clear();
        self.anchor = None;
    }

    /// Get all selected paths.
    pub fn paths(&self) -> Vec<PathBuf> {
        self.selected.iter().cloned().collect()
    }

    /// Number of selected items.
    pub fn count(&self) -> usize {
        self.selected.len()
    }

    /// Set the anchor point for range selection.
    pub fn set_anchor(&mut self, idx: usize) {
        self.anchor = Some(idx);
    }

    /// Get the anchor index.
    pub fn anchor(&self) -> Option<usize> {
        self.anchor
    }

    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}
