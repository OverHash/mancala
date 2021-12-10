use crate::board::Board;

/// Represents the current position of the game.
#[derive(Debug)]
pub struct Game {
    /// The current board of the game.
    pub current_board: Board,
    /// A Vec of all the prior boards.
    pub previous_boards: Vec<Board>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            current_board: Board::new(),
            previous_boards: Vec::new(),
        }
    }

    pub fn with_starting_position(starting_position: Board) -> Self {
        Game {
            current_board: starting_position,
            ..Self::new()
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
