//! # Grid Handler
//!
//! A lightweight, stack-allocated, fixed-size 2D grid structure built for speed and simplicity.
//!
//! ## Quick Start
//!
//! ```rust
//! use grid_handler::grid::Grid;
//!
//! // Create a 3x3 grid initialized with zeroes
//! let mut grid = Grid::<3, 3, i32>::new(0);
//!
//! // Modify a cell using 2D indexing syntax
//! grid[(1, 1)] = 42;
//!
//! assert_eq!(grid[(1, 1)], 42);
//! ```
//!
//! ## Features
//!
//! - **Fixed-size & Stack Allocated:** Uses const generics for zero-heap allocation overhead.
//! - **Idiomatic Traits:** Supports standard indexing `grid[(x, y)]`, iteration, and slice conversions (`AsRef`).
//! - **Safe Out-of-Bounds Handling:** Provides methods like `.get_value_at()` returning `Result` or `Option`.

pub mod error;
pub mod grid;
