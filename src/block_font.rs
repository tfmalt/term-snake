/// Block font typeface using Unicode full and half block glyphs.
///
/// Each character is defined as 4 rows of strings (variable width).
/// Uses `█` (full block), `▀` (upper half), `▄` (lower half), and space
/// to create a chunky, geometric typeface inspired by terminal block art.
///
/// # Example
///
/// ```
/// use snake::block_font::render_text;
///
/// let lines = render_text("SNAKE");
/// for line in &lines {
///     println!("{line}");
/// }
/// ```

/// Number of rows per character in this font.
pub const FONT_HEIGHT: usize = 4;

/// Default spacing (in columns) between characters.
pub const CHAR_SPACING: usize = 1;

/// Returns the 4-row glyph for a given character, or `None` if unsupported.
pub fn glyph(ch: char) -> Option<&'static [&'static str; 4]> {
    let normalized = if ch.is_ascii_alphabetic() {
        ch.to_ascii_lowercase()
    } else {
        ch
    };

    GLYPHS
        .iter()
        .find(|(c, _)| *c == normalized)
        .map(|(_, g)| g)
}

/// Returns the display width (columns) of a single character glyph.
pub fn glyph_width(ch: char) -> usize {
    glyph(ch).map_or(0, |g| g[0].chars().count())
}

/// Renders a string into 4 lines of block text.
///
/// Unsupported characters are silently skipped.
/// Characters are separated by `spacing` columns of space.
pub fn render_text(text: &str) -> Vec<String> {
    render_text_with_spacing(text, CHAR_SPACING)
}

/// Renders a string into 4 lines of block text with custom spacing.
pub fn render_text_with_spacing(text: &str, spacing: usize) -> Vec<String> {
    let mut rows = vec![String::new(); FONT_HEIGHT];
    let spacer = " ".repeat(spacing);
    let mut first = true;

    for ch in text.chars() {
        if let Some(g) = glyph(ch) {
            for (i, row) in g.iter().enumerate() {
                if !first && i == 0 {
                    // Add spacer to all rows (done below)
                }
                if !first {
                    rows[i].push_str(&spacer);
                }
                rows[i].push_str(row);
            }
            first = false;
        }
    }

    rows
}

/// Returns the total display width of a rendered string.
pub fn text_width(text: &str) -> usize {
    text_width_with_spacing(text, CHAR_SPACING)
}

/// Returns the total display width of a rendered string with custom spacing.
pub fn text_width_with_spacing(text: &str, spacing: usize) -> usize {
    let mut width = 0;
    let mut count = 0;

    for ch in text.chars() {
        if glyph(ch).is_some() {
            if count > 0 {
                width += spacing;
            }
            width += glyph_width(ch);
            count += 1;
        }
    }

    width
}

// ── Glyph definitions ──────────────────────────────────────────────────
//
// Each glyph is 4 rows tall, variable width.
// Design principles:
//   ▄  at top edges    → letter "rises" smoothly
//   ▀  at bottom edges → letter "sinks" smoothly
//   █  for solid fills
//   ·  (space) for cutouts / negative space

