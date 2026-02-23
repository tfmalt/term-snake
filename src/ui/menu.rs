use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph};
use std::time::Duration;

use crate::config::{GLYPH_HALF_LOWER, GLYPH_HALF_UPPER, Theme};
use crate::game::{DeathReason, FoodDensity};
use crate::theme::ThemeItem;

pub struct ThemeSelectView<'a> {
    pub selected_idx: usize,
    pub themes: &'a [ThemeItem],
}

/// Draws the start screen as a centered popup.
pub fn render_start_menu(
    frame: &mut Frame<'_>,
    area: Rect,
    _high_score: u32,
    theme: &Theme,
    selected_idx: usize,
    food_density: FoodDensity,
    theme_select: Option<ThemeSelectView<'_>>,
) {
    let popup = centered_popup(area, 70, 45);
    frame.render_widget(Clear, popup);
    render_menu_panel(frame, popup, theme);

    let [_, title_row, body_row, footer_row] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(3),
        Constraint::Length(2),
    ])
    .areas(popup);

    frame.render_widget(
        Paragraph::new(Line::from("TERMINAL SNAKE"))
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(theme.ui_accent)
                    .add_modifier(Modifier::BOLD),
            ),
        title_row,
    );

    let body = vec![
        menu_option_line("Start", selected_idx == 0, theme),
        menu_option_line(format!("Theme:  {}", theme.name), selected_idx == 1, theme),
        menu_option_line(
            format_food_menu_label(food_density),
            selected_idx == 2,
            theme,
        ),
        menu_option_line("Quit", selected_idx == 3, theme),
    ];
    frame.render_widget(
        Paragraph::new(body)
            .alignment(Alignment::Left)
            .style(menu_body_style(theme)),
        body_row,
    );

    frame.render_widget(
        Paragraph::new(Line::from("Use arrows/WASD or D-pad/stick to move"))
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.ui_muted)),
        footer_row,
    );

    render_menu_bottom_margin(frame, popup, theme);

    if let Some(select_view) = theme_select {
        render_theme_select_list(frame, area, theme, &select_view);
    }
}

/// Draws the pause screen as a centered popup.
pub fn render_pause_menu(
    frame: &mut Frame<'_>,
    area: Rect,
    theme: &Theme,
    selected_idx: usize,
    food_density: FoodDensity,
    theme_select: Option<ThemeSelectView<'_>>,
) {
    let popup = centered_popup(area, 60, 30);
    frame.render_widget(Clear, popup);
    render_menu_panel(frame, popup, theme);

    let lines = vec![
        Line::from("PAUSED").style(Style::default().fg(theme.ui_accent)),
        Line::from(""),
        menu_option_line("Resume", selected_idx == 0, theme),
        menu_option_line(format!("Theme:  {}", theme.name), selected_idx == 1, theme),
        menu_option_line(
            format_food_menu_label(food_density),
            selected_idx == 2,
            theme,
        ),
        menu_option_line("Quit", selected_idx == 3, theme),
    ];
    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Left)
            .style(menu_body_style(theme)),
        popup,
    );

    render_menu_bottom_margin(frame, popup, theme);

    if let Some(select_view) = theme_select {
        render_theme_select_list(frame, area, theme, &select_view);
    }
}

/// Draws the game-over screen as a centered popup.
#[allow(clippy::too_many_arguments)]
pub fn render_game_over_menu(
    frame: &mut Frame<'_>,
    area: Rect,
    score: u32,
    high_score: u32,
    death_reason: Option<DeathReason>,
    game_length: Duration,
    theme: &Theme,
    selected_idx: usize,
) {
    let popup = centered_popup(area, 70, 45);
    frame.render_widget(Clear, popup);
    render_menu_panel(frame, popup, theme);

    let [_, title_row, body_row, footer_row] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(3),
        Constraint::Length(2),
    ])
    .areas(popup);

    let is_new_high = score > high_score;
    frame.render_widget(
        Paragraph::new(Line::from("GAME OVER"))
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(theme.ui_accent)
                    .add_modifier(Modifier::BOLD),
            ),
        title_row,
    );

    let shown_high_score = if is_new_high { score } else { high_score };
    let seconds = game_length.as_secs_f64();
    let foods_per_minute = if seconds > 0.0 {
        (f64::from(score) / seconds) * 60.0
    } else {
        0.0
    };

    let mut body = vec![
        table_row("Metric", "Value"),
        table_row("Score", score.to_string()),
        table_row("High score", shown_high_score.to_string()),
        table_row(
            "Cause",
            match death_reason {
                Some(DeathReason::WallCollision) => "hit wall".to_string(),
                Some(DeathReason::SelfCollision) => "hit yourself".to_string(),
                None => "-".to_string(),
            },
        ),
        table_row("Game length", format_game_length(game_length)),
        table_row("Food/min", format!("{foods_per_minute:.1}")),
        Line::from(""),
    ];

    if is_new_high {
        body.push(Line::from("New high score!"));
        body.push(Line::from(""));
    }

    body.push(menu_option_line("Play Again", selected_idx == 0, theme));
    body.push(menu_option_line("Quit", selected_idx == 1, theme));

    frame.render_widget(
        Paragraph::new(body)
            .alignment(Alignment::Left)
            .style(menu_body_style(theme)),
        body_row,
    );

    frame.render_widget(
        Paragraph::new(Line::from("Use arrows/WASD or D-pad/stick to move"))
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.ui_muted)),
        footer_row,
    );

    render_menu_bottom_margin(frame, popup, theme);
}

