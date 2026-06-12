use crate::types::{Position, GridCell, GRID_WIDTH, GRID_HEIGHT};
use crate::fusion::occupancy::OccupancyGrid;
use rand::Rng;

pub const LIDAR_RANGE: usize = 6;

pub fn scan(pos: Position, grid: &mut OccupancyGrid, obstacles: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {
    let mut rng = rand::thread_rng();
    let origin = GridCell::from_position(pos);

    for dr in -(LIDAR_RANGE as isize)..=(LIDAR_RANGE as isize) {
        for dc in -(LIDAR_RANGE as isize)..=(LIDAR_RANGE as isize) {
            let row = origin.row as isize + dr;
            let col = origin.col as isize + dc;
            if row < 0 || row >= GRID_HEIGHT as isize || col < 0 || col >= GRID_WIDTH as isize {
                continue;
            }
            let cell = GridCell { row: row as usize, col: col as usize };
            let dist = ((dr * dr + dc * dc) as f32).sqrt();
            if dist > LIDAR_RANGE as f32 { continue; }

            let noise: f32 = rng.gen_range(-0.05..0.05);
            if obstacles[cell.row][cell.col] {
                grid.update(cell, 0.9 + noise);
            } else {
                grid.update(cell, (0.05 + noise).max(0.0));
            }
        }
    }
}
