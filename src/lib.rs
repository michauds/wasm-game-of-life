mod utils;

extern crate fixedbitset;
extern crate js_sys;

use fixedbitset::FixedBitSet;
use std::fmt;
use js_sys::Math;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello, {}!", name).as_str());
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(in_width: u32, in_height: u32) -> Universe {
        let width = in_width;
        let height = in_height;

        /*
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        */
        let universe_size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(universe_size);

        for i in 0..universe_size {
            if Math::round(Math::random()) == 1.0 {
                cells.set(i, true)
            } else {
                cells.set(i, false)
            }
        };
        /*
        let cells = (0..width * height)
            .map(|i| {
                if Math::round(Math::random()) == 1.0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        */

        let mut universe = Universe{
            width,
            height,
            cells
        };

        universe.create_spaceship();

        universe
    }

    fn create_spaceship(&mut self) {
        /*
        smallest glider:

        0 1 0
        0 0 1
        1 1 1

        since memory is linear in wasm
        */

        let mut next = self.cells.clone();

        /*
        next[self.get_index(0, 0)] = Cell::Dead;
        next[self.get_index(0, 1)] = Cell::Alive;
        next[self.get_index(0, 2)] = Cell::Dead;
        next[self.get_index(1, 0)] = Cell::Dead;
        next[self.get_index(1, 1)] = Cell::Dead;
        next[self.get_index(1, 2)] = Cell::Alive;
        next[self.get_index(2, 0)] = Cell::Alive;
        next[self.get_index(2, 1)] = Cell::Alive;
        next[self.get_index(2,2 )] = Cell::Alive;
        */


        next.set(self.get_index(0, 0), false);
        next.set(self.get_index(0, 1), true);
        next.set(self.get_index(0, 2), false);
        next.set(self.get_index(1, 0), false);
        next.set(self.get_index(1, 1), false);
        next.set(self.get_index(1, 2), true);
        next.set(self.get_index(2, 0), true);
        next.set(self.get_index(2, 1), true);
        next.set(self.get_index(2, 2), true);

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        if Math::random() > 0.9 {
            self.create_spaceship()
        }

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                /*
                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
                */

                next.set(idx, match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol);
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