fn centered_popup(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let [_, mid, _] = Layout::vertical([
        Constraint::Percentage((100 - height_percent) / 2),
        Constraint::Percentage(height_percent),
        Constraint::Percentage((100 - height_percent) / 2),
    ])
    .areas(area);

    let [_, center, _] = Layout::horizontal([
        Constraint::Percentage((100 - width_percent) / 2),
        Constraint::Percentage(width_percent),
        Constraint::Percentage((100 - width_percent) / 2),
    ])
    .areas(mid);

    center
}

fn render_theme_select_list(
    frame: &mut Frame<'_>,
    area: Rect,
    active_theme: &Theme,
    select_view: &ThemeSelectView<'_>,
) {
    let desired_list_height = u16::try_from(select_view.themes.len().max(1)).unwrap_or(u16::MAX);
    let desired_popup_height = desired_list_height;
    let base_popup = centered_popup(area, 52, 40);
    let desired_popup_width = theme_list_width(select_view.themes);
    let popup = left_anchored_popup_with_size(
        area,
        base_popup.x,
        desired_popup_width,
        desired_popup_height,
    );
    frame.render_widget(Clear, popup);
    render_menu_panel(frame, popup, active_theme);
    let inner = popup;

    let list_height = desired_list_height.min(inner.height.max(1));

    let [list_row] = Layout::vertical([Constraint::Length(list_height)]).areas(inner);

    let items = visible_theme_lines(
        select_view.themes,
        select_view.selected_idx,
        usize::from(list_height),
        active_theme,
    );
    frame.render_widget(
        Paragraph::new(items)
            .alignment(Alignment::Left)
            .style(theme_select_list_style(active_theme)),
        list_row,
    );

    render_menu_bottom_margin(frame, popup, active_theme);

    if let Some(preview_area) = right_preview_area(area, popup) {
        render_theme_preview_palette(frame, preview_area, active_theme);
    }
}

fn right_preview_area(container: Rect, anchor: Rect) -> Option<Rect> {
    let x = anchor.right().saturating_add(1);
    if x >= container.right() {
        return None;
    }

    let available_width = container.right().saturating_sub(x);
    if available_width < 18 {
        return None;
    }

    Some(Rect {
        x,
        y: anchor.y,
        width: available_width.min(30),
        height: anchor.height,
    })
}

fn render_theme_preview_palette(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    frame.render_widget(Clear, area);
    render_menu_panel(frame, area, theme);

    let [_, content] = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(area);

    frame.render_widget(
        Paragraph::new(theme_preview_lines(theme))
            .alignment(Alignment::Left)
            .style(menu_body_style(theme)),
        content,
    );

    render_menu_bottom_margin(frame, area, theme);
}

fn theme_preview_lines(theme: &Theme) -> Vec<Line<'static>> {
    vec![
        Line::from("Preview"),
        Line::from(""),
        swatch_line("Head", theme.snake_head, "snake head"),
        swatch_line("Body", theme.snake_body, "snake body"),
        swatch_line("Tail", theme.snake_tail, "snake tail"),
        swatch_line("Food", theme.food, "food"),
        swatch_line("Term bg", theme.terminal_bg, "terminal_bg"),
        swatch_line("Field", theme.field_bg, "field_bg"),
        swatch_line("UI bg", theme.ui_bg, "ui_bg"),
        swatch_line("UI text", theme.ui_text, "ui_text"),
    ]
}

