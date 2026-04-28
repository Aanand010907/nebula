/// Keybinding reference for the help system.
/// This is a data-only module that documents all available keybindings.

pub struct KeybindingInfo {
    pub key: &'static str,
    pub description: &'static str,
    pub mode: &'static str,
}

/// All keybindings for display in help dialogs.
pub const KEYBINDINGS: &[KeybindingInfo] = &[
    // Normal mode — Navigation
    KeybindingInfo { key: "j / ↓",   description: "Move cursor down",        mode: "Normal" },
    KeybindingInfo { key: "k / ↑",   description: "Move cursor up",          mode: "Normal" },
    KeybindingInfo { key: "h / ←",   description: "Go to parent directory",  mode: "Normal" },
    KeybindingInfo { key: "l / → / ↵", description: "Enter directory / open",  mode: "Normal" },
    KeybindingInfo { key: "g",       description: "Jump to first item",      mode: "Normal" },
    KeybindingInfo { key: "G",       description: "Jump to last item",       mode: "Normal" },
    KeybindingInfo { key: "Ctrl+d",  description: "Page down",               mode: "Normal" },
    KeybindingInfo { key: "Ctrl+u",  description: "Page up",                 mode: "Normal" },

    // Normal mode — Actions
    KeybindingInfo { key: "a",       description: "Create new file",         mode: "Normal" },
    KeybindingInfo { key: "A",       description: "Create new directory",    mode: "Normal" },
    KeybindingInfo { key: "r",       description: "Rename current item",     mode: "Normal" },
    KeybindingInfo { key: "d",       description: "Delete current item",     mode: "Normal" },
    KeybindingInfo { key: "Space",   description: "Toggle selection",        mode: "Normal" },
    KeybindingInfo { key: "s",       description: "Cycle sort mode",         mode: "Normal" },
    KeybindingInfo { key: ".",       description: "Toggle hidden files",     mode: "Normal" },

    // Normal mode — Mode transitions
    KeybindingInfo { key: "v",       description: "Enter Visual mode",       mode: "Normal" },
    KeybindingInfo { key: ":",       description: "Enter Command mode",      mode: "Normal" },
    KeybindingInfo { key: "Ctrl+k",  description: "Show keybinding help",    mode: "Normal" },
    KeybindingInfo { key: "?",       description: "Show keybinding help",    mode: "Normal" },
    KeybindingInfo { key: "q",       description: "Quit",                    mode: "Normal" },

    // Visual mode
    KeybindingInfo { key: "j/k",     description: "Move and extend selection", mode: "Visual" },
    KeybindingInfo { key: "Space",   description: "Toggle current selection", mode: "Visual" },
    KeybindingInfo { key: "a",       description: "Select all",              mode: "Visual" },
    KeybindingInfo { key: "d",       description: "Delete selected",         mode: "Visual" },
    KeybindingInfo { key: "Esc / v", description: "Exit Visual mode",        mode: "Visual" },

    // Command mode
    KeybindingInfo { key: "Enter",   description: "Execute command",         mode: "Command" },
    KeybindingInfo { key: "Esc",     description: "Cancel command",          mode: "Command" },
];
