# Ant Colony Simulation - Project Context

This project is a high-performance ant colony simulation built with **Rust** and the **Bevy** game engine. It simulates ant behavior including foraging, trail formation via pheromones, and colony management.

## Project Overview

*   **Technology Stack**:
    *   **Engine**: Bevy 0.11
    *   **UI**: `bevy_egui` for the simulation dashboard.
    *   **Spatial Indexing**: `kd-tree` for efficient pheromone gradient lookups.
    *   **Camera**: `bevy_pancam` for panning and zooming.
*   **Performance**: Capable of handling ~5,000 ants on the CPU by utilizing k-d trees for spatial queries and a query caching system for steering targets.
*   **Architecture**: Follows Bevy's ECS (Entity Component System) pattern, organized into modular plugins:
    *   `AntPlugin`: Ant logic, movement, and state transitions (FindFood <-> FindHome).
    *   `PheromonePlugin`: Management of pheromone grids, decay cycles, and k-d tree updates.
    *   `PathVizPlugin`: Aesthetic/debug visualization of ant movement history.
    *   `GuiPlugin`: Interactive settings and real-time statistics.

## Key Components

*   **`src/configs.rs`**: Centralized configuration. Modify this file to tune simulation parameters like ant speed, pheromone decay rates, and colony/food locations.
*   **`src/ant.rs`**: Core ant behavior. Ants drop "to-home" pheromones when searching for food and "to-food" pheromones when returning home.
*   **`src/grid.rs`**: Implements `WorldGrid` and `DecayGrid`. Uses a coordinate system optimized for pheromone density lookups.
*   **`src/pheromone.rs`**: Handles the lifecycle of pheromones, including emission, decay, and the underlying k-d tree maintenance.

## Building and Running

*   **Run Simulation**:
    ```bash
    cargo run --release
    ```
    *Note: Running in release mode is highly recommended due to the heavy computational load of 5,000 ants.*
*   **Build**:
    ```bash
    cargo build --release
    ```
*   **Testing**:
    ```bash
    cargo test
    ```
    *(Note: The project currently relies primarily on manual verification through the simulation GUI.)*

## Development Conventions

*   **Configuration**: Avoid hardcoding values in systems; always prefer adding a constant to `src/configs.rs`.
*   **Performance**: When adding spatial logic, ensure it integrates with the existing k-d tree or caching mechanisms to maintain high ant counts.
*   **UI**: Use the `SimSettings` and `SimStatistics` resources to communicate between simulation systems and the GUI.
*   **Shortcuts (In-App)**:
    *   `Tab`: Toggle Settings Menu
    *   `H`: Toggle Home Pheromones
    *   `F`: Toggle Food Pheromones
    *   `P`: Toggle Path Visualization
    *   `A`: Toggle Ant Visibility
    *   `Esc`: Close Simulation