fn swatch_line(label: &str, color: ratatui::style::Color, usage: &str) -> Line<'static> {
    Line::from(vec![
        Span::raw(format!("{label:<7} ")),
        Span::styled("   ", Style::default().bg(color)),
        Span::raw(format!(" {usage}")),
    ])
}

fn left_anchored_popup_with_size(area: Rect, x: u16, width: u16, height: u16) -> Rect {
    let left = x.clamp(area.x, area.right().saturating_sub(1));
    let max_width = area.right().saturating_sub(left);
    let popup_width = width.min(max_width).max(1);
    let popup_height = height.min(area.height).max(1);

    let y = area.y + area.height.saturating_sub(popup_height) / 2;

    Rect {
        x: left,
        y,
        width: popup_width,
        height: popup_height,
    }
}

fn visible_theme_lines(
    themes: &[ThemeItem],
    selected_idx: usize,
    count: usize,
    active_theme: &Theme,
) -> Vec<Line<'static>> {
    let longest_name = longest_theme_name_width(themes);

    if themes.is_empty() {
        return vec![Line::from(format!(
            " {:<width$} ",
            "No themes available",
            width = longest_name,
        ))];
    }

    let show_count = count.min(themes.len());
    let center = show_count / 2;
    let start = (selected_idx + themes.len() - center) % themes.len();

    let mut lines = Vec::with_capacity(show_count);
    for offset in 0..show_count {
        let idx = (start + offset) % themes.len();
        let line = format!(" {:<width$} ", themes[idx].theme.name, width = longest_name,);
        if idx == selected_idx {
            lines.push(
                Line::from(line).style(
                    Style::default()
                        .fg(active_theme.ui_bg)
                        .bg(active_theme.ui_text)
                        .add_modifier(Modifier::BOLD),
                ),
            );
        } else {
            lines.push(Line::from(line));
        }
    }
    lines
}

fn menu_option_line<T: Into<String>>(label: T, selected: bool, theme: &Theme) -> Line<'static> {
    let prefix = if selected { "> " } else { "  " };
    let line = format!("{prefix}{}", label.into());
    if selected {
        Line::from(line).style(
            Style::default()
                .fg(theme.ui_accent)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Line::from(line)
    }
}

fn format_food_menu_label(food_density: FoodDensity) -> String {
    format!(
        "Food:   {}/{}",
        food_density.foods_per, food_density.cells_per
    )
}

fn table_row(label: &str, value: impl AsRef<str>) -> Line<'static> {
    Line::from(format!("{label:<14} {value}", value = value.as_ref()))
}

fn format_game_length(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    format!("{minutes:02}:{seconds:02}")
}

fn render_menu_panel(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    frame.render_widget(
        Paragraph::new("").style(Style::default().bg(theme.ui_bg).fg(theme.ui_text)),
        area,
    );

    if area.height < 2 {
        return;
    }

    let top_y = area.y;
    let bottom_y = area.bottom().saturating_sub(1);
    let margin_style = Style::default().fg(theme.ui_bg).bg(theme.field_bg);
    let buffer = frame.buffer_mut();

    for x in area.x..area.right() {
        buffer.set_string(x, top_y, GLYPH_HALF_LOWER, margin_style);
        buffer.set_string(x, bottom_y, GLYPH_HALF_UPPER, margin_style);
    }
}

fn render_menu_bottom_margin(frame: &mut Frame<'_>, area: Rect, theme: &Theme) {
    if area.height < 1 {
        return;
    }

    let bottom_y = area.bottom().saturating_sub(1);
    let margin_style = Style::default().fg(theme.ui_bg).bg(theme.field_bg);
    let buffer = frame.buffer_mut();
    for x in area.x..area.right() {
        buffer.set_string(x, bottom_y, GLYPH_HALF_UPPER, margin_style);
    }
}

fn menu_body_style(theme: &Theme) -> Style {
    Style::default().fg(theme.ui_text).bg(theme.ui_bg)
}

fn theme_select_list_style(theme: &Theme) -> Style {
    Style::default().fg(theme.ui_text).bg(theme.field_bg)
}

fn theme_list_width(themes: &[ThemeItem]) -> u16 {
    let longest_name = longest_theme_name_width(themes);

    let width_with_margin = longest_name.saturating_add(2);
    width_with_margin.min(u16::MAX as usize) as u16
}

fn longest_theme_name_width(themes: &[ThemeItem]) -> usize {
    if themes.is_empty() {
        "No themes available".chars().count()
    } else {
        themes
            .iter()
            .map(|theme| theme.theme.name.chars().count())
            .max()
            .unwrap_or(0)
    }
}