#[rustfmt::skip]
const GLYPHS: &[(char, [&str; 4])] = &[
    // ── Letters (lowercase) ─────────────────────────────────────────────
    ('a', [
        "    ",
        "▀▀▀█",
        "█▀▀█",
        "▀▀▀▀",
    ]),
    ('b', [
        "▄   ",
        "█▀▀█",
        "█  █",
        "▀▀▀▀",
    ]),
    ('c', [
        "    ",
        "█▀▀▀",
        "█   ",
        "▀▀▀▀",
    ]),
    ('d', [
        "   ▄",
        "█▀▀█",
        "█  █",
        "▀▀▀▀",
    ]),
    ('e', [
        "    ",
        "█▀▀█",
        "█▀▀▀",
        "▀▀▀▀",
    ]),
    ('f', [
        " ▄▀ ",
        " █  ",
        "▀█▀▀",
        " ▀  ",
    ]),
    ('g', [
        "    ",
        "█▀▀█",
        "▀▀▀█",
        "▀▀▀▀",
    ]),
    ('h', [
        "    ",
        "█  █",
        "█▀▀█",
        "▀  ▀",
    ]),
    ('i', [ 
        "▀",
        "█",
        "█",
        "▀",
    ]),
    ('j', [
        "  ▄",
        "  █",
        "  █",
        "▀█ ",
    ]),
    ('k', [
        "    ",
        "█  █",
        "█▀▀▄",
        "▀  ▀",
    ]),
    ('l', [
        "    ",
        "█   ",
        "█   ",
        "▀▀▀▀",
    ]),
    ('m', [
        "     ",
        "█▀▄▀█",
        "█ █ █",
        "▀ ▀ ▀",
    ]),
    ('n', [
        "    ",
        "█▀▀▄",
        "█  █",
        "▀  ▀",
    ]),
    ('o', [
        "    ",
        "█▀▀█",
        "█  █",
        "▀▀▀▀",
    ]),
    ('p', [
        "    ",
        "███▄",
        "███▀",
        "█   ",
    ]),
    ('q', [
        "    ",
        "▄██▄",
        "█ ▄█",
        "▀█▀█",
    ]),
    ('r', [
        "    ",
        "▄▀▀▀",
        "█   ",
        "▀   ",
    ]),
    ('s', [
        "    ",
        "█▀▀▀",
        "▀▀▀█",
        "▀▀▀▀",
    ]),
    ('t', [
        "     ",
        "▀▀█▀▀",
        "  █  ",
        "  ▀  ",
    ]),
    ('u', [
        "    ",
        "█  █",
        "█  █",
        "▀██▀",
    ]),
    ('v', [
        "    ",
        "█  █",
        "▀▄▄▀",
        " ██ ",
    ]),
    ('w', [
        "     ",
        "█   █",
        "█ █ █",
        "▀█▀█▀",
    ]),
    ('x', [
        "    ",
        "█  █",
        " ██ ",
        "█  █",
    ]),
    ('y', [
        "    ",
        "█  █",
        "▀██▀",
        " ██ ",
    ]),
    ('z', [
        "    ",
        "████",
        " ▄█▀",
        "████",
    ]),

    // ── Digits ──────────────────────────────────────────────────────────
    ('0', [
        "    ",
        "▄██▄",
        "█▄▀█",
        "▀██▀",
    ]),
    ('1', [
        "   ",
        "▄█ ",
        " █ ",
        "███",
    ]),
    ('2', [
        "    ",
        "▄██▄",
        "▄██▀",
        "████",
    ]),
    ('3', [
        "    ",
        "███▄",
        " ██▄",
        "███▀",
    ]),
    ('4', [
        "    ",
        "█  █",
        "▀███",
        "   █",
    ]),
    ('5', [
        "    ",
        "████",
        "▀██▄",
        "███▀",
    ]),
    ('6', [
        "    ",
        "▄███",
        "███▄",
        "▀██▀",
    ]),
    ('7', [
        "    ",
        "████",
        "  ▄█",
        "  █ ",
    ]),
    ('8', [
        "    ",
        "▄██▄",
        "▄██▄",
        "▀██▀",
    ]),
    ('9', [
        "    ",
        "▄██▄",
        "▀███",
        " ██▀",
    ]),

    // ── Punctuation & symbols ───────────────────────────────────────────
    (' ', [
        "  ",
        "  ",
        "  ",
        "  ",
    ]),
    ('!', [
        "  ",
        "██",
        "██",
        "▀▀",
    ]),
    ('.', [
        "  ",
        "  ",
        "  ",
        "▀▀",
    ]),
    (',', [
        "  ",
        "  ",
        "  ",
        "▄█",
    ]),
    (':', [
        "  ",
        "▄▄",
        "  ",
        "▀▀",
    ]),
    ('-', [
        "    ",
        "    ",
        "████",
        "    ",
    ]),
    ('?', [
        "    ",
        "▄██▄",
        " ▄█▀",
        " ▀▀ ",
    ]),
    ('/', [
        "    ",
        "  ▄█",
        " █▀ ",
        "█▀  ",
    ]),
    ('\'', [
        "  ",
        "██",
        "▀▀",
        "  ",
    ]),
    ('"', [
        "     ",
        "██ ██",
        "▀▀ ▀▀",
        "     ",
    ]),
    ('#', [
        "     ",
        " █ █ ",
        "█████",
        " █ █ ",
    ]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_glyphs_have_consistent_row_widths() {
        for &(ch, ref rows) in GLYPHS {
            let w: usize = rows[0].chars().count();
            for (i, row) in rows.iter().enumerate() {
                assert_eq!(
                    row.chars().count(),
                    w,
                    "Glyph '{ch}' row {i} width {} != expected {w}",
                    row.chars().count()
                );
            }
        }
    }

    #[test]
    fn render_snake() {
        let lines = render_text("SNAKE");
        assert_eq!(lines.len(), FONT_HEIGHT);
        // Each line should be non-empty
        for line in &lines {
            assert!(!line.is_empty());
        }
    }

    #[test]
    fn text_width_matches_render() {
        let text = "HELLO";
        let lines = render_text(text);
        let expected = text_width(text);
        let actual = lines[0].chars().count();
        assert_eq!(actual, expected, "text_width disagrees with render");
    }

    #[test]
    fn space_inserts_gap() {
        let lines = render_text("A B");
        // Should contain glyphs for A, space, B with spacing between each
        assert_eq!(lines.len(), FONT_HEIGHT);
        assert!(lines[0].chars().count() > glyph_width('A') + glyph_width('B'));
    }
}
