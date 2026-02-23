use ratatui::style::Color;
use ratatui::symbols::border;

/// Logical grid dimensions passed through the game as a named type.
///
/// Replaces the anonymous `(u16, u16)` tuple that was used for bounds,
/// making width vs. height unambiguous at every call site.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GridSize {
    pub width: u16,
    pub height: u16,
}

impl GridSize {
    /// Returns the total number of cells in the grid.
    #[must_use]
    pub fn total_cells(self) -> usize {
        usize::from(self.width) * usize::from(self.height)
    }
}

/// A color theme applied to all visual elements.
///
/// In half-block rendering mode every entity is a solid colored block.
/// The `snake_head`, `snake_body`, `snake_tail`, and `food` fields each
/// specify the solid block color for that entity.
///
/// UI fields (`ui_bg`, `ui_text`, `ui_accent`, `ui_muted`) style the HUD
/// and menu panels. JSON theme keys match these field names 1:1.
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    /// Solid block color for the snake head segment.
    pub snake_head: Color,
    /// Solid block color for body segments.
    pub snake_body: Color,
    /// Solid block color for the tail segment.
    pub snake_tail: Color,
    /// Solid block color for food items.
    pub food: Color,
    /// Background color painted across the entire terminal before all other layers.
    /// Set to `Color::Reset` to use the terminal's own default background.
    pub terminal_bg: Color,
    /// Background color for empty play-field cells.
    pub field_bg: Color,
    /// Background color for menu panels and popups.
    pub ui_bg: Color,
    /// Primary text color used in the HUD, score display, and menu body.
    pub ui_text: Color,
    /// Accent color for menu titles and selected-option highlights.
    pub ui_accent: Color,
    /// Subdued color for footer hints and secondary labels.
    pub ui_muted: Color,
}

/// Emergency fallback theme used when no external/bundled themes load.
#[must_use]
pub fn fallback_theme() -> Theme {
    Theme {
        name: "fallback".to_owned(),
        snake_head: Color::White,
        snake_body: Color::Blue,
        snake_tail: Color::DarkGray,
        food: Color::Red,
        terminal_bg: Color::Reset,
        field_bg: Color::Black,
        ui_bg: Color::DarkGray,
        ui_text: Color::White,
        ui_accent: Color::Green,
        ui_muted: Color::DarkGray,
    }
}

/// Invisible border set used to reserve one-cell wall padding.
///
/// The game still keeps a visual/logic buffer around the play area, but the
/// border glyphs render as spaces so the wall blends into the terminal.
pub const BORDER_HALF_BLOCK: border::Set = border::Set {
    top_left: " ",
    top_right: " ",
    bottom_left: " ",
    bottom_right: " ",
    vertical_left: " ",
    vertical_right: " ",
    horizontal_top: " ",
    horizontal_bottom: " ",
};

/// Upper half-block glyph for compositing.
pub const GLYPH_HALF_UPPER: &str = "▀";

/// Lower half-block glyph for compositing.
pub const GLYPH_HALF_LOWER: &str = "▄";

/// Base tick interval in milliseconds.
pub const DEFAULT_TICK_INTERVAL_MS: u64 = 200;

/// Minimum tick interval in milliseconds.
pub const MIN_TICK_INTERVAL_MS: u64 = 60;

/// Score needed per speed level increase.
pub const POINTS_PER_SPEED_LEVEL: u32 = 5;
