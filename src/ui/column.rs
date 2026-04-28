use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

use crate::app::App;
use crate::state::mode::Mode;
use crate::theme::palette::Palette;
use crate::vfs::entry::FileEntry;

/// Render the parent directory column (left pane).
pub fn render_parent(frame: &mut Frame, app: &mut App, area: Rect) {
    let items = build_list_items(&app.tab().parent_entries, &app.selection_paths(), false);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Palette::inactive_border())
        .style(Style::default().bg(Palette::BG_SURFACE))
        .title_top(Line::from(vec![
            Span::styled(" Parent ", Style::default().fg(Palette::DIM)),
        ]));

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Palette::BG_OVERLAY)
                .fg(Palette::SUBTEXT),
        );

    frame.render_stateful_widget(list, area, &mut app.tab_mut().parent_state);
}

/// Render the current directory column (center pane — the active one).
pub fn render_current(frame: &mut Frame, app: &mut App, area: Rect) {
    let is_visual = app.mode == Mode::Visual;
    let selected_paths = app.selection_paths();
    let entry_count = app.tab().entry_count();
    let cursor = app.tab().cursor_index();
    let items = build_list_items(&app.tab().current_entries, &selected_paths, is_visual);

    // Title shows entry count
    let title = format!(" {} items ", entry_count);
    let position = if entry_count > 0 {
        format!(" {}/{} ", cursor + 1, entry_count)
    } else {
        String::from(" empty ")
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Palette::active_border())
        .style(Style::default().bg(Palette::BG_SURFACE))
        .title_top(Line::from(vec![
            Span::styled(title, Style::default().fg(Palette::BLUE)),
        ]))
        .title_bottom(Line::from(vec![
            Span::styled(position, Style::default().fg(Palette::DIM)),
        ]));

    let list = List::new(items)
        .block(block)
        .highlight_style(Palette::cursor_style())
        .highlight_symbol("▌");

    frame.render_stateful_widget(list, area, &mut app.tab_mut().current_state);
}

/// Build list items from file entries with icons, names, and selection indicators.
fn build_list_items(
    entries: &[FileEntry],
    selected_paths: &[std::path::PathBuf],
    show_selection_marks: bool,
) -> Vec<ListItem<'static>> {
    entries
        .iter()
        .map(|entry| {
            let is_selected = selected_paths.contains(&entry.path);

            let mut spans = Vec::new();

            // Selection marker
            if show_selection_marks {
                if is_selected {
                    spans.push(Span::styled(
                        " ● ",
                        Style::default().fg(Palette::YELLOW).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw("   "));
                }
            } else {
                spans.push(Span::raw(" "));
            }

            // Icon with its specific color
            spans.push(Span::styled(
                entry.icon.to_string(),
                Style::default().fg(entry.icon_color),
            ));
            spans.push(Span::raw(" "));

            // Filename with type-appropriate styling
            let name_style = if is_selected {
                Palette::selected_style()
            } else if entry.is_broken_symlink {
                // Broken symlinks get red + italic + dim
                Style::default()
                    .fg(Palette::RED)
                    .add_modifier(Modifier::ITALIC | Modifier::DIM)
            } else if entry.is_symlink {
                Palette::symlink_style()
            } else if entry.is_dir {
                Palette::dir_style()
            } else if entry.permissions & 0o111 != 0 {
                // Executable
                Palette::exec_style()
            } else if entry.is_hidden {
                Palette::dim_style()
            } else {
                Style::default().fg(Palette::TEXT)
            };

            let display_name = if entry.is_broken_symlink {
                format!("{} [broken]", entry.name)
            } else if entry.is_dir {
                format!("{}/", entry.name)
            } else {
                entry.name.clone()
            };

            spans.push(Span::styled(display_name, name_style));

            // Symlink target indicator
            if entry.is_broken_symlink {
                spans.push(Span::styled(" ⇥✗", Style::default().fg(Palette::RED)));
            } else if entry.is_symlink {
                spans.push(Span::styled(" →", Style::default().fg(Palette::DIM)));
            }

            ListItem::new(Line::from(spans))
        })
        .collect()
}
