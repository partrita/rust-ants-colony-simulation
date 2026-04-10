// 🛡️ Security Enhancement: Prevent memory safety vulnerabilities by forbidding unsafe code.
#![forbid(unsafe_code)]
// 🛡️ Security Enhancement: Prevent DoS by forbidding panics.
#![forbid(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub mod ant;
pub mod configs;
pub mod food;
pub mod grid;
pub mod gui;
pub mod pathviz;
pub mod pheromone;
pub mod utils;

pub use configs::*;

use bevy::prelude::Event;

#[derive(Event)]
pub struct ResetSimulationEvent;
#[cfg(test)]
mod tests {
    use crate::utils::find_n_points_with_max_z;

    #[test]
    fn test_find_n_points_with_max_z_empty() {
        let mut points = vec![];
        find_n_points_with_max_z(&mut points, 5);
    }
}
