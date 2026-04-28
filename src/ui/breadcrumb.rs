use std::path::Path;

use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::theme::palette::Palette;

/// Render the breadcrumb trail at the top of the screen.
/// Shows: ~  ›  projects  ›  nebula  ›  src
pub fn render(frame: &mut Frame, current_dir: &Path, area: Rect) {
    let home = dirs_home();

    // Build path segments
    let path_str = current_dir.to_string_lossy();
    let display_path = if let Some(home_dir) = &home {
        if let Some(rest) = path_str.strip_prefix(home_dir.as_str()) {
            format!("~{}", rest)
        } else {
            path_str.to_string()
        }
    } else {
        path_str.to_string()
    };

    let segments: Vec<&str> = display_path.split('/').filter(|s| !s.is_empty()).collect();

    let mut spans = Vec::new();

    // Leading icon
    spans.push(Span::styled(
        "  \u{f07b} ", // folder icon
        Style::default().fg(Palette::LAVENDER),
    ));

    if display_path.starts_with('~') {
        // Home prefix
        for (i, segment) in segments.iter().enumerate() {
            let is_last = i == segments.len() - 1;

            if is_last {
                spans.push(Span::styled(
                    segment.to_string(),
                    Palette::breadcrumb_current(),
                ));
            } else {
                spans.push(Span::styled(
                    segment.to_string(),
                    Palette::breadcrumb_segment(),
                ));
            }

            if !is_last {
                spans.push(Span::styled("  ›  ", Palette::breadcrumb_sep()));
            }
        }
    } else {
        // Absolute path starting with /
        spans.push(Span::styled("/", Palette::breadcrumb_segment()));
        if !segments.is_empty() {
            spans.push(Span::styled("  ›  ", Palette::breadcrumb_sep()));
        }
        for (i, segment) in segments.iter().enumerate() {
            let is_last = i == segments.len() - 1;

            if is_last {
                spans.push(Span::styled(
                    segment.to_string(),
                    Palette::breadcrumb_current(),
                ));
            } else {
                spans.push(Span::styled(
                    segment.to_string(),
                    Palette::breadcrumb_segment(),
                ));
            }

            if !is_last {
                spans.push(Span::styled("  ›  ", Palette::breadcrumb_sep()));
            }
        }
    }

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Palette::BORDER_DIM))
        .style(Style::default().bg(Palette::BG_BASE));

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    frame.render_widget(paragraph, area);
}

/// Get the home directory path as a string.
fn dirs_home() -> Option<String> {
    std::env::var("HOME").ok()
}
