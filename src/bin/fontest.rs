use clap::Parser;
use snake::block_font::{CHAR_SPACING, render_text_with_spacing};

#[derive(Debug, Parser)]
#[command(name = "fontest", about = "Print block-font text to stdout")]
struct Cli {
    /// Text to render in the block font.
    #[arg(required = true)]
    text: Vec<String>,

    /// Number of spaces between rendered glyphs.
    #[arg(long, default_value_t = CHAR_SPACING)]
    spacing: usize,
}

fn main() {
    let cli = Cli::parse();
    let text = cli.text.join(" ");
    for line in render_text_with_spacing(&text, cli.spacing) {
        println!("{line}");
    }
}
