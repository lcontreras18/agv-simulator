use crate::types::{Position, GridCell, VehicleState};
use crate::fusion::occupancy::OccupancyGrid;
use crate::planning::astar;

pub struct Vehicle {
    pub position: Position,
    pub goal: GridCell,
    pub state: VehicleState,
    pub path: Vec<GridCell>,
    pub path_index: usize,
    pub scan_ticks: u32,
}

impl Vehicle {
    pub fn new(start: Position, goal: GridCell) -> Self {
        Vehicle {
            position: start,
            goal,
            state: VehicleState::Scanning,
            path: Vec::new(),
            path_index: 0,
            scan_ticks: 0,
        }
    }

    pub fn update(&mut self, grid: &OccupancyGrid) {
        match self.state {
            VehicleState::Idle => {
                self.state = VehicleState::Scanning;
            }

            VehicleState::Scanning => {
                self.scan_ticks += 1;
                if self.scan_ticks >= 3 {
                    self.scan_ticks = 0;
                    self.replan(grid);
                }
            }

            VehicleState::Navigating => {
                if self.path_index >= self.path.len() {
                    self.state = VehicleState::GoalReached;
                    return;
                }

                let next_cell = self.path[self.path_index];
                let next_pos = next_cell.to_position();

                if grid.is_occupied(next_cell) {
                    self.state = VehicleState::ObstacleAvoiding;
                    return;
                }

                self.position = next_pos;
                self.path_index += 1;

                let current_cell = GridCell::from_position(self.position);
                if current_cell == self.goal {
                    self.state = VehicleState::GoalReached;
                }
            }

            VehicleState::ObstacleAvoiding => {
                self.replan(grid);
            }

            VehicleState::GoalReached => {}
        }
    }

    fn replan(&mut self, grid: &OccupancyGrid) {
        let start = GridCell::from_position(self.position);
        match astar::plan(start, self.goal, grid) {
            Some(path) => {
                self.path = path;
                self.path_index = 1;
                self.state = VehicleState::Navigating;
            }
            None => {
                self.state = VehicleState::Scanning;
            }
        }
    }
}
