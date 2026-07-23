# Tic-Tac-Toe Example

A command-line Tic-Tac-Toe game that demonstrates the usage of the [`gridpx`](https://crates.io/crates/gridpx) library.

The game allows you to play as either **X** or **O** against an unbeatable AI player powered by the **Minimax algorithm**.

## Demonstration of `gridpx`

This example showcases how to use `gridpx::Grid` to model a 2D board structure:
- Initializing a `Grid` using `Grid::new(Value::None)` with fixed const dimensions `<3, 3>`.
- Setting board cell values with `.set_value_at(x, y, value)`.
- Getting board cell values with `.get_value_at(x, y)` or using 1D/2D indexing operators (e.g., `grid[index]`).

## Project Structure

- [`src/main.rs`](file:///home/nanashi/mnt/home_bkp/Documents/Rust/gridpx/examples/tic-tac-toe/src/main.rs): Entrypoint containing game loops, user input, and turn management.
- [`src/board.rs`](file:///home/nanashi/mnt/home_bkp/Documents/Rust/gridpx/examples/tic-tac-toe/src/board.rs): Models the 3x3 game board, handles cell placement, and evaluates win/draw conditions.
- [`src/moves.rs`](file:///home/nanashi/mnt/home_bkp/Documents/Rust/gridpx/examples/tic-tac-toe/src/moves.rs): Contains the `Move` enum representing cell locations, and helper conversions.
- [`src/minimax.rs`](file:///home/nanashi/mnt/home_bkp/Documents/Rust/gridpx/examples/tic-tac-toe/src/minimax.rs): Implements the Minimax pathfinding AI to make the computer unbeatable.

## Running the Example

Run the following command from the workspace root or the project folder:

```bash
cargo run --bin tic-tac-toe
```
