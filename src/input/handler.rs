use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;
use crate::state::mode::Mode;

/// Maps a key event to an action based on the current mode.
/// Returns `None` if the key has no binding in the current mode.
pub fn handle_key_event(mode: &Mode, key: KeyEvent) -> Option<Action> {
    match mode {
        Mode::Normal  => handle_normal_mode(key),
        Mode::Visual  => handle_visual_mode(key),
        Mode::Command => handle_command_mode(key),
        Mode::Input   => handle_input_mode(key),
    }
}

/// Normal mode — navigation and single-key commands.
fn handle_normal_mode(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        // ── Quit ────────────────────────────────
        (KeyCode::Char('q'), KeyModifiers::NONE)          => Some(Action::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL)       => Some(Action::Quit),

        // ── Help ────────────────────────────────
        (KeyCode::Char('k'), KeyModifiers::CONTROL)       => Some(Action::ShowHelp),
        (KeyCode::Char('?'), KeyModifiers::NONE | KeyModifiers::SHIFT) => Some(Action::ShowHelp),

        // ── Navigation ──────────────────────────
        (KeyCode::Char('j') | KeyCode::Down, _)          => Some(Action::MoveDown),
        (KeyCode::Char('k') | KeyCode::Up, _)            => Some(Action::MoveUp),
        (KeyCode::Char('h') | KeyCode::Left, _)          => Some(Action::GoBack),
        (KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter, _) => Some(Action::Enter),
        (KeyCode::Char('g'), KeyModifiers::NONE)          => Some(Action::MoveTop),
        (KeyCode::Char('G'), KeyModifiers::NONE | KeyModifiers::SHIFT) => Some(Action::MoveBottom),
        (KeyCode::Char('d'), KeyModifiers::CONTROL)       => Some(Action::PageDown),
        (KeyCode::Char('u'), KeyModifiers::CONTROL)       => Some(Action::PageUp),
        (KeyCode::PageDown, _)                            => Some(Action::PageDown),
        (KeyCode::PageUp, _)                              => Some(Action::PageUp),
        (KeyCode::Home, _)                                => Some(Action::MoveTop),
        (KeyCode::End, _)                                 => Some(Action::MoveBottom),

        // ── Mode transitions ────────────────────
        (KeyCode::Char('v'), KeyModifiers::NONE)          => Some(Action::EnterVisual),
        (KeyCode::Char(':'), KeyModifiers::NONE | KeyModifiers::SHIFT) => Some(Action::EnterCommand),

        // ── CRUD ────────────────────────────────
        (KeyCode::Char('a'), KeyModifiers::NONE)          => Some(Action::PromptCreateFile),
        (KeyCode::Char('A'), KeyModifiers::NONE | KeyModifiers::SHIFT) => Some(Action::PromptCreateDir),
        (KeyCode::Char('r'), KeyModifiers::NONE)          => Some(Action::PromptRename),
        (KeyCode::Char('d'), KeyModifiers::NONE)          => Some(Action::PromptDelete),
        (KeyCode::Delete, _)                              => Some(Action::PromptDelete),

        // ── Toggles ─────────────────────────────
        (KeyCode::Char('s'), KeyModifiers::NONE)          => Some(Action::CycleSortMode),
        (KeyCode::Char('.'), KeyModifiers::NONE)          => Some(Action::ToggleHidden),

        // ── Selection ───────────────────────────
        (KeyCode::Char(' '), KeyModifiers::NONE)          => Some(Action::ToggleSelect),

        _ => None,
    }
}

/// Visual mode — multi-selection with movement.
fn handle_visual_mode(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        // ── Exit ────────────────────────────────
        (KeyCode::Esc, _)                               => Some(Action::ExitVisual),
        (KeyCode::Char('v'), KeyModifiers::NONE)        => Some(Action::ExitVisual),

        // ── Navigation (keeps selecting) ────────
        (KeyCode::Char('j') | KeyCode::Down, _)        => Some(Action::MoveDown),
        (KeyCode::Char('k') | KeyCode::Up, _)          => Some(Action::MoveUp),
        (KeyCode::Char('g'), KeyModifiers::NONE)        => Some(Action::MoveTop),
        (KeyCode::Char('G'), KeyModifiers::NONE | KeyModifiers::SHIFT) => Some(Action::MoveBottom),

        // ── Toggle select ───────────────────────
        (KeyCode::Char(' '), KeyModifiers::NONE)        => Some(Action::ToggleSelect),
        (KeyCode::Char('a'), KeyModifiers::NONE)        => Some(Action::SelectAll),

        // ── Bulk operations ─────────────────────
        (KeyCode::Char('d'), KeyModifiers::NONE)        => Some(Action::PromptDelete),
        (KeyCode::Delete, _)                            => Some(Action::PromptDelete),

        _ => None,
    }
}

/// Command mode — `:` prompt for commands.
fn handle_command_mode(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc       => Some(Action::ExitCommand),
        KeyCode::Enter     => None, // Handled by app — submits the command buffer
        KeyCode::Backspace => None, // Handled by app — deletes from command buffer
        KeyCode::Char(_)   => None, // Handled by app — appends to command buffer
        _ => None,
    }
}

/// Input mode — for rename / create file/dir prompts.
fn handle_input_mode(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Esc   => Some(Action::CancelDialog),
        KeyCode::Enter => None, // Handled by app — submits input
        _              => None, // Handled by app — text editing
    }
}
