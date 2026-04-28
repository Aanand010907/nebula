use std::fmt;

/// Modal editing modes, inspired by Vim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Default navigation mode — h/j/k/l movement, single-key actions.
    Normal,
    /// Multi-select mode — j/k to move, Space to toggle selection.
    Visual,
    /// Command input mode — type a command after ':'.
    Command,
    /// Text input mode — for rename, create file/dir prompts.
    Input,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Normal  => write!(f, "NORMAL"),
            Mode::Visual  => write!(f, "VISUAL"),
            Mode::Command => write!(f, "COMMAND"),
            Mode::Input   => write!(f, "INPUT"),
        }
    }
}

impl Mode {
    /// Returns true if the mode accepts text input.
    pub fn is_input_mode(&self) -> bool {
        matches!(self, Mode::Command | Mode::Input)
    }
}
