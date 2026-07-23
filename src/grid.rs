use crate::error::GridError;
use core::fmt;
use std::{
    ops::{Index, IndexMut},
    slice::Chunks,
};

/// A fixed-size, heap-allocated 2D grid.
///
/// `Grid` provides a contiguous, row-major memory layout. It uses const generics
/// to enforce dimensions (`W` and `H`) at compile time while protecting the stack
/// by forcing data onto the heap.
///
/// # Type Parameters
/// * `W` - The fixed width (number of columns) of the grid.
/// * `H` - The fixed height (number of rows) of the grid.
/// * `T` - The data type stored in each cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<const W: usize, const H: usize, T: Clone>(Box<[T]>);

impl<const W: usize, const H: usize, T: Clone> Grid<W, H, T> {
    /// Creates a new `Grid` filled entirely with a cloned default value.
    ///
    /// # Performance
    /// Allocates exactly `W * H * size_of::<T>()` bytes on the heap. The `default_value`
    /// is cloned `(W * H) - 1` times to fill the space.
    ///
    /// # Examples
    /// ```
    /// use gridpx::grid::Grid;
    ///
    /// let grid = Grid::<3, 2, i32>::new(7);
    /// assert_eq!(grid.get_value_at(0, 0), Ok(&7));
    /// ```
    pub fn new(default_value: T) -> Self {
        let data = vec![default_value; W * H].into_boxed_slice();

        Self(data)
    }

    /// Retrieves a shared reference to the value at the specified coordinates.
    ///
    /// Coordinates are zero-indexed, where `(0, 0)` represents the top-left corner.
    ///
    /// # Arguments
    /// * `x` - The column index (horizontal axis).
    /// * `y` - The row index (vertical axis).
    ///
    /// # Errors
    /// Returns [`GridError::OutOfBounds`] if `x >= W` or `y >= H`.
    ///
    /// # Examples
    /// ```
    /// use gridpx::grid::Grid;
    /// let grid = Grid::<2, 2, &str>::new("empty");
    ///
    /// // Successful lookup
    /// assert_eq!(grid.get_value_at(1, 0), Ok(&"empty"));
    ///
    /// // Out of bounds lookup
    /// assert!(grid.get_value_at(2, 0).is_err());
    /// ```
    pub fn get_value_at(&self, x: usize, y: usize) -> Result<&T, GridError> {
        if x >= W || y >= H {
            return Err(GridError::OutOfBounds {
                x,
                y,
                width: W,
                height: H,
            });
        }

        let unit_index = y * W + x;
        Ok(&self.0[unit_index])
    }

    /// Overwrites the value at the specified coordinates.
    ///
    /// # Arguments
    /// * `x` - The column index (horizontal axis).
    /// * `y` - The row index (vertical axis).
    /// * `value` - The new data to insert into the cell.
    ///
    /// # Errors
    /// Returns [`GridError::OutOfBounds`] if the target coordinates exceed the grid boundaries.
    ///
    /// # Examples
    /// ```
    /// use gridpx::grid::Grid;
    /// let mut grid = Grid::<3, 3, i32>::new(0);
    ///
    /// grid.set_value_at(1, 2, 42).unwrap();
    /// assert_eq!(grid.get_value_at(1, 2), Ok(&42));
    /// ```
    pub fn set_value_at(&mut self, x: usize, y: usize, value: T) -> Result<(), GridError> {
        if x >= W || y >= H {
            return Err(GridError::OutOfBounds {
                x,
                y,
                width: W,
                height: H,
            });
        }

        let unit_index = y * W + x;
        self.0[unit_index] = value;

        Ok(())
    }

