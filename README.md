# gridpx

[![Crates.io](https://img.shields.io/crates/v/gridpx.svg)](https://crates.io/crates/gridpx)
[![Documentation](https://docs.rs/gridpx/badge.svg)](https://docs.rs/gridpx)
[![License](https://img.shields.io/crates/l/gridpx.svg)](#license)

A lightweight, stack-allocated, fixed-size 2D grid structure for Rust, built with const generics for zero-heap allocation overhead.

## Features

- 🚀 **Zero-Heap Allocation:** Uses Rust const generics (`Grid<W, H, T>`) to keep data stack-allocated for speed.
- 🎯 **Idiomatic Indexing:** Supports standard 2D tuple indexing like `grid[(x, y)]`.
- 🛡️ **Safe & Fallible Access:** Offers checked accessors (`get_value_at`) alongside panic-free error handling via `GridError`.
- 🔄 **Iteration & Slices:** Easy conversion to slices and row/column iterators.

## Installation

Add `gridpx` to your `Cargo.toml`:

```bash
cargo add gridpx
```

## Quick Start

```rust

use gridpx::Grid;

fn main() {
    // Create a 3x3 grid initialized with zeroes
    let mut grid = Grid::<3, 3, i32>::new(0);

    // Set values using tuple indexing
    grid[(1, 1)] = 42;

    // Read values safely
    assert_eq!(grid[(1, 1)], 42);

    // Fallible access
    match grid.get_value_at(0, 0) {
        Ok(value) => println!("Value at (0, 0): {value}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
