use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::types::{GridCell, GRID_WIDTH, GRID_HEIGHT};
use crate::fusion::occupancy::OccupancyGrid;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    cell: GridCell,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: GridCell, b: GridCell) -> u32 {
    let dx = (a.col as isize - b.col as isize).unsigned_abs() as u32;
    let dy = (a.row as isize - b.row as isize).unsigned_abs() as u32;
    dx + dy
}

pub fn plan(start: GridCell, goal: GridCell, grid: &OccupancyGrid) -> Option<Vec<GridCell>> {
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), u32> = HashMap::new();

    let start_key = (start.col, start.row);
    g_score.insert(start_key, 0);
    open.push(Node { cost: heuristic(start, goal), cell: start });

    while let Some(Node { cell: current, .. }) = open.pop() {
        if current == goal {
            let mut path = vec![goal];
            let mut cur_key = (goal.col, goal.row);
            while let Some(&prev) = came_from.get(&cur_key) {
                path.push(GridCell { col: prev.0, row: prev.1 });
                cur_key = prev;
            }
            path.reverse();
            return Some(path);
        }

        for neighbor in current.neighbors() {
            if grid.is_occupied(neighbor) { continue; }
            let cur_key = (current.col, current.row);
            let nb_key = (neighbor.col, neighbor.row);
            let tentative = g_score.get(&cur_key).unwrap_or(&u32::MAX).saturating_add(1);
            if tentative < *g_score.get(&nb_key).unwrap_or(&u32::MAX) {
                came_from.insert(nb_key, cur_key);
                g_score.insert(nb_key, tentative);
                let f = tentative + heuristic(neighbor, goal);
                open.push(Node { cost: f, cell: neighbor });
            }
        }
    }
    None
}
