use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Clear};
use ratatui::Frame;

use crate::theme::palette::Palette;

/// Computed layout regions for the main UI.
pub struct LayoutRegions {
    pub breadcrumb: Rect,
    pub parent: Rect,
    pub current: Rect,
    pub preview: Rect,
    pub status: Rect,
}

/// Calculate the main layout regions from the terminal area.
pub fn calculate_layout(area: Rect) -> LayoutRegions {
    // Vertical: breadcrumb (2) | 1px gap | columns (fill) | 1px gap | status (2)
    let vertical = Layout::vertical([
        Constraint::Length(2),   // Breadcrumb bar
        Constraint::Length(1),   // Top margin
        Constraint::Min(6),     // Miller columns
        Constraint::Length(1),   // Bottom margin
        Constraint::Length(2),   // Status bar
    ])
    .split(area);

    let breadcrumb = vertical[0];
    let columns_area = vertical[2];
    let status = vertical[4];

    // Horizontal: parent (20%) | gap (1) | current (35%) | gap (1) | preview (fill)
    let horizontal = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Length(1),    // Column gap
        Constraint::Percentage(30),
        Constraint::Length(1),    // Column gap
        Constraint::Min(20),     // Preview fills the rest
    ])
    .split(columns_area);

    LayoutRegions {
        breadcrumb,
        parent: horizontal[0],
        current: horizontal[2],
        preview: horizontal[4],
        status,
    }
}

/// Fill the entire terminal area with the base background color.
pub fn render_background(frame: &mut Frame, area: Rect) {
    let bg = Block::default().style(Style::default().bg(Palette::BG_BASE));
    frame.render_widget(Clear, area);
    frame.render_widget(bg, area);
}

/// Create a centered floating rectangle within a parent area.
/// Used for dialogs and modals.
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

/// Create a fixed-size centered rectangle.
pub fn centered_rect_fixed(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(
        x,
        y,
        width.min(area.width),
        height.min(area.height),
    )
}
