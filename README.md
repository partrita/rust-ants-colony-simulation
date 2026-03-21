# Ant Colony Simulation
This is an ant colony simulation, it internally uses kdtree and query caching, it's able to handle about 5k ants on the cpu.


Built with [Rust](https://www.rust-lang.org/) and [Bevy](https://bevyengine.org/) game engine

![screenshot](/screenshot.png)

# Demo
Here's the entire timelapse of the AI learning to drive

[![youtube](https://img.youtube.com/vi/98pUSZAM_7M/0.jpg)](https://youtu.be/98pUSZAM_7M)

# Timelapses with Approach 1
[![youtube](https://img.youtube.com/vi/5xdfTJBMnwI/0.jpg)](https://youtu.be/5xdfTJBMnwI)


## Usage
- Clone the repo
```bash
git clone git@github.com:bones-ai/rust-ants-colony-simulation.git
cd rust-ants-colony-simulation
```
- Run the simulation
```bash
cargo run --release
```
> **Note:** Running in release mode is highly recommended to maintain performance with 5,000 ants.

### Keyboard Shortcuts
- `Tab`: Toggle Settings/Stats Menu
- `H`: Toggle Home Pheromones visibility
- `F`: Toggle Food Pheromones visibility
- `P`: Toggle Ant Paths visualization
- `A`: Toggle Ant visibility
- `Esc`: Close the simulation

## Configurations
- The project config file is located at `src/configs.rs`
- If all ants aren't forming a single trail even after a long time, try increasing `ANT_INITIAL_PH_STRENGTH` in the configs to a greater value (example: `40.0`)
- You can also adjust `NUM_ANTS`, `ANT_SPEED`, and other simulation parameters in the same file.
