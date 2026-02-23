use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;

use crate::config::{BORDER_HALF_BLOCK, GLYPH_HALF_LOWER, GLYPH_HALF_UPPER, GridSize, Theme};
use crate::game::{FoodDensity, GameState, GameStatus};
use crate::platform::Platform;
use crate::ui::hud::{HudInfo, render_hud};
use crate::ui::menu::{
    ThemeSelectView, render_game_over_menu, render_pause_menu, render_start_menu,
};

pub struct MenuUiState<'a> {
    pub start_selected_idx: usize,
    pub pause_selected_idx: usize,
    pub game_over_selected_idx: usize,
    pub food_density: FoodDensity,
    pub start_theme_select: Option<ThemeSelectView<'a>>,
    pub pause_theme_select: Option<ThemeSelectView<'a>>,
}

/// What occupies a single logical game cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellKind {
    Empty,
    Snake,
    Food,
}

/// Renders the full game frame from immutable state.
pub fn render(
    frame: &mut Frame<'_>,
    state: &GameState,
    platform: Platform,
    hud_info: HudInfo<'_>,
    menu_ui: MenuUiState<'_>,
) {
    let area = frame.area();

    let theme = hud_info.theme;
    frame.render_widget(
        Block::default().style(Style::new().bg(theme.terminal_bg)),
        area,
    );

    let play_area = render_hud(frame, area, state, platform, &hud_info);
    let block = Block::bordered()
        .border_set(BORDER_HALF_BLOCK)
        .border_style(Style::new().fg(Color::Reset).bg(Color::Reset));

    let inner = block.inner(play_area);
    frame.render_widget(block, play_area);

    render_play_area(frame, inner, state, theme);

    if state.is_start_screen() {
        render_start_menu(
            frame,
            play_area,
            hud_info.high_score,
            hud_info.theme,
            menu_ui.start_selected_idx,
            menu_ui.food_density,
            menu_ui.start_theme_select,
        );
        return;
    }

    match state.status {
        GameStatus::Paused => render_pause_menu(
            frame,
            play_area,
            hud_info.theme,
            menu_ui.pause_selected_idx,
            menu_ui.food_density,
            menu_ui.pause_theme_select,
        ),
        GameStatus::GameOver => render_game_over_menu(
            frame,
            play_area,
            state.score,
            hud_info.game_over_reference_high_score,
            state.death_reason,
            state.elapsed_duration(),
            hud_info.theme,
            menu_ui.game_over_selected_idx,
        ),
        GameStatus::Victory => render_game_over_menu(
            frame,
            play_area,
            state.score,
            hud_info.game_over_reference_high_score,
            state.death_reason,
            state.elapsed_duration(),
            hud_info.theme,
            menu_ui.game_over_selected_idx,
        ),
        _ => {}
    }
}

/// Builds a color grid from game state and composites half-block row-pairs.
fn render_play_area(frame: &mut Frame<'_>, inner: Rect, state: &GameState, theme: &Theme) {
    let bounds = state.bounds();
    let grid = build_cell_grid(state, bounds);

    let buffer = frame.buffer_mut();
    let game_h = usize::from(bounds.height);
    // Each terminal row composites two game rows.
    let term_rows = game_h.div_ceil(2);

    for term_row in 0..term_rows {
        let top_game_row = term_row * 2;
        let bot_game_row = term_row * 2 + 1;
        let y = inner.y.saturating_add(term_row as u16);
        if y >= inner.bottom() {
            break;
        }

        for col in 0..usize::from(bounds.width) {
            let x = inner.x.saturating_add(col as u16);
            if x >= inner.right() {
                break;
            }

            let top_kind = grid[top_game_row * usize::from(bounds.width) + col];
            let bot_kind = if bot_game_row < game_h {
                grid[bot_game_row * usize::from(bounds.width) + col]
            } else {
                CellKind::Empty
            };

            let (glyph, fg, bg) = composite_half_block(top_kind, bot_kind, theme);
            buffer.set_string(x, y, glyph, Style::new().fg(fg).bg(bg));
        }
    }
}

/// Populates a flat grid of `CellKind` values indexed by `row * width + col`.
fn build_cell_grid(state: &GameState, bounds: GridSize) -> Vec<CellKind> {
    let w = usize::from(bounds.width);
    let h = usize::from(bounds.height);
    let mut grid = vec![CellKind::Empty; w * h];

    // Food
    for food in &state.foods {
        let fp = food.position;
        if fp.is_within_bounds(bounds) {
            grid[fp.y as usize * w + fp.x as usize] = CellKind::Food;
        }
    }

    // Snake segments
    for seg in state.snake.segments() {
        if !seg.is_within_bounds(bounds) {
            continue;
        }
        grid[seg.y as usize * w + seg.x as usize] = CellKind::Snake;
    }

    grid
}

/// Returns (glyph, fg_color, bg_color) for a terminal cell compositing two game rows.
fn composite_half_block(
    top: CellKind,
    bot: CellKind,
    theme: &Theme,
) -> (&'static str, ratatui::style::Color, ratatui::style::Color) {
    let bg = theme.field_bg;

    match (top, bot) {
        (CellKind::Empty, CellKind::Empty) => (" ", bg, bg),
        (top_kind, CellKind::Empty) => {
            // Upper half-block: fg = top color, bg = empty
            (GLYPH_HALF_UPPER, cell_color(top_kind, theme), bg)
        }
        (CellKind::Empty, bot_kind) => {
            // Lower half-block: fg = bottom color, bg = empty
            (GLYPH_HALF_LOWER, cell_color(bot_kind, theme), bg)
        }
        (top_kind, bot_kind) => {
            // Upper half-block: fg = top color, bg = bottom color
            (
                GLYPH_HALF_UPPER,
                cell_color(top_kind, theme),
                cell_color(bot_kind, theme),
            )
        }
    }
}

/// Maps a non-empty `CellKind` to its theme color.
fn cell_color(kind: CellKind, theme: &Theme) -> ratatui::style::Color {
    match kind {
        CellKind::Snake => theme.snake_body,
        CellKind::Food => theme.food,
        CellKind::Empty => theme.field_bg,
    }
}