    /// Returns chunks with size of `W`.
    ///
    /// # Examples
    /// ```
    /// use gridpx::grid::Grid;
    /// let mut grid = Grid::<3, 3, i32>::new(0);
    ///
    /// let mut chunks = grid.get_rows();
    ///
    /// assert_eq!(chunks.next(), Some(&[0, 0, 0][..]));
    /// assert_eq!(chunks.next(), Some(&[0, 0, 0][..]));
    /// assert_eq!(chunks.next(), Some(&[0, 0, 0][..]));
    /// assert_eq!(chunks.next(), None);
    /// ```
    pub fn get_rows<'a>(&'a self) -> Chunks<'a, T> {
        self.0.chunks(W)
    }

    /// Returns a slice from the internal array.
    ///
    /// # Examples
    /// ```
    /// use gridpx::grid::Grid;
    /// let grid = Grid::<3, 3, i32>::new(0);
    ///
    /// let array = grid.as_slice();
    ///
    /// assert_eq!(array, &[0; 9]);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Fills the entire grid with `value`.
    ///
    /// # Arguments
    /// * `value` - the value used to fill the grid.
    ///
    /// # Examples
    /// ```
    /// use gridpx::grid::Grid;
    /// let mut grid = Grid::<3, 3, i32>::new(0);
    ///
    /// grid.fill(1);
    ///
    /// let array = grid.as_slice();
    ///
    /// assert_eq!(array, &[1; 9]);
    /// ```
    pub fn fill(&mut self, value: T) {
        self.0.fill(value);
    }

    /// Overwrites the grid's contents with the elements of the given slice.
    ///
    /// The grid's existing elements are updated by cloning from the input slice,
    /// reusing the existing heap allocation to avoid new memory allocations.
    ///
    /// # Arguments
    /// * `array` - A slice of elements to copy into the grid.
    ///
    /// # Panics
    /// Panics if the length of the slice is not equal to `W * H` (the grid size).
    ///
    /// # Examples
    /// ```
    /// use gridpx::grid::Grid;
    /// let mut grid = Grid::<2, 2, i32>::new(0);
    ///
    /// grid.set_array(&[1, 2, 3, 4]);
    ///
    /// assert_eq!(grid.as_slice(), &[1, 2, 3, 4]);
    /// ```
    pub fn set_array(&mut self, array: &[T]) {
        assert_eq!(array.len(), W * H, "array length must match grid size");
        self.0.clone_from_slice(array);
    }
}

impl<const W: usize, const H: usize, T: Clone> Index<(usize, usize)> for Grid<W, H, T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.get_value_at(x, y).expect("grid index out of bounds")
    }
}

impl<const W: usize, const H: usize, T: Clone> IndexMut<(usize, usize)> for Grid<W, H, T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x >= W || y >= H {
            panic!("grid index out of bounds");
        }
        &mut self.0[y * W + x]
    }
}

impl<const W: usize, const H: usize, T: Clone> Index<usize> for Grid<W, H, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const W: usize, const H: usize, T: Clone> IndexMut<usize> for Grid<W, H, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const W: usize, const H: usize, T: Clone> AsRef<[T]> for Grid<W, H, T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<const W: usize, const H: usize, T: Clone> AsMut<[T]> for Grid<W, H, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<'a, const W: usize, const H: usize, T: Clone> IntoIterator for &'a Grid<W, H, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, const W: usize, const H: usize, T: Clone> IntoIterator for &'a mut Grid<W, H, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<const W: usize, const H: usize, T: Clone + fmt::Display> fmt::Display for Grid<W, H, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.get_rows() {
            for cell in row {
                write!(f, "{cell} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize, T: Clone> From<&[T]> for Grid<W, H, T> {
    fn from(value: &[T]) -> Self {
        assert_eq!(value.len(), W * H, "array length must match grid size");
        Self(value.to_vec().into_boxed_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_indexing() {
        let mut grid = Grid::<3, 2, i32>::new(0);

        // Ensure y * W + x places value at the correct flat index
        grid[(2, 1)] = 99;
        assert_eq!(grid.as_slice()[5], 99);
    }

    #[test]
    fn test_out_of_bounds_error_payload() {
        let grid = Grid::<2, 2, i32>::new(0);
        let err = grid.get_value_at(2, 0).unwrap_err();

        assert_eq!(
            err,
            GridError::OutOfBounds {
                x: 2,
                y: 0,
                width: 2,
                height: 2
            }
        );
    }
}
