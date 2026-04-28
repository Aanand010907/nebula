use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::state::mode::Mode;
use crate::theme::palette::Palette;

/// Render the status bar at the bottom of the screen.
/// Layout: [MODE] | file info | permissions | owner:group | size | modified | [position] | [sort]
pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let status_layout = Layout::horizontal([
        Constraint::Length(10),   // Mode indicator
        Constraint::Min(20),     // File info (center)
        Constraint::Length(20),  // Position + sort (right)
    ])
    .split(area);

    // ── Mode indicator ──────────────────────────────────────────
    let (mode_text, mode_style) = match app.mode {
        Mode::Normal  => (" NORMAL ", Palette::mode_style_normal()),
        Mode::Visual  => (" VISUAL ", Palette::mode_style_visual()),
        Mode::Command => (" CMD    ", Palette::mode_style_command()),
        Mode::Input   => (" INPUT  ", Palette::mode_style_input()),
    };

    let mode_widget = Paragraph::new(Line::from(Span::styled(mode_text, mode_style)))
        .style(Style::default().bg(Palette::BG_DEEP));
    frame.render_widget(mode_widget, status_layout[0]);

    // ── Center: file metadata ───────────────────────────────────
    let center_content = if let Some(ref msg) = app.status_message {
        // Show status message if available
        let style = if app.status_is_error {
            Palette::error_style()
        } else {
            Palette::success_style()
        };
        Line::from(Span::styled(format!(" {}", msg), style))
    } else if let Some(entry) = app.tab().current_entry() {
        // Show file metadata
        let mut spans = vec![Span::raw(" ")];

        // File name
        spans.push(Span::styled(
            &entry.name,
            Style::default()
                .fg(Palette::TEXT)
                .add_modifier(Modifier::BOLD),
        ));

        spans.push(Span::styled("  │  ", Style::default().fg(Palette::BORDER)));

        // Permissions
        spans.push(Span::styled(
            entry.permissions_string(),
            Style::default().fg(Palette::SUBTEXT),
        ));

        spans.push(Span::styled("  │  ", Style::default().fg(Palette::BORDER)));

        // Owner:Group
        spans.push(Span::styled(
            format!("{}:{}", entry.owner, entry.group),
            Style::default().fg(Palette::TEAL),
        ));

        spans.push(Span::styled("  │  ", Style::default().fg(Palette::BORDER)));

        // Size
        spans.push(Span::styled(
            entry.size_string(),
            Style::default().fg(Palette::PEACH),
        ));

        spans.push(Span::styled("  │  ", Style::default().fg(Palette::BORDER)));

        // Modified date
        spans.push(Span::styled(
            entry.modified_string(),
            Style::default().fg(Palette::SUBTEXT),
        ));

        // Selection count (if any)
        if app.selection.count() > 0 {
            spans.push(Span::styled("  │  ", Style::default().fg(Palette::BORDER)));
            spans.push(Span::styled(
                format!("{} selected", app.selection.count()),
                Style::default()
                    .fg(Palette::YELLOW)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        Line::from(spans)
    } else {
        Line::from(Span::styled(" (empty)", Style::default().fg(Palette::DIM)))
    };

    let center_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Palette::BG_DEEP));

    let center_widget = Paragraph::new(center_content).block(center_block);
    frame.render_widget(center_widget, status_layout[1]);

    // ── Right: position + sort ──────────────────────────────────
    let tab = app.tab();
    let pos_text = if tab.entry_count() > 0 {
        format!("{}/{}", tab.cursor_index() + 1, tab.entry_count())
    } else {
        "—".to_string()
    };

    let hidden_indicator = if app.show_hidden {
        Span::styled(" \u{f06e} ", Style::default().fg(Palette::GREEN))  //  eye icon — showing
    } else {
        Span::styled(" \u{f070} ", Style::default().fg(Palette::DIM))    //  eye-slash — hidden
    };

    let right_content = Line::from(vec![
        hidden_indicator,
        Span::styled(
            format!(" {}", tab.sort_mode),
            Style::default().fg(Palette::SAPPHIRE),
        ),
        Span::styled("  ", Style::default().fg(Palette::BORDER)),
        Span::styled(
            format!("{} ", pos_text),
            Style::default().fg(Palette::DIM),
        ),
    ]);

    let right_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(Palette::BG_DEEP));

    let right_widget = Paragraph::new(right_content)
        .block(right_block)
        .alignment(ratatui::layout::Alignment::Right);
    frame.render_widget(right_widget, status_layout[2]);
}
