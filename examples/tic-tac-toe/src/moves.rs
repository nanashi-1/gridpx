// Represents a valid move on a 3x3 Tic-Tac-Toe board.
// Each variant corresponds to a specific cell in the grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    TopLeft,
    Top,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    Bottom,
    BottomRight,
}

// Converts a Move variant into 2D grid coordinates (x, y).
// Grid is 0-indexed: top-left is (0, 0), bottom-right is (2, 2).
impl From<Move> for (usize, usize) {
    fn from(value: Move) -> Self {
        use Move::*;
        match value {
            TopLeft => (0, 0),
            Top => (1, 0),
            TopRight => (2, 0),
            CenterLeft => (0, 1),
            Center => (1, 1),
            CenterRight => (2, 1),
            BottomLeft => (0, 2),
            Bottom => (1, 2),
            BottomRight => (2, 2),
        }
    }
}

// Converts a 1D index (0 to 8) into the corresponding Move variant.
impl From<usize> for Move {
    fn from(value: usize) -> Self {
        use Move::*;
        match value {
            0 => TopLeft,
            1 => Top,
            2 => TopRight,
            3 => CenterLeft,
            4 => Center,
            5 => CenterRight,
            6 => BottomLeft,
            7 => Bottom,
            8 => BottomRight,
            _ => panic!("Out of bounds index for Tic-Tac-Toe move"),
        }
    }
}
