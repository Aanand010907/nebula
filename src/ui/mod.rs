pub mod layout;
pub mod column;
pub mod breadcrumb;
pub mod statusbar;
pub mod preview;
pub mod command_line;
pub mod dialog;
pub mod help;

use ratatui::Frame;

use crate::app::App;

/// Master render function — draws the entire UI each frame.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    // Calculate the main layout regions
    let regions = layout::calculate_layout(area);

    // Draw the background fill
    layout::render_background(frame, area);

    // Draw breadcrumb trail at the top
    breadcrumb::render(frame, &app.tab().current_dir, regions.breadcrumb);

    // Draw the three Miller columns
    column::render_parent(frame, app, regions.parent);
    column::render_current(frame, app, regions.current);
    preview::render(frame, app, regions.preview);

    // Draw the status bar at the bottom
    statusbar::render(frame, app, regions.status);

    // Overlay: command line (replaces status bar when active)
    if app.mode == crate::state::mode::Mode::Command {
        command_line::render(frame, &app.command_input, regions.status);
    }

    // Overlay: input prompt (floating)
    if app.mode == crate::state::mode::Mode::Input {
        if let Some(ref prompt) = app.input_prompt {
            let prompt_str = prompt.to_string();
            dialog::render_input(frame, &prompt_str, &app.input_buffer, area);
        }
    }

    // Overlay: confirmation dialog
    if app.show_confirm_dialog {
        dialog::render_confirm(frame, &app.confirm_message, area);
    }

    // Overlay: help menu (topmost layer)
    if app.show_help {
        help::render(frame, area);
    }
}
