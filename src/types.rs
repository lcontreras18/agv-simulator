pub const GRID_WIDTH: usize = 40;
pub const GRID_HEIGHT: usize = 40;
pub const CELL_SIZE: u32 = 16;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridCell {
    pub col: usize,
    pub row: usize,
}

impl GridCell {
    pub fn from_position(pos: Position) -> Self {
        GridCell {
            col: (pos.x as usize).min(GRID_WIDTH - 1),
            row: (pos.y as usize).min(GRID_HEIGHT - 1),
        }
    }

    pub fn to_position(self) -> Position {
        Position {
            x: self.col as f32 + 0.5,
            y: self.row as f32 + 0.5,
        }
    }

    pub fn neighbors(self) -> Vec<GridCell> {
        let mut n = Vec::new();
        let (col, row) = (self.col as isize, self.row as isize);
        for (dc, dr) in &[(-1,0),(1,0),(0,-1),(0,1)] {
            let nc = col + dc;
            let nr = row + dr;
            if nc >= 0 && nc < GRID_WIDTH as isize && nr >= 0 && nr < GRID_HEIGHT as isize {
                n.push(GridCell { col: nc as usize, row: nr as usize });
            }
        }
        n
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VehicleState {
    Idle,
    Scanning,
    Navigating,
    ObstacleAvoiding,
    GoalReached,
}
