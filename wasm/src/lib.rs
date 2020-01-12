mod utils;

use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate web_sys;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn get_state(cell: bool, live_neighbors: u8) -> bool {
    match (cell, live_neighbors) {
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
        (otherwise, _) => otherwise,
    }
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

fn random(threshold: f64) -> bool {
    js_sys::Math::random() < threshold
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for drow in [self.height - 1, 0, 1].iter().cloned() {
            for dcol in [self.width - 1, 0, 1].iter().cloned() {
                if drow == 0 && dcol == 0 {
                    continue;
                }

                let neighbor_row = (row + drow) % self.height;
                let neighbor_col = (col + dcol) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn reset(&mut self) {
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset();
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);
        Universe { width, height, cells }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.height {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = get_state(cell, live_neighbors);

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn fill_random(&mut self) {
        for i in 0..self.cells.len() {
            self.cells.set(i, random(0.5));
        }
    }

    pub fn create_glider(&mut self) {
        let glider_cells = [(0, 0), (1, 1), (2, 0), (2, 1), (1, 2)];
        self.set_cells(&glider_cells);
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
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

    pub fn clear(&mut self) {
        self.cells.clear();
    }
}
