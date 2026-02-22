use rand::Rng;

use crate::config::GridSize;
use crate::snake::{Position, Snake};

/// Food entity currently active on the board.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Food {
    pub position: Position,
}

impl Food {
    /// Creates food at `position`.
    #[must_use]
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    /// Returns the score value granted when eaten.
    #[must_use]
    pub fn points(self) -> u32 {
        1
    }

    /// Spawns food in an unoccupied cell.
    #[must_use]
    pub fn spawn<R: Rng + ?Sized>(rng: &mut R, bounds: GridSize, snake: &Snake) -> Self {
        Self::new(spawn_position(rng, bounds, snake))
    }
}

/// Returns a random position not currently occupied by the snake.
#[must_use]
pub fn spawn_position<R: Rng + ?Sized>(rng: &mut R, bounds: GridSize, snake: &Snake) -> Position {
    let mut candidates = Vec::new();

    for y in 0..i32::from(bounds.height) {
        for x in 0..i32::from(bounds.width) {
            let position = Position { x, y };
            if !snake.occupies(position) {
                candidates.push(position);
            }
        }
    }

    assert!(
        !candidates.is_empty(),
        "spawn_position: no free cells on the board ({}×{})",
        bounds.width,
        bounds.height,
    );

    let index = rng.gen_range(0..candidates.len());
    candidates[index]
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use crate::config::GridSize;
    use crate::input::Direction;

    use super::{Food, spawn_position};
    use crate::snake::{Position, Snake};

    #[test]
    fn food_spawn_never_overlaps_snake() {
        let mut rng = StdRng::seed_from_u64(7);
        let snake = Snake::from_segments(
            vec![
                Position { x: 0, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 2, y: 0 },
            ],
            Direction::Right,
        );

        for _ in 0..100 {
            let food_position = spawn_position(
                &mut rng,
                GridSize {
                    width: 8,
                    height: 6,
                },
                &snake,
            );
            assert!(!snake.occupies(food_position));
        }
    }

    #[test]
    fn food_grants_one_point() {
        let food = Food::new(Position { x: 1, y: 1 });
        assert_eq!(food.points(), 1);
    }
}
