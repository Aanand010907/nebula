use ratatui::style::{Color, Modifier, Style};

/// Curated dark color palette inspired by Catppuccin Mocha / Tokyo Night.
/// Every color is a precise RGB value for premium rendering on true-color terminals.
pub struct Palette;

impl Palette {
    // ── Base Backgrounds ────────────────────────────────────────
    pub const BG_DEEP:     Color = Color::Rgb(17,  17,  27);   // #11111b — deepest / shadow
    pub const BG_BASE:     Color = Color::Rgb(30,  30,  46);   // #1e1e2e — main background
    pub const BG_SURFACE:  Color = Color::Rgb(36,  36,  54);   // #242436 — panel surface
    pub const BG_OVERLAY:  Color = Color::Rgb(49,  50,  68);   // #313244 — hover / overlay
    pub const BG_HIGHLIGHT:Color = Color::Rgb(40,  40,  60);   // #28283c — cursor line

    // ── Borders ─────────────────────────────────────────────────
    pub const BORDER:       Color = Color::Rgb(69,  71,  90);  // #45475a
    pub const BORDER_ACTIVE:Color = Color::Rgb(137, 180, 250); // #89b4fa — focused panel
    pub const BORDER_DIM:   Color = Color::Rgb(49,  50,  68);  // #313244 — inactive

    // ── Text ────────────────────────────────────────────────────
    pub const TEXT:         Color = Color::Rgb(205, 214, 244); // #cdd6f4
    pub const SUBTEXT:     Color = Color::Rgb(166, 173, 200); // #a6adc8
    pub const DIM:         Color = Color::Rgb(108, 112, 134); // #6c7086
    pub const OVERLAY_TEXT:Color = Color::Rgb(147, 153, 178); // #9399b2

    // ── Accent Colors ───────────────────────────────────────────
    pub const BLUE:     Color = Color::Rgb(137, 180, 250); // #89b4fa
    pub const LAVENDER: Color = Color::Rgb(180, 190, 254); // #b4befe
    pub const SAPPHIRE: Color = Color::Rgb(116, 199, 236); // #74c7ec
    pub const GREEN:    Color = Color::Rgb(166, 227, 161); // #a6e3a1
    pub const YELLOW:   Color = Color::Rgb(249, 226, 175); // #f9e2af
    pub const PEACH:    Color = Color::Rgb(250, 179, 135); // #fab387
    pub const RED:      Color = Color::Rgb(243, 139, 168); // #f38ba8
    pub const PINK:     Color = Color::Rgb(245, 194, 231); // #f5c2e7
    pub const MAUVE:    Color = Color::Rgb(203, 166, 247); // #cba6f7
    pub const TEAL:     Color = Color::Rgb(148, 226, 213); // #94e2d5
    pub const FLAMINGO: Color = Color::Rgb(242, 205, 205); // #f2cdcd
    pub const ROSEWATER:Color = Color::Rgb(245, 224, 220); // #f5e0dc
    pub const SKY:      Color = Color::Rgb(137, 220, 235); // #89dceb

    // ── Selection & Cursor ──────────────────────────────────────
    pub const SELECTION_BG: Color = Color::Rgb(55, 55, 80);   // visible selection
    pub const CURSOR_LINE:  Color = Color::Rgb(40, 40, 60);   // subtle line highlight
}

// ── Pre-built Styles ────────────────────────────────────────────

impl Palette {
    /// Normal text on base background.
    pub fn default_style() -> Style {
        Style::default().fg(Self::TEXT).bg(Self::BG_BASE)
    }

    /// Dimmed text for hidden files, secondary info.
    pub fn dim_style() -> Style {
        Style::default().fg(Self::DIM)
    }

    /// The currently highlighted item in the active column.
    pub fn cursor_style() -> Style {
        Style::default()
            .fg(Self::TEXT)
            .bg(Self::SELECTION_BG)
            .add_modifier(Modifier::BOLD)
    }

    /// Selected items in visual mode.
    pub fn selected_style() -> Style {
        Style::default()
            .fg(Self::YELLOW)
            .bg(Self::BG_OVERLAY)
            .add_modifier(Modifier::BOLD)
    }

    /// Directory names.
    pub fn dir_style() -> Style {
        Style::default().fg(Self::BLUE).add_modifier(Modifier::BOLD)
    }

    /// Symlink names.
    pub fn symlink_style() -> Style {
        Style::default().fg(Self::MAUVE).add_modifier(Modifier::ITALIC)
    }

    /// Executable files.
    pub fn exec_style() -> Style {
        Style::default().fg(Self::GREEN).add_modifier(Modifier::BOLD)
    }

    /// Active border style.
    pub fn active_border() -> Style {
        Style::default().fg(Self::BORDER_ACTIVE)
    }

    /// Inactive border style.
    pub fn inactive_border() -> Style {
        Style::default().fg(Self::BORDER_DIM)
    }

    /// Mode indicator styles.
    pub fn mode_style_normal() -> Style {
        Style::default()
            .fg(Self::BG_DEEP)
            .bg(Self::GREEN)
            .add_modifier(Modifier::BOLD)
    }

    pub fn mode_style_visual() -> Style {
        Style::default()
            .fg(Self::BG_DEEP)
            .bg(Self::YELLOW)
            .add_modifier(Modifier::BOLD)
    }

    pub fn mode_style_command() -> Style {
        Style::default()
            .fg(Self::BG_DEEP)
            .bg(Self::BLUE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn mode_style_input() -> Style {
        Style::default()
            .fg(Self::BG_DEEP)
            .bg(Self::PEACH)
            .add_modifier(Modifier::BOLD)
    }

    /// Error message style.
    pub fn error_style() -> Style {
        Style::default().fg(Self::RED).add_modifier(Modifier::BOLD)
    }

    /// Success message style.
    pub fn success_style() -> Style {
        Style::default().fg(Self::GREEN)
    }

    /// Breadcrumb separator style.
    pub fn breadcrumb_sep() -> Style {
        Style::default().fg(Self::DIM)
    }

    /// Breadcrumb path segment style.
    pub fn breadcrumb_segment() -> Style {
        Style::default().fg(Self::SAPPHIRE)
    }

    /// Breadcrumb last (current) segment style.
    pub fn breadcrumb_current() -> Style {
        Style::default()
            .fg(Self::LAVENDER)
            .add_modifier(Modifier::BOLD)
    }
}
