use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::theme::palette::Palette;
use crate::ui::layout::centered_rect;

/// Section of keybindings for the help menu.
struct HelpSection {
    title: &'static str,
    bindings: &'static [(&'static str, &'static str)],
}

const HELP_SECTIONS: &[HelpSection] = &[
    HelpSection {
        title: "Navigation",
        bindings: &[
            ("j / ↓",       "Move cursor down"),
            ("k / ↑",       "Move cursor up"),
            ("h / ←",       "Go to parent directory"),
            ("l / → / ↵",   "Enter dir / Open file"),
            ("g",           "Jump to first item"),
            ("G",           "Jump to last item"),
            ("Ctrl+d",      "Page down"),
            ("Ctrl+u",      "Page up"),
        ],
    },
    HelpSection {
        title: "Actions",
        bindings: &[
            ("a",           "Create new file"),
            ("A",           "Create new directory"),
            ("r",           "Rename current item"),
            ("d / Del",     "Delete item(s)"),
            ("Space",       "Toggle selection"),
            ("s",           "Cycle sort mode"),
            (".",           "Toggle hidden files"),
        ],
    },
    HelpSection {
        title: "Modes",
        bindings: &[
            ("v",           "Enter Visual mode"),
            (":",           "Enter Command mode"),
            ("Ctrl+k",      "Show this help"),
            ("q",           "Quit"),
            ("Esc",         "Exit mode / Close"),
        ],
    },
    HelpSection {
        title: "Visual Mode",
        bindings: &[
            ("j / k",       "Move + extend selection"),
            ("Space",       "Toggle current item"),
            ("a",           "Select all"),
            ("d",           "Delete selected"),
            ("Esc / v",     "Exit Visual mode"),
        ],
    },
    HelpSection {
        title: "Commands (:)",
        bindings: &[
            (":q / :quit",  "Quit"),
            (":mkdir NAME", "Create directory"),
            (":touch NAME", "Create file"),
            (":sort",       "Cycle sort mode"),
            (":hidden",     "Toggle hidden files"),
        ],
    },
];

/// Render the floating keybinding help menu.
pub fn render(frame: &mut Frame, area: Rect) {
    let popup = centered_rect(60, 80, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup);

    // Shadow effect
    let shadow_area = Rect::new(
        (popup.x + 1).min(area.x + area.width - 1),
        (popup.y + 1).min(area.y + area.height - 1),
        popup.width.min(area.width.saturating_sub(popup.x - area.x + 1)),
        popup.height.min(area.height.saturating_sub(popup.y - area.y + 1)),
    );
    let shadow = Block::default().style(Style::default().bg(Palette::BG_DEEP));
    frame.render_widget(shadow, shadow_area);

    // Main popup block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Palette::LAVENDER))
        .style(Style::default().bg(Palette::BG_OVERLAY))
        .title_top(Line::from(vec![
            Span::styled(
                " \u{f059} Keybindings ",  //  help icon
                Style::default()
                    .fg(Palette::LAVENDER)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .title_bottom(Line::from(vec![
            Span::styled(
                " Esc / q to close ",
                Style::default().fg(Palette::DIM),
            ),
        ]));

    // Build help text lines
    let mut lines: Vec<Line> = Vec::new();

    for (section_idx, section) in HELP_SECTIONS.iter().enumerate() {
        if section_idx > 0 {
            lines.push(Line::from("")); // Spacer between sections
        }

        // Section header
        lines.push(Line::from(vec![
            Span::styled(
                format!("  ── {} ", section.title),
                Style::default()
                    .fg(Palette::BLUE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "─".repeat(30),
                Style::default().fg(Palette::BORDER_DIM),
            ),
        ]));

        // Keybindings
        for (key, desc) in section.bindings {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(
                    format!("{:<16}", key),
                    Style::default()
                        .fg(Palette::YELLOW)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    *desc,
                    Style::default().fg(Palette::TEXT),
                ),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, popup);
}
