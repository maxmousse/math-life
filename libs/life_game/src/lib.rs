use std::fmt;

use wasm_bindgen::prelude::*;

use utils::{log, set_panic_hook};

mod utils;

/// Represents a Cell of the game of life universe
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

/// Represents the universe of a game of life
#[wasm_bindgen]
pub struct Universe {
    tick_count: u32,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Cell {
    /// Toggle the state of a cell instance
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

#[wasm_bindgen]
impl Universe {
    /// Instantiate a new empty universe
    pub fn new(width: u32, height: u32) -> Universe {
        // Install panic hook
        set_panic_hook();

        Universe {
            tick_count: 0,
            width,
            height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    /// Init universe with an interesting template
    pub fn init(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
    }

    /// Basic version of rendering where
    /// the universe is rendered as a string
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Move the universe to its next state by
    /// calculating the next generation of cells
    pub fn tick(&mut self) {
        self.tick_count += 1;

        log!("Current tick: {}", self.tick_count);

        let mut next_universe = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let cell_index = self.get_cell_index(row, column);
                let current_cell = self.cells[cell_index];
                let alive_neighbors = self.count_alive_neighbors_of_cell(row, column);

                let next_cell = match (current_cell, alive_neighbors) {
                    (Cell::Alive, x) if !(2..=3).contains(&x) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (no_change, _) => no_change,
                };

                next_universe[cell_index] = next_cell;
            }
        }

        self.cells = next_universe;
    }

    /// Given the position of a cell in the universe, return
    /// its index in the storage vector
    fn get_cell_index(&self, row: u32, column: u32) -> usize {
        ((row * self.width) + column) as usize
    }

    /// Given the position of a cell, return the number of alive neighbors
    fn count_alive_neighbors_of_cell(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // Note: the two loops are equivalent to applying a mask around
        // the target cell to select the neighbor cells
        //       -1      0      1
        // -1   -1,-1   -1,0   -1,1
        // 0    0,-1    0,0    0,1
        // 1    1,-1    1,0    1,1
        // Modulo are used to avoid using if to handle edges of the universe
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_cell_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    /// Toggle the state of a cell in the universe
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_cell_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

/// Methods needed for test (that should not be ported to js/ts by wasm-bindgen)
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells_alive(&mut self, cells: Vec<(u32, u32)>) {
        for (row, column) in cells {
            let cell_index = self.get_cell_index(row, column);
            self.cells[cell_index] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
