use std::io;
use std::panic;
use std::thread;
use std::time::Duration;

use clap::Parser;
use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders};
use ratatui::{Frame, Terminal};
use snake::input::{GameInput, InputConfig, InputHandler};
use snake::platform::Platform;

#[derive(Debug, Parser)]
struct Cli {
    /// Disable controller input even when available.
    #[arg(long = "no-controller")]
    no_controller: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let platform = Platform::detect();

    install_panic_hook();

    run(cli, platform)?;
    cleanup_terminal()?;
    Ok(())
}

fn run(cli: Cli, platform: Platform) -> io::Result<()> {
    let mut terminal = setup_terminal()?;
    let mut input = InputHandler::new(InputConfig {
        enable_controller: !cli.no_controller,
        is_wsl: platform.is_wsl(),
    });

    loop {
        terminal.draw(render_placeholder)?;

        if let Some(game_input) = input.poll_input()? {
            if matches!(game_input, GameInput::Quit) {
                break;
            }
        }

        thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, Show, LeaveAlternateScreen)?;

    Ok(())
}

fn install_panic_hook() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        restore_terminal_after_panic();
        default_hook(panic_info);
    }));
}

fn restore_terminal_after_panic() {
    let _ = disable_raw_mode();

    let mut stdout = io::stdout();
    let _ = execute!(stdout, Show, LeaveAlternateScreen);
}

fn render_placeholder(frame: &mut Frame<'_>) {
    let area = frame.area();

    let block = Block::default()
        .title(" snake phase 2 scaffold ")
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}
