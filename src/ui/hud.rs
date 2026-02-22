use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::config::Theme;
use crate::game::GameState;
use crate::platform::Platform;

/// Supplemental values displayed by the top HUD row.
#[derive(Debug, Clone)]
pub struct HudInfo {
    pub high_score: u32,
    pub game_over_reference_high_score: u32,
    pub controller_enabled: bool,
    pub theme: &'static Theme,
    /// Whether the debug row is enabled (`--debug` flag).
    pub debug: bool,
    /// Pre-formatted debug string; empty when `debug` is false.
    pub debug_line: String,
}

impl Default for HudInfo {
    fn default() -> Self {
        use crate::config::THEME_CLASSIC;
        Self {
            high_score: 0,
            game_over_reference_high_score: 0,
            controller_enabled: true,
            theme: &THEME_CLASSIC,
            debug: false,
            debug_line: String::new(),
        }
    }
}

/// Renders HUD and returns the remaining play area below it.
#[must_use]
pub fn render_hud(
    frame: &mut Frame<'_>,
    area: Rect,
    state: &GameState,
    platform: Platform,
    info: &HudInfo,
) -> Rect {
    let debug_height = u16::from(info.debug);
    let [play_area, hud_area, debug_area] = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(debug_height),
    ])
    .areas(area);
    let [left, center, right] = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ])
    .areas(hud_area);

    let left_text = format!("Score: {}", state.score);
    frame.render_widget(
        Paragraph::new(Line::from(left_text))
            .alignment(Alignment::Left)
            .style(left_style(info.theme)),
        left,
    );

    let center_text = format!("Speed: {}", state.speed_level);
    frame.render_widget(
        Paragraph::new(Line::from(center_text)).alignment(Alignment::Center),
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

    if info.debug {
        frame.render_widget(
            Paragraph::new(Line::from(info.debug_line.as_str()))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::DarkGray)),
            debug_area,
        );
    }

    play_area
}

fn left_style(theme: &Theme) -> Style {
    Style::default()
        .fg(theme.hud_score)
        .add_modifier(Modifier::BOLD)
}
