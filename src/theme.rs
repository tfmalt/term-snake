use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use ratatui::style::Color;
use serde::Deserialize;

use crate::config::{Theme, fallback_theme};

#[derive(Debug, Clone)]
pub struct ThemeItem {
    pub id: String,
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub struct ThemeCatalog {
    themes: Vec<ThemeItem>,
    selected_idx: usize,
}

impl ThemeCatalog {
    /// Loads bundled and user-provided themes with OpenCode-like precedence.
    #[must_use]
    pub fn load() -> Self {
        let mut order = Vec::<String>::new();
        let mut by_id = HashMap::<String, Theme>::new();

        for path in theme_dirs() {
            merge_theme_dir(&path, &mut order, &mut by_id);
        }

        if by_id.is_empty() {
            insert_theme(
                &mut order,
                &mut by_id,
                "fallback".to_owned(),
                fallback_theme(),
            );
        }

        let mut themes = Vec::with_capacity(order.len());
        for id in order {
            if let Some(theme) = by_id.remove(&id) {
                themes.push(ThemeItem { id, theme });
            }
        }

        let selected_idx = themes
            .iter()
            .position(|theme| theme.id == "opencode")
            .unwrap_or(0);

        Self {
            themes,
            selected_idx,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.themes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.themes.is_empty()
    }

    #[must_use]
    pub fn current_theme(&self) -> &Theme {
        &self.themes[self.selected_idx].theme
    }

    #[must_use]
    pub fn current_index(&self) -> usize {
        self.selected_idx
    }

    #[must_use]
    pub fn current_id(&self) -> &str {
        &self.themes[self.selected_idx].id
    }

    #[must_use]
    pub fn items(&self) -> &[ThemeItem] {
        &self.themes
    }

    #[must_use]
    pub fn theme_at(&self, idx: usize) -> Option<&Theme> {
        self.themes.get(idx).map(|item| &item.theme)
    }

    #[must_use]
    pub fn id_at(&self, idx: usize) -> Option<&str> {
        self.themes.get(idx).map(|item| item.id.as_str())
    }

    pub fn select_index(&mut self, idx: usize) -> bool {
        if idx < self.themes.len() {
            self.selected_idx = idx;
            return true;
        }

        false
    }

    pub fn select_next(&mut self) {
        self.selected_idx = (self.selected_idx + 1) % self.themes.len();
    }

    pub fn select_previous(&mut self) {
        self.selected_idx = if self.selected_idx == 0 {
            self.themes.len() - 1
        } else {
            self.selected_idx - 1
        };
    }

    #[must_use]
    pub fn select_by_id(&mut self, id: &str) -> bool {
        if let Some(idx) = self.themes.iter().position(|theme| theme.id == id) {
            self.selected_idx = idx;
            return true;
        }

        false
    }
}

#[derive(Debug, Deserialize)]
struct ThemeFile {
    #[serde(default)]
    defs: HashMap<String, ColorValue>,
    theme: HashMap<String, ColorValue>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum ColorValue {
    String(String),
    Ansi(u8),
    Variant {
        #[serde(default)]
        dark: Option<Box<ColorValue>>,
        #[serde(default)]
        light: Option<Box<ColorValue>>,
    },
}

fn insert_theme(
    order: &mut Vec<String>,
    by_id: &mut HashMap<String, Theme>,
    id: String,
    theme: Theme,
) {
    if !by_id.contains_key(&id) {
        order.push(id.clone());
    }
    by_id.insert(id, theme);
}

fn merge_theme_dir(path: &Path, order: &mut Vec<String>, by_id: &mut HashMap<String, Theme>) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let file_path = entry.path();
        if !is_json_file(&file_path) {
            continue;
        }

        let Some(id) = file_path
            .file_stem()
            .and_then(|name| name.to_str())
            .map(str::to_owned)
        else {
            continue;
        };

        let content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(error) => {
                eprintln!(
                    "Warning: failed to read theme file {}: {error}",
                    file_path.display()
                );
                continue;
            }
        };

        match parse_theme_from_str(&id, &content) {
            Some(theme) => insert_theme(order, by_id, id, theme),
            None => eprintln!(
                "Warning: invalid theme file {}; using defaults",
                file_path.display()
            ),
        }
    }
}

