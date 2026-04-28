use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::theme::palette::Palette;

/// Render the command-mode input bar (overlays the status bar area).
/// Shows `:` prefix with the current command input and a blinking cursor.
pub fn render(frame: &mut Frame, input: &str, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Palette::active_border())
        .style(Style::default().bg(Palette::BG_DEEP));

    let content = Line::from(vec![
        Span::styled(
            ":",
            Style::default()
                .fg(Palette::BLUE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(input, Style::default().fg(Palette::TEXT)),
        Span::styled(
            "▎",
            Style::default()
                .fg(Palette::BLUE)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]);

    let paragraph = Paragraph::new(content).block(block);
    frame.render_widget(paragraph, area);
}
