mod types;
mod sensor;
mod fusion;
mod planning;
mod vehicle;
mod viz;

use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use types::{Position, GridCell, GRID_WIDTH, GRID_HEIGHT, CELL_SIZE};
use fusion::occupancy::OccupancyGrid;
use vehicle::state::Vehicle;

fn generate_obstacles() -> [[bool; GRID_WIDTH]; GRID_HEIGHT] {
    let mut obstacles = [[false; GRID_WIDTH]; GRID_HEIGHT];
    let walls = [
        (5, 5, 5, 15),
        (10, 2, 20, 2),
        (15, 8, 15, 20),
        (25, 5, 25, 18),
        (8, 25, 20, 25),
        (30, 20, 38, 20),
        (20, 30, 20, 38),
    ];
    for (c1, r1, c2, r2) in walls.iter() {
        let (rc1, cc1) = (r1.min(r2), c1.min(c2));
        let (rc2, cc2) = (r1.max(r2), c1.max(c2));
        for r in *rc1..=*rc2 {
            for c in *cc1..=*cc2 {
                if r < GRID_HEIGHT && c < GRID_WIDTH {
                    obstacles[r][c] = true;
                }
            }
        }
    }
    obstacles
}

fn prepopulate_grid(grid: &mut OccupancyGrid, obstacles: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) {
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            if obstacles[row][col] {
                grid.update(types::GridCell { row, col }, 1.0);
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let window = video
        .window(
            "AGV Simulator",
            GRID_WIDTH as u32 * CELL_SIZE,
            GRID_HEIGHT as u32 * CELL_SIZE,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let obstacles = Arc::new(generate_obstacles());

    let mut initial_grid = OccupancyGrid::new();
    prepopulate_grid(&mut initial_grid, &obstacles);
    let grid = Arc::new(RwLock::new(initial_grid));

    let (tx, rx) = mpsc::channel::<()>();

    let grid_sensor = Arc::clone(&grid);
    let obstacles_sensor = Arc::clone(&obstacles);
    thread::spawn(move || {
        let mut pos = Position { x: 1.5, y: 1.5 };
        loop {
            if rx.try_recv().is_ok() {
                break;
            }
            {
                let mut g = grid_sensor.write().unwrap();
                sensor::lidar::scan(pos, &mut g, &obstacles_sensor);
                sensor::proximity::scan(pos, &mut g, &obstacles_sensor);
            }
            pos.x = (pos.x + 0.1).min(GRID_WIDTH as f32 - 1.0);
            thread::sleep(Duration::from_millis(50));
        }
    });

    let start = Position { x: 1.5, y: 1.5 };
    let goal = GridCell { col: GRID_WIDTH - 3, row: GRID_HEIGHT - 3 };
    let mut vehicle = Vehicle::new(start, goal);

    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    let _ = tx.send(());
                    break 'running;
                }
                _ => {}
            }
        }

        {
            let g = grid.read().unwrap();
            vehicle.update(&g);
        }

        {
            let g = grid.read().unwrap();
            viz::renderer::draw(&mut canvas, &g, &vehicle, &obstacles);
        }

        thread::sleep(Duration::from_millis(150));
    }
}