fn parse_theme_from_str(id: &str, raw: &str) -> Option<Theme> {
    let parsed = serde_json::from_str::<ThemeFile>(raw).ok()?;
    let fallback = fallback_theme();
    let mut stack = Vec::new();
    let ui_muted =
        resolve_token(&parsed, "ui_muted", true, &mut stack).unwrap_or(fallback.ui_muted);
    let ui_bright_default = brighten_30_percent(ui_muted);

    Some(Theme {
        name: display_name(id),
        snake_head: resolve_token(&parsed, "snake_head", true, &mut stack)
            .unwrap_or(fallback.snake_head),
        snake_body: resolve_token(&parsed, "snake_body", true, &mut stack)
            .unwrap_or(fallback.snake_body),
        snake_tail: resolve_token(&parsed, "snake_tail", true, &mut stack)
            .unwrap_or(fallback.snake_tail),
        food: resolve_token(&parsed, "food", true, &mut stack).unwrap_or(fallback.food),
        terminal_bg: resolve_token(&parsed, "terminal_bg", true, &mut stack)
            .unwrap_or(fallback.terminal_bg),
        field_bg: resolve_token(&parsed, "field_bg", true, &mut stack).unwrap_or(fallback.field_bg),
        ui_bg: resolve_token(&parsed, "ui_bg", true, &mut stack).unwrap_or(fallback.ui_bg),
        ui_text: resolve_token(&parsed, "ui_text", true, &mut stack).unwrap_or(fallback.ui_text),
        ui_accent: resolve_token(&parsed, "ui_accent", true, &mut stack)
            .unwrap_or(fallback.ui_accent),
        ui_muted,
        ui_bright: resolve_token(&parsed, "ui_bright", true, &mut stack)
            .unwrap_or(ui_bright_default),
    })
}

fn brighten_30_percent(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(
            brighten_channel_30_percent(r),
            brighten_channel_30_percent(g),
            brighten_channel_30_percent(b),
        ),
        Color::Black => Color::DarkGray,
        Color::DarkGray => Color::Gray,
        Color::Gray => Color::White,
        Color::White => Color::White,
        other => other,
    }
}

fn brighten_channel_30_percent(channel: u8) -> u8 {
    let remaining = 255u16.saturating_sub(u16::from(channel));
    let increase = (remaining * 30 + 50) / 100;
    (u16::from(channel) + increase).min(255) as u8
}

fn resolve_token(
    file: &ThemeFile,
    token: &str,
    prefer_dark: bool,
    stack: &mut Vec<String>,
) -> Option<Color> {
    let value = file.theme.get(token)?;
    resolve_value(file, value, prefer_dark, stack)
}

fn resolve_value(
    file: &ThemeFile,
    value: &ColorValue,
    prefer_dark: bool,
    stack: &mut Vec<String>,
) -> Option<Color> {
    match value {
        ColorValue::String(s) => parse_color_string(file, s, prefer_dark, stack),
        ColorValue::Ansi(code) => Some(Color::Indexed(*code)),
        ColorValue::Variant { dark, light } => {
            let preferred = if prefer_dark {
                dark.as_deref()
            } else {
                light.as_deref()
            };
            let fallback = if prefer_dark {
                light.as_deref()
            } else {
                dark.as_deref()
            };

            preferred
                .and_then(|value| resolve_value(file, value, prefer_dark, stack))
                .or_else(|| {
                    fallback.and_then(|value| resolve_value(file, value, prefer_dark, stack))
                })
        }
    }
}

fn parse_color_string(
    file: &ThemeFile,
    value: &str,
    prefer_dark: bool,
    stack: &mut Vec<String>,
) -> Option<Color> {
    if value.eq_ignore_ascii_case("none") {
        return Some(Color::Reset);
    }

    if let Some(color) = parse_hex_color(value) {
        return Some(color);
    }

    if stack.iter().any(|seen| seen == value) {
        return None;
    }

    let referenced = file.defs.get(value).or_else(|| file.theme.get(value))?;

    stack.push(value.to_owned());
    let resolved = resolve_value(file, referenced, prefer_dark, stack);
    let _ = stack.pop();
    resolved
}

fn parse_hex_color(value: &str) -> Option<Color> {
    let hex = value.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }

    let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color::Rgb(red, green, blue))
}

fn is_json_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
}

fn display_name(id: &str) -> String {
    let mut output = String::new();
    for (idx, part) in id.split(['-', '_']).enumerate() {
        if idx > 0 {
            output.push(' ');
        }

        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            output.push(first.to_ascii_uppercase());
            output.push_str(chars.as_str());
        }
    }
    output
}

