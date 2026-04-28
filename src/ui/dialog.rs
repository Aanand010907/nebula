use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::theme::palette::Palette;
use crate::ui::layout::centered_rect_fixed;

/// Render a text input dialog (used for create file, create dir, rename).
pub fn render_input(frame: &mut Frame, prompt: &str, input: &str, area: Rect) {
    let dialog_area = centered_rect_fixed(50, 5, area);

    // Clear the area behind the dialog
    frame.render_widget(Clear, dialog_area);

    // Shadow effect — draw a slightly offset darker block behind
    let shadow_area = Rect::new(
        dialog_area.x + 1,
        dialog_area.y + 1,
        dialog_area.width,
        dialog_area.height,
    );
    if shadow_area.x + shadow_area.width <= area.x + area.width
        && shadow_area.y + shadow_area.height <= area.y + area.height
    {
        let shadow = Block::default()
            .style(Style::default().bg(Palette::BG_DEEP));
        frame.render_widget(shadow, shadow_area);
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Palette::BLUE))
        .style(Style::default().bg(Palette::BG_OVERLAY))
        .title_top(Line::from(vec![
            Span::styled(
                format!(" {} ", prompt),
                Style::default()
                    .fg(Palette::LAVENDER)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

    let content = Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(input, Style::default().fg(Palette::TEXT)),
        Span::styled(
            "▎",
            Style::default()
                .fg(Palette::BLUE)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]);

    let hint = Line::from(vec![
        Span::styled(
            " Enter to confirm • Esc to cancel",
            Style::default().fg(Palette::DIM),
        ),
    ]);

    let paragraph = Paragraph::new(vec![content, hint])
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, dialog_area);
}

/// Render a confirmation dialog (used for delete operations).
pub fn render_confirm(frame: &mut Frame, message: &str, area: Rect) {
    let dialog_area = centered_rect_fixed(50, 7, area);

    // Clear the area behind the dialog
    frame.render_widget(Clear, dialog_area);

    // Shadow effect
    let shadow_area = Rect::new(
        dialog_area.x + 1,
        dialog_area.y + 1,
        dialog_area.width,
        dialog_area.height,
    );
    if shadow_area.x + shadow_area.width <= area.x + area.width
        && shadow_area.y + shadow_area.height <= area.y + area.height
    {
        let shadow = Block::default()
            .style(Style::default().bg(Palette::BG_DEEP));
        frame.render_widget(shadow, shadow_area);
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Palette::RED))
        .style(Style::default().bg(Palette::BG_OVERLAY))
        .title_top(Line::from(vec![
            Span::styled(
                " \u{f06a} Confirm Delete ",
                Style::default()
                    .fg(Palette::RED)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("  {}", message),
                Style::default().fg(Palette::TEXT),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                " y ",
                Style::default()
                    .fg(Palette::BG_DEEP)
                    .bg(Palette::RED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Delete  ", Style::default().fg(Palette::SUBTEXT)),
            Span::styled(
                " n ",
                Style::default()
                    .fg(Palette::BG_DEEP)
                    .bg(Palette::GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Cancel", Style::default().fg(Palette::SUBTEXT)),
        ]),
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, dialog_area);
}
