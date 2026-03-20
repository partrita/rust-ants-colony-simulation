## 2024-03-20 - Bounds Checking Prevention in Image Grids
**Vulnerability:** Out-of-bounds coordinates generated in logic causing potential aliasing and unintended pixel manipulation when converting from world coordinates to image buffer grid points in Rust.
**Learning:** `add_map_to_grid_img` did not explicitly check for negative values or boundary values exceeding array logic (which used saturating arithmetic as an inadequate catch-all).
**Prevention:** Explicit boundary checks (`if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 { continue; }`) must be added before memory/buffer offset calculations.