/// Returns directories to search for theme JSON files, in priority order.
///
/// Directories are searched lowest-priority first so that later entries
/// (user config) can override earlier ones (bundled assets).
///
/// Search order:
/// 1. `<exe_dir>/assets/themes` — themes distributed alongside the binary
/// 2. `<project_root>/assets/themes` — detected via `.git`, covers `cargo run`
///    from any subdirectory
/// 3. `<cwd>/assets/themes` — direct `cargo run` from the project root
/// 4. `~/.config/snake/themes` — user-specific themes and overrides
fn theme_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();

    // 1. Next to the installed binary.
    if let Ok(exe) = std::env::current_exe()
        && let Some(exe_dir) = exe.parent()
    {
        dirs.push(exe_dir.join("assets/themes"));
    }

    let cwd = std::env::current_dir().unwrap_or_default();

    // 2. Project root (git-detected) — handles running from subdirectories.
    if let Some(root) = find_project_root(&cwd) {
        let p = root.join("assets/themes");
        if !dirs.contains(&p) {
            dirs.push(p);
        }
    }

    // 3. CWD — handles `cargo run` from the project root directly.
    let p = cwd.join("assets/themes");
    if !dirs.contains(&p) {
        dirs.push(p);
    }

    // 4. User config — highest priority, overrides everything above.
    if let Some(config_dir) = dirs::config_dir() {
        dirs.push(config_dir.join("snake/themes"));
    }

    dirs
}

fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);

    while let Some(path) = current {
        if path.join(".git").exists() {
            return Some(path.to_path_buf());
        }
        current = path.parent();
    }

    None
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::{parse_hex_color, parse_theme_from_str};

    #[test]
    fn parses_hex_color() {
        assert_eq!(parse_hex_color("#AABBCC"), Some(Color::Rgb(170, 187, 204)));
        assert_eq!(parse_hex_color("invalid"), None);
    }

    #[test]
    fn resolves_defs_and_variants() {
        let json = r##"
        {
          "defs": {
            "bg": "#111111",
            "panel": { "dark": "#222222", "light": "#eeeeee" },
            "accent_ref": "#AA00AA"
          },
          "theme": {
            "snake_head":  "accent_ref",
            "snake_body":  "accent_ref",
            "snake_tail":  "accent_ref",
            "food":        "#FF0000",
            "field_bg":    "bg",
            "ui_bg":       "panel",
            "ui_text":     "#00FF00",
            "ui_accent":   "accent_ref",
            "ui_muted":    "#888888"
          }
        }
        "##;

        let theme = parse_theme_from_str("custom", json).expect("theme should parse");
        assert_eq!(theme.field_bg, Color::Rgb(17, 17, 17));
        assert_eq!(theme.ui_bg, Color::Rgb(34, 34, 34));
        assert_eq!(theme.ui_accent, Color::Rgb(170, 0, 170));
        assert_eq!(theme.ui_bright, Color::Rgb(172, 172, 172));
    }

    #[test]
    fn none_maps_to_terminal_default() {
        let json = r##"
        {
          "theme": {
            "snake_head":  "#00CC00",
            "snake_body":  "#00CC00",
            "snake_tail":  "#00CC00",
            "food":        "#FF0000",
            "field_bg":    "none",
            "ui_bg":       "none",
            "ui_text":     "#00FF00",
            "ui_accent":   "#00CC00",
            "ui_muted":    "#777777"
          }
        }
        "##;

        let theme = parse_theme_from_str("system", json).expect("theme should parse");
        assert_eq!(theme.field_bg, Color::Reset);
        assert_eq!(theme.ui_bg, Color::Reset);
    }

    #[test]
    fn explicit_ui_bright_overrides_default() {
        let json = r##"
        {
          "theme": {
            "snake_head":  "#00CC00",
            "snake_body":  "#00CC00",
            "snake_tail":  "#00CC00",
            "food":        "#FF0000",
            "field_bg":    "#000000",
            "ui_bg":       "#111111",
            "ui_text":     "#00FF00",
            "ui_accent":   "#00CC00",
            "ui_muted":    "#202020",
            "ui_bright":   "#123456"
          }
        }
        "##;

        let theme = parse_theme_from_str("custom", json).expect("theme should parse");
        assert_eq!(theme.ui_bright, Color::Rgb(18, 52, 86));
    }
}
