use std::fmt::Display;

use gridpx::Grid;

use crate::moves::Move;

// Represents a Tic-Tac-Toe board using a 3x3 Grid from gridpx.
#[derive(Debug, Clone)]
pub struct Board {
    grid: gridpx::Grid<3, 3, Value>,
}

// Represents the value of a tile on the board: O, X, or None (empty).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    O,
    X,
    None,
}

// Represents the state of the game: Ongoing, Draw, or one of the players winning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    OnGoing,
    Draw,
    XWins,
    OWins,
}

// The 8 possible winning line configurations (3 rows, 3 columns, 2 diagonals)
// represented as 1D indices (0-8) on a 3x3 board.
const WINNING_INDEXES: [[usize; 3]; 8] = [
    [0, 1, 2], // Top row
    [3, 4, 5], // Middle row
    [6, 7, 8], // Bottom row
    [0, 3, 6], // Left column
    [1, 4, 7], // Middle column
    [2, 5, 8], // Right column
    [0, 4, 8], // Diagonal from top-left to bottom-right
    [2, 4, 6], // Diagonal from top-right to bottom-left
];

impl Board {
    // Creates a new empty Tic-Tac-Toe board.
    pub fn new() -> Self {
        Self {
            grid: Grid::new(Value::None),
        }
    }

    // Attempts to place 'X' at the specified move location.
    // Returns Some(()) if the move was successful, or None if the tile was already occupied.
    pub fn place_x(&mut self, m: Move) -> Option<()> {
        let (x, y) = m.into();
        if self.grid.get_value_at(x, y) != Ok(&Value::None) {
            return None;
        }

        self.grid.set_value_at(x, y, Value::X).unwrap();
        Some(())
    }

    // Attempts to place 'O' at the specified move location.
    // Returns Some(()) if the move was successful, or None if the tile was already occupied.
    pub fn place_o(&mut self, m: Move) -> Option<()> {
        let (x, y) = m.into();
        if self.grid.get_value_at(x, y) != Ok(&Value::None) {
            return None;
        }

        self.grid.set_value_at(x, y, Value::O).unwrap();
        Some(())
    }

    // Evaluates the current game state to check for a win, draw, or ongoing game.
    pub fn evaluate_board(&self) -> State {
        // Check all winning line configurations.
        for [a, b, c] in WINNING_INDEXES {
            if self.grid[a] != Value::None
                && self.grid[a] == self.grid[b]
                && self.grid[a] == self.grid[c]
            {
                return match self.grid[a] {
                    Value::O => State::OWins,
                    Value::X => State::XWins,
                    Value::None => unreachable!(),
                };
            }
        }

        // If no one won and there are empty tiles remaining, the game is ongoing.
        if self.grid.as_slice().iter().any(|tile| tile == &Value::None) {
            State::OnGoing
        } else {
            // Otherwise, it's a draw.
            State::Draw
        }
    }

    // Returns a reference to the underlying grid.
    pub fn get_grid(&self) -> &Grid<3, 3, Value> {
        &self.grid
    }
}

// Pretty prints the board to standard output with box-drawing characters.
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let g = &self.grid;
        f.write_str("┌─┬─┬─┐\n")?;
        f.write_str(&format!("│{}│{}│{}│\n", g[0], g[1], g[2]))?;
        f.write_str("├─┼─┼─┤\n")?;
        f.write_str(&format!("│{}│{}│{}│\n", g[3], g[4], g[5]))?;
        f.write_str("├─┼─┼─┤\n")?;
        f.write_str(&format!("│{}│{}│{}│\n", g[6], g[7], g[8]))?;
        f.write_str("└─┴─┴─┘\n")?;

        Ok(())
    }
}

// Formats Value for pretty printing: X for X, O for O, and spaces for empty cells.
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::O => write!(f, "O"),
            Value::X => write!(f, "X"),
            Value::None => write!(f, " "),
        }
    }
}

// Allows comparing the board layout with a compact string format.
// e.g., "XNN NXN NNX" where N stands for None/empty.
impl PartialEq<&str> for Board {
    fn eq(&self, other: &&str) -> bool {
        let other = other.replace(" ", "");
        let mut str_iter = other.chars();
        for value in self.grid.as_slice().iter() {
            let value_char = match value {
                Value::O => 'O',
                Value::X => 'X',
                Value::None => 'N',
            };

            if str_iter.next() == Some(value_char) {
                continue;
            }

            return false;
        }

        true
    }
}

// Helper to construct a Board directly from a string representation.
impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let value = value.replace(" ", "");
        let str_iter = value.chars();

        let values: Vec<_> = str_iter
            .map(|v_s| match v_s {
                'O' => Value::O,
                'X' => Value::X,
                _ => Value::None,
            })
            .collect();

        let mut grid = Grid::new(Value::None);
        grid.set_array(values.as_slice());

        Self { grid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_string_comparison() {
        let mut board = Board::new();
        board.place_x(Move::TopLeft);
        board.place_x(Move::Center);
        board.place_x(Move::BottomRight);

        assert_eq!(board, "XNN NXN NNX");
    }

    #[test]
    fn test_x_wins_row() {
        let board_str = "XXX OON NNN";
        let board = Board::from(board_str);

        assert_eq!(board.evaluate_board(), State::XWins);
    }

    #[test]
    fn test_o_wins_diagonal() {
        let board_str = "OXN XON XNO";
        let board = Board::from(board_str);

        assert_eq!(board.evaluate_board(), State::OWins);
    }

    #[test]
    fn test_draw() {
        let board_str = "XOX XOO OXX";
        let board = Board::from(board_str);

        assert_eq!(board.evaluate_board(), State::Draw);
    }

    #[test]
    fn test_ongoing() {
        let board_str = "XON NNN NNN";
        let board = Board::from(board_str);

        assert_eq!(board.evaluate_board(), State::OnGoing);
    }
}
