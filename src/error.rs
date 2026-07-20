#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum GridError {
    #[error("Grid index out of bounds: tried to access coordinate ({x}, {y}) on a grid with dimensions ({width}, {height})")]
    OutOfBounds {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    },
}
