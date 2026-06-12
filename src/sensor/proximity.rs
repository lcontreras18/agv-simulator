use crate::types::{Position, GridCell, GRID_WIDTH, GRID_HEIGHT};
use crate::fusion::occupancy::OccupancyGrid;

pub const PROXIMITY_RANGE: usize = 2;

pub fn scan(pos: Position, grid: &mut OccupancyGrid, obstacles: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {
    let origin = GridCell::from_position(pos);

    for dr in -(PROXIMITY_RANGE as isize)..=(PROXIMITY_RANGE as isize) {
        for dc in -(PROXIMITY_RANGE as isize)..=(PROXIMITY_RANGE as isize) {
            let row = origin.row as isize + dr;
            let col = origin.col as isize + dc;
            if row < 0 || row >= GRID_HEIGHT as isize || col < 0 || col >= GRID_WIDTH as isize {
                continue;
            }
            let cell = GridCell { row: row as usize, col: col as usize };
            if obstacles[cell.row][cell.col] {
                grid.update(cell, 1.0);
            }
        }
    }
}
