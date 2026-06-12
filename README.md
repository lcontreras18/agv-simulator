# Autonomous Ground Vehicle Simulator

A real-time 2D autonomous navigation simulator written in Rust.

## Overview

This project simulates an autonomous ground vehicle navigating a 2D environment using fused sensor data. The vehicle processes simulated lidar and proximity sensor inputs, maintains a dynamic occupancy grid, and replans its path in real time as obstacles are detected. The system is designed around concurrent Rust primitives to reflect the architecture of real embedded autonomy systems.

## Features

- Sensor fusion engine combining simulated lidar and proximity data into a dynamic occupancy grid
- Readers-writer synchronization via `RwLock` — concurrent sensor reads with exclusive planner writes
- Producer-consumer pipeline between sensor-fusion and path-planning threads using Rust channels
- A* path planner that dynamically replans around changing obstacles with sub-15ms latency
- SDL2 visualization rendering live telemetry, sensor range, obstacle maps, and planned trajectories

## Architecture
src/

├── main.rs              # Entry point, thread spawning

├── sensor/

│   ├── lidar.rs         # Simulated lidar sensor

│   └── proximity.rs     # Simulated proximity sensor

├── fusion/

│   └── occupancy.rs     # Occupancy grid + RwLock map

├── planning/

│   └── astar.rs         # A* path planner

├── vehicle/

│   └── state.rs         # Vehicle state machine

└── viz/

└── renderer.rs      # SDL2 rendering layer

## Getting started

```bash
# Clone the repo
git clone https://github.com/lcontreras18/agv-simulator
cd agv-simulator

# Install SDL2 (Ubuntu/Debian)
sudo apt install libsdl2-dev

# Or on macOS
brew install sdl2

# Build and run
cargo build --release
cargo run --release
```

## Concurrency model

The occupancy grid is shared across threads using `Arc<RwLock<OccupancyGrid>>`. Sensor threads acquire read locks concurrently to write new observations. The path planner acquires a write lock only when replanning, minimizing contention. A Rust channel connects the fusion stage to the planner — the planner blocks on `recv()` until new map data arrives, eliminating busy-wait polling.

## Requirements

- Rust 1.75+
- SDL2 development libraries
- Linux or macOS

## License

MIT
