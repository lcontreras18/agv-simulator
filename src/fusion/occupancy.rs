use crate::types::{GRID_WIDTH, GRID_HEIGHT, GridCell};

#[derive(Clone)]
pub struct OccupancyGrid {
    cells: [[f32; GRID_WIDTH]; GRID_HEIGHT],
}

impl OccupancyGrid {
    pub fn new() -> Self {
        OccupancyGrid {
            cells: [[0.0; GRID_WIDTH]; GRID_HEIGHT],
        }
    }

    pub fn update(&mut self, cell: GridCell, probability: f32) {
        self.cells[cell.row][cell.col] = probability.clamp(0.0, 1.0);
    }

    pub fn is_occupied(&self, cell: GridCell) -> bool {
        self.cells[cell.row][cell.col] > 0.5
    }

    pub fn get(&self, cell: GridCell) -> f32 {
        self.cells[cell.row][cell.col]
    }

    pub fn decay(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell *= 0.95;
            }
        }
    }
}
