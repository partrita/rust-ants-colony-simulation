# Bolt's Performance Journal - Ant Colony Simulation

## 2026-03-21 - Parallelizing Ant Updates
**Learning:** In Bevy simulations with high entity counts (5000+), the per-frame movement and rotation updates are significant CPU consumers when executed sequentially. Parallelizing these via `par_iter_mut()` is a standard and effective optimization.
**Action:** Use `par_iter_mut()` for the `update_position` system and other high-frequency per-ant loops where possible.

## 2026-03-21 - Avoiding Redundant Math in Frame Loops
**Learning:** Systems that run every frame (like `update_position`) often perform redundant calculations like vector normalization and rotation updates even when the velocity hasn't changed.
**Action:** Implement checks (e.g., `acceleration != Vec2::ZERO`) to skip expensive math operations in the hot path.
