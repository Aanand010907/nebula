use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::theme::palette::Palette;
use crate::vfs::entry::FileEntry;

/// Preview content types for the right column.
#[derive(Debug, Clone)]
pub enum PreviewContent {
    /// Directory listing preview.
    Directory(Vec<FileEntry>),
    /// Empty / nothing selected or file selected (blank pane).
    Empty,
    /// Empty directory (has entries but they are all filtered or truly empty).
    EmptyDir,
    /// Permission denied — cannot read the directory or file.
    PermissionDenied,
    /// Content is being loaded.
    Loading,
    /// An error occurred.
    Error(String),
}

/// Render the preview pane (right column).
pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let tab = app.tab();

    let title = if let Some(entry) = tab.current_entry() {
        format!(" {} ", entry.name)
    } else {
        " Preview ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Palette::inactive_border())
        .style(Style::default().bg(Palette::BG_SURFACE))
        .title_top(Line::from(vec![
            Span::styled(title, Style::default().fg(Palette::DIM)),
        ]));

    match &tab.preview_content {
        PreviewContent::Directory(entries) => {
            let items: Vec<ListItem> = entries
                .iter()
                .take(area.height as usize) // Only render what's visible
                .map(|entry| {
                    let name_style = if entry.is_broken_symlink {
                        Style::default()
                            .fg(Palette::RED)
                            .add_modifier(Modifier::ITALIC | Modifier::DIM)
                    } else if entry.is_dir {
                        Palette::dir_style()
                    } else if entry.is_hidden {
                        Palette::dim_style()
                    } else {
                        Style::default().fg(Palette::SUBTEXT)
                    };

                    let display_name = if entry.is_broken_symlink {
                        format!("{} [broken]", entry.name)
                    } else if entry.is_dir {
                        format!("{}/", entry.name)
                    } else {
                        entry.name.clone()
                    };

                    ListItem::new(Line::from(vec![
                        Span::raw(" "),
                        Span::styled(entry.icon.to_string(), Style::default().fg(entry.icon_color)),
                        Span::raw(" "),
                        Span::styled(display_name, name_style),
                    ]))
                })
                .collect();

            let list = List::new(items).block(block);
            frame.render_widget(list, area);
        }

        PreviewContent::Loading => {
            let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let frame_idx = (app.tick_count / 3) as usize % spinner_frames.len();
            let spinner = spinner_frames[frame_idx];

            let content = Line::from(vec![
                Span::styled(
                    format!("  {} Loading...", spinner),
                    Style::default()
                        .fg(Palette::BLUE)
                        .add_modifier(Modifier::BOLD),
                ),
            ]);

            let paragraph = Paragraph::new(content).block(block);
            frame.render_widget(paragraph, area);
        }

        PreviewContent::Empty => {
            // Blank pane — just the styled border and background, no content
            let paragraph = Paragraph::new("").block(block);
            frame.render_widget(paragraph, area);
        }

        PreviewContent::EmptyDir => {
            // Centered "Empty Directory" message
            let inner_height = area.height.saturating_sub(2); // Account for borders
            let pad_top = inner_height / 2;

            let mut lines: Vec<Line> = Vec::new();
            for _ in 0..pad_top.saturating_sub(1) {
                lines.push(Line::from(""));
            }
            lines.push(Line::from(vec![
                Span::styled(
                    "\u{f07b}",  //  folder icon
                    Style::default().fg(Palette::DIM),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled(
                    "Empty Directory",
                    Style::default()
                        .fg(Palette::DIM)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));

            let paragraph = Paragraph::new(lines)
                .block(block)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }

        PreviewContent::PermissionDenied => {
            let inner_height = area.height.saturating_sub(2);
            let pad_top = inner_height / 2;

            let mut lines: Vec<Line> = Vec::new();
            for _ in 0..pad_top.saturating_sub(1) {
                lines.push(Line::from(""));
            }
            lines.push(Line::from(vec![
                Span::styled(
                    "\u{f023}",  //  lock icon
                    Style::default().fg(Palette::RED),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled(
                    "Permission Denied",
                    Style::default()
                        .fg(Palette::RED)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            let paragraph = Paragraph::new(lines)
                .block(block)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        }

        PreviewContent::Error(msg) => {
            let content = Line::from(vec![
                Span::styled(
                    format!("  \u{f06a} {}", msg), //  — warning icon
                    Palette::error_style(),
                ),
            ]);

            let paragraph = Paragraph::new(content).block(block);
            frame.render_widget(paragraph, area);
        }
    }
}
