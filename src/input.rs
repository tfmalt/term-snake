/// Canonical movement directions for snake input.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the opposite direction.
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// High-level input events consumed by the game loop.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameInput {
    Direction(Direction),
    Pause,
    Quit,
    Confirm,
}

/// Non-blocking input poller for keyboard and controller sources.
#[derive(Debug, Default)]
pub struct InputHandler;

impl InputHandler {
    /// Builds a new input handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Polls for one input event without blocking the game loop.
    pub fn poll_input(&mut self) -> io::Result<Option<GameInput>> {
        if event::poll(Duration::from_millis(0))? {
            let terminal_event = event::read()?;
            return Ok(map_terminal_event(terminal_event));
        }

        Ok(None)
    }
}

/// Returns whether a direction change is legal (no immediate 180° turns).
#[must_use]
pub fn direction_change_is_valid(current: Direction, next: Direction) -> bool {
    next != current.opposite()
}

fn map_terminal_event(event: Event) -> Option<GameInput> {
    let Event::Key(key_event) = event else {
        return None;
    };

    map_key_event(key_event)
}

fn map_key_event(key_event: KeyEvent) -> Option<GameInput> {
    let key_code = key_event.code;

    if matches!(key_code, KeyCode::Char('c')) && key_event.modifiers.contains(KeyModifiers::CONTROL)
    {
        return Some(GameInput::Quit);
    }

    match key_code {
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            Some(GameInput::Direction(Direction::Up))
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            Some(GameInput::Direction(Direction::Down))
        }
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
            Some(GameInput::Direction(Direction::Left))
        }
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
            Some(GameInput::Direction(Direction::Right))
        }
        KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Esc => Some(GameInput::Pause),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(GameInput::Quit),
        KeyCode::Enter | KeyCode::Char(' ') => Some(GameInput::Confirm),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::{direction_change_is_valid, map_key_event, Direction, GameInput};

    #[test]
    fn opposite_direction_is_correct() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Right.opposite(), Direction::Left);
    }

    #[test]
    fn direction_buffer_rejects_reverse() {
        assert!(!direction_change_is_valid(Direction::Up, Direction::Down));
        assert!(!direction_change_is_valid(Direction::Down, Direction::Up));
        assert!(!direction_change_is_valid(
            Direction::Left,
            Direction::Right
        ));
        assert!(!direction_change_is_valid(
            Direction::Right,
            Direction::Left
        ));

        assert!(direction_change_is_valid(Direction::Up, Direction::Left));
        assert!(direction_change_is_valid(Direction::Up, Direction::Right));
    }

    #[test]
    fn keyboard_mapping_supports_wasd_and_arrows() {
        let up = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE);
        let right = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);

        assert_eq!(map_key_event(up), Some(GameInput::Direction(Direction::Up)));
        assert_eq!(
            map_key_event(right),
            Some(GameInput::Direction(Direction::Right))
        );
    }

    #[test]
    fn keyboard_mapping_supports_quit_pause_and_confirm() {
        let quit = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let pause = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let confirm = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

        assert_eq!(map_key_event(quit), Some(GameInput::Quit));
        assert_eq!(map_key_event(pause), Some(GameInput::Pause));
        assert_eq!(map_key_event(confirm), Some(GameInput::Confirm));
        assert_eq!(map_key_event(ctrl_c), Some(GameInput::Quit));
    }
}
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
