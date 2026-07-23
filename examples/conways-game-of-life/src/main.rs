use gridpx::Grid;
use macroquad::prelude::*;

// Configuration constants for the visual display and grid setup.
const GRID_SIZE: f32 = 30.0;
const WIDTH: usize = 50;
const HEIGHT: usize = 30;
const MARGIN: f32 = 30.0;
const BORDER_WIDTH: f32 = 2.0;
const BORDER_SPACING: f32 = GRID_SIZE + BORDER_WIDTH;
// Simulation update delay in seconds (4 updates per second).
const DELAY: f64 = 0.25;

/// Represents the status of each grid cell.
/// Implements `Clone` and `Copy` to allow grid initialization and cloning.
#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Life,  // Active / Alive cell
    Space, // Empty / Dead cell
}

/// Helper configuration to define window settings for Macroquad.
fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life (gridpx Example)".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

/// The main entry point of the simulation using macroquad's async main loop.
#[macroquad::main(window_conf)]
async fn main() {
    // Initialize a new grid of dimensions `WIDTH` x `HEIGHT` filled with dead cells (`Tile::Space`).
    // gridpx::Grid manages the contiguous data on the heap to avoid stack overflows.
    let mut grid: Grid<WIDTH, HEIGHT, _> = Grid::new(Tile::Space);

    // Spawn a Glider pattern in the top-left corner.
    // Gliders travel diagonally across the grid.
    grid[(1, 0)] = Tile::Life;
    grid[(2, 1)] = Tile::Life;
    grid[(0, 2)] = Tile::Life;
    grid[(1, 2)] = Tile::Life;
    grid[(2, 2)] = Tile::Life;

    // Spawn several "Blinker" or "Spinner" patterns (period 2 oscillators).
    spawn_spinner(&mut grid, 16, 17);
    spawn_spinner(&mut grid, 10, 20);
    spawn_spinner(&mut grid, 25, 10);

    // Track the time of the last simulation update.
    let mut last_update = get_time();

    loop {
        // 1. Draw the current state of the simulation.
        // Clear background with a dark blue color.
        clear_background(Color::from_hex(0x0B132B));
        // Draw the alive and dead cell blocks.
        draw_grid_contents(&grid);
        // Draw the light gray grid lines.
        draw_grid();

        // 2. Perform Conway's Game of Life simulation updates at set intervals (defined by `DELAY`).
        // Keeping simulation logic separate from the frame rate ensures smooth rendering.
        if get_time() - last_update > DELAY {
            last_update = get_time();

            // Clone the grid state to read from the previous step while modifying the current grid.
            let grid_copy = grid.clone();

            // Loop through each cell in the grid to apply Conway's rules.
            // `get_mut_rows` returns mutable chunks representing each row.
            for (y, row) in grid.get_mut_rows().enumerate() {
                for (x, tile) in row.iter_mut().enumerate() {
                    let neighbors_count = count_neighbors(&grid_copy, x, y);

                    // Apply the standard B3/S23 rules:
                    // - Birth: A dead cell with exactly 3 live neighbors becomes alive (B3).
                    // - Survival: A live cell with 2 or 3 live neighbors stays alive (S23).
                    // - Death: Any other live cell dies due to underpopulation or overpopulation.
                    match neighbors_count {
                        3 if *tile == Tile::Space => *tile = Tile::Life,
                        _ if *tile == Tile::Space => (),
                        0..=1 | 4.. => *tile = Tile::Space,
                        2..=3 if *tile == Tile::Life => (),
                        _ => (),
                    }
                }
            }
        }

        // 3. Yield execution back to Macroquad to swap frames and keep the window responsive.
        next_frame().await
    }
}

/// Counts the number of active `Tile::Life` neighbors surrounding a target cell coordinate.
/// Checks all 8 adjacent cells, ensuring bounds safety.
fn count_neighbors(grid: &Grid<WIDTH, HEIGHT, Tile>, x: usize, y: usize) -> usize {
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            // Do not count the target cell itself.
            if dr == 0 && dc == 0 {
                continue;
            }

            // Calculate neighbor coordinates using signed arithmetic to handle edge underflows safely.
            if let (Some(r), Some(c)) = (y.checked_add_signed(dr), x.checked_add_signed(dc)) {
                // Ensure coordinates fall within the grid limits before indexing.
                if r < HEIGHT && c < WIDTH && grid[(c, r)] == Tile::Life {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Draws individual grid cells using macroquad shapes.
fn draw_grid_contents(grid: &Grid<WIDTH, HEIGHT, Tile>) {
    let life_color = Color::from_hex(0xEF8354); // Warm orange for live cells
    let space_color = Color::from_hex(0x1C2541); // Dark navy for dead cells

    for (y, row) in grid.get_rows().enumerate() {
        // Calculate the physical Y position on the screen, factoring in border spacing, margins, and line width.
        let screen_y = y as f32 * BORDER_SPACING + MARGIN + BORDER_WIDTH / 2.0;

        for (x, content) in row.iter().enumerate() {
            // Calculate the physical X position on the screen.
            let screen_x = x as f32 * BORDER_SPACING + MARGIN + BORDER_WIDTH / 2.0;

            match content {
                Tile::Life => draw_rectangle(screen_x, screen_y, GRID_SIZE, GRID_SIZE, life_color),
                Tile::Space => {
                    draw_rectangle(screen_x, screen_y, GRID_SIZE, GRID_SIZE, space_color)
                }
            }
        }
    }
}

/// Draws the grid lines dividing the cells.
fn draw_grid() {
    let border_color = Color::from_hex(0xAAABBC); // Light gray for grid borders

    // Draw vertical grid lines.
    for x in 0..=WIDTH {
        let screen_x = x as f32 * BORDER_SPACING + MARGIN;
        draw_line(
            screen_x,
            MARGIN,
            screen_x,
            HEIGHT as f32 * BORDER_SPACING + MARGIN,
            BORDER_WIDTH,
            border_color,
        );
    }

    // Draw horizontal grid lines.
    for y in 0..=HEIGHT {
        let screen_y = y as f32 * BORDER_SPACING + MARGIN;
        draw_line(
            MARGIN,
            screen_y,
            WIDTH as f32 * BORDER_SPACING + MARGIN,
            screen_y,
            BORDER_WIDTH,
            border_color,
        );
    }
}

/// Spawns a horizontal Blinker/Spinner pattern (a line of 3 live cells) centered at `(x, y)`.
/// Oscillator transitions from horizontal line to vertical line back and forth every generation.
fn spawn_spinner(grid: &mut Grid<WIDTH, HEIGHT, Tile>, x: usize, y: usize) {
    let w_limit = WIDTH.saturating_sub(1);
    let h_limit = HEIGHT.saturating_sub(1);

    grid[(x.saturating_sub(1).min(w_limit), y.min(h_limit))] = Tile::Life;
    grid[(x.min(w_limit), y.min(h_limit))] = Tile::Life;
    grid[(w_limit.min(x + 1), y.min(h_limit))] = Tile::Life;
}
