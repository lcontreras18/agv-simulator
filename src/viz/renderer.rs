use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::types::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, GridCell, VehicleState};
use crate::fusion::occupancy::OccupancyGrid;
use crate::vehicle::state::Vehicle;

pub fn draw(
    canvas: &mut Canvas<Window>,
    grid: &OccupancyGrid,
    vehicle: &Vehicle,
    obstacles: &[[bool; GRID_WIDTH]; GRID_HEIGHT],
) {
    canvas.set_draw_color(Color::RGB(20, 20, 30));
    canvas.clear();

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let cell = GridCell { row, col };
            let rect = Rect::new(
                (col as u32 * CELL_SIZE) as i32,
                (row as u32 * CELL_SIZE) as i32,
                CELL_SIZE,
                CELL_SIZE,
            );

            if obstacles[row][col] {
                canvas.set_draw_color(Color::RGB(180, 60, 60));
                canvas.fill_rect(rect).unwrap();
            } else {
                let occ = grid.get(cell);
                let intensity = (occ * 180.0) as u8;
                canvas.set_draw_color(Color::RGB(intensity, intensity / 2, intensity / 3));
                canvas.fill_rect(rect).unwrap();
            }

            canvas.set_draw_color(Color::RGB(40, 40, 50));
            canvas.draw_rect(rect).unwrap();
        }
    }

    for i in 1..vehicle.path.len() {
        let a = vehicle.path[i - 1];
        let b = vehicle.path[i];
        canvas.set_draw_color(Color::RGB(80, 180, 255));
        let _ = canvas.draw_line(
            (
                (a.col as u32 * CELL_SIZE + CELL_SIZE / 2) as i32,
                (a.row as u32 * CELL_SIZE + CELL_SIZE / 2) as i32,
            ),
            (
                (b.col as u32 * CELL_SIZE + CELL_SIZE / 2) as i32,
                (b.row as u32 * CELL_SIZE + CELL_SIZE / 2) as i32,
            ),
        );
    }

    let goal_rect = Rect::new(
        (vehicle.goal.col as u32 * CELL_SIZE + 2) as i32,
        (vehicle.goal.row as u32 * CELL_SIZE + 2) as i32,
        CELL_SIZE - 4,
        CELL_SIZE - 4,
    );
    canvas.set_draw_color(Color::RGB(80, 255, 120));
    canvas.fill_rect(goal_rect).unwrap();

    let vehicle_cell = GridCell::from_position(vehicle.position);
    let vehicle_rect = Rect::new(
        (vehicle_cell.col as u32 * CELL_SIZE + 2) as i32,
        (vehicle_cell.row as u32 * CELL_SIZE + 2) as i32,
        CELL_SIZE - 4,
        CELL_SIZE - 4,
    );
    let vehicle_color = match vehicle.state {
        VehicleState::Navigating => Color::RGB(255, 220, 50),
        VehicleState::Scanning => Color::RGB(50, 180, 255),
        VehicleState::ObstacleAvoiding => Color::RGB(255, 120, 50),
        VehicleState::GoalReached => Color::RGB(80, 255, 120),
        VehicleState::Idle => Color::RGB(150, 150, 150),
    };
    canvas.set_draw_color(vehicle_color);
    canvas.fill_rect(vehicle_rect).unwrap();

    canvas.present();
}
