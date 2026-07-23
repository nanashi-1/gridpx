# Conway's Game of Life Example

This is a complete, interactive simulation of **Conway's Game of Life** built in Rust. It serves as an example of how to use the [`gridpx`](https://github.com/nanashi-1/gridpx) crate—a lightweight, heap-allocated, fixed-size 2D grid structure—alongside the [macroquad](https://macroquad.rs/) game library for high-performance rendering.

---

## 📖 How It Works

Conway's Game of Life is a zero-player cellular automaton played on a 2D grid. The state of the grid evolves in discrete steps (generations) based on a simple set of rules applied to each cell and its 8 immediate neighbors:

1. **Underpopulation:** Any live cell with fewer than two live neighbors dies.
2. **Survival:** Any live cell with two or three live neighbors lives on to the next generation.
3. **Overpopulation:** Any live cell with more than three live neighbors dies.
4. **Birth:** Any dead cell with exactly three live neighbors becomes a live cell.

These rules are often abbreviated as **B3/S23** (Birth on 3, Survive on 2 or 3).

---

## 🛠️ Code Structure & Implementation Details

The implementation is located in [`src/main.rs`](file:///home/nanashi/mnt/home_bkp/Documents/Rust/gridpx/examples/conways-game-of-life/src/main.rs) and consists of the following key components:

### 1. The Grid Representation (`gridpx::Grid`)
Instead of using nested vectors (`Vec<Vec<T>>`), which can cause cache-locality issues and multiple heap allocations, this example uses:
```rust
let mut grid: Grid<WIDTH, HEIGHT, Tile> = Grid::new(Tile::Space);
```
- **Heap Allocation:** Data is stored contiguously on the heap using a boxed slice (`Box<[T]>`), minimizing allocation overhead and maximizing memory cache friendliness.
- **Const Generics:** Grid width (`WIDTH`) and height (`HEIGHT`) are defined as compile-time constants.
- **Tupled 2D Indexing:** Individual cells are accessed cleanly using the `(x, y)` coordinate pattern via `Index` and `IndexMut` implementations (e.g., `grid[(col, row)]`).

### 2. The Main Simulation Loop
The game runs inside Macroquad's async game loop. To keep the visual frame rate smooth and independent of the simulation updates, we decouple rendering from the physics calculations:
- The screen clears, grid lines, and active cells are drawn **every frame**.
- The cellular automaton state is computed only when the elapsed time exceeds `DELAY` (0.25 seconds).
- The grid is cloned (`let grid_copy = grid.clone();`) at the start of each generation so we can check neighbors against the previous state while writing the new state to the main grid.

### 3. Edge Safety & Boundary Checks
The coordinates for cell indexing are bounds-safe. The `count_neighbors` function uses signed additions via `.checked_add_signed()` to prevent integer underflows/overflows at the boundaries of the grid. Initial setups like `spawn_spinner` use `.saturating_sub()` and `.min()` to guarantee no out-of-bounds index panics occur.

---

## 🚀 How to Run

Ensure you have Rust and Cargo installed. Then run the example from the root directory of the workspace:

```bash
cargo run -p conways-game-of-life
```

Or run directly from the example folder:
```bash
cd examples/conways-game-of-life
cargo run
```
