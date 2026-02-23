use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::config::Theme;
use crate::game::{GameState, GameStatus};
use crate::platform::Platform;

/// Supplemental values displayed by the HUD rows.
#[derive(Debug, Clone)]
pub struct HudInfo<'a> {
    pub high_score: u32,
    pub game_over_reference_high_score: u32,
    pub controller_enabled: bool,
    pub theme: &'a Theme,
    /// Whether the debug row is enabled (`--debug` flag).
    pub debug: bool,
    /// Pre-formatted debug string; empty when `debug` is false.
    pub debug_line: String,
}

/// Renders the two-line HUD and returns the remaining play area above it.
#[must_use]
pub fn render_hud(
    frame: &mut Frame<'_>,
    area: Rect,
    state: &GameState,
    platform: Platform,
    info: &HudInfo<'_>,
) -> Rect {
    let debug_height = u16::from(info.debug);
    let [play_area, score_area, status_area, debug_area] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(debug_height),
    ])
    .areas(area);

    // Score line: Score | Speed | Hi + flags
    let [left, center, right] = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .areas(score_area);

    frame.render_widget(
        Paragraph::new(Line::from(format!("Score: {}", state.score)))
            .alignment(Alignment::Left)
            .style(left_style(info.theme)),
        left,
    );

    frame.render_widget(
        Paragraph::new(Line::from(format!("Speed: {}", state.speed_level)))
            .alignment(Alignment::Center),
        center,
    );

    let right_text = format!(
        "Hi: {} {}{}",
        info.high_score,
        if info.controller_enabled {
            "[PAD]"
        } else {
            "[NOPAD]"
        },
        if platform.is_wsl() { " [WSL]" } else { "" }
    );
    frame.render_widget(
        Paragraph::new(Line::from(right_text)).alignment(Alignment::Right),
        right,
    );

    // Status line: game state label
    let dimensions_text = format!("{}x{}", state.bounds().width, state.bounds().height);
    let food_count_text = state.calculated_food_count().to_string();
    let [_, status_center, status_right] = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ])
    .areas(status_area);

    frame.render_widget(
        Paragraph::new(Line::from(status_label(state, platform)))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray)),
        status_center,
    );
    frame.render_widget(
        Paragraph::new(bottom_info_line(
            dimensions_text.as_str(),
            food_count_text.as_str(),
            info.theme.food,
        ))
        .alignment(Alignment::Right)
        .style(Style::default().fg(Color::DarkGray)),
        status_right,
    );

    if info.debug {
        let debug_width = bottom_info_width(dimensions_text.as_str(), food_count_text.as_str())
            .min(u16::MAX as usize) as u16;
        let [debug_left, debug_right] =
            Layout::horizontal([Constraint::Min(0), Constraint::Length(debug_width)])
                .areas(debug_area);

        frame.render_widget(
            Paragraph::new(Line::from(info.debug_line.as_str()))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::DarkGray)),
            debug_left,
        );
        frame.render_widget(
            Paragraph::new(bottom_info_line(
                dimensions_text.as_str(),
                food_count_text.as_str(),
                info.theme.food,
            ))
            .alignment(Alignment::Right)
            .style(Style::default().fg(Color::DarkGray)),
            debug_right,
        );
    }

    play_area
}

fn status_label(state: &GameState, platform: Platform) -> &'static str {
    match state.status {
        GameStatus::Paused if state.is_start_screen() => {
            if platform.is_wsl() {
                "snake (wsl)"
            } else {
                "snake"
            }
        }
        GameStatus::Playing => {
            if platform.is_wsl() {
                "snake (wsl)"
            } else {
                "snake"
            }
        }
        GameStatus::Paused => "paused",
        GameStatus::GameOver => "game over",
        GameStatus::Victory => "victory",
    }
}

fn left_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.ui_text)
        .add_modifier(Modifier::BOLD)
}

fn bottom_info_line<'a>(dimensions: &'a str, food_count: &'a str, food_color: Color) -> Line<'a> {
    Line::from(vec![
        Span::raw(format!("{dimensions}  ")),
        Span::styled("█", Style::default().fg(food_color)),
        Span::raw(format!(" = {food_count}")),
    ])
}

fn bottom_info_width(dimensions: &str, food_count: &str) -> usize {
    dimensions.chars().count() + 2 + 1 + 3 + food_count.chars().count()
}
