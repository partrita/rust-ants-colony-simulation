## 2024-12-10 - [idna crate Vulnerability Fix]
**Vulnerability:** `idna` accepts Punycode labels that do not produce any non-ASCII when decoded (RUSTSEC-2024-0421), which can lead to privilege escalation. Found via `cargo audit`.
**Learning:** Upgrading `idna` directly failed due to conflicts with the `smallvec` dependency from other crates.
**Prevention:** In complex dependency trees with Cargo, updating intermediate dependencies (`smallvec` and `url` in this case) may be required to resolve version conflicts when upgrading an underlying vulnerable crate.

## 2024-04-06 - Unsafe ECS Query Unwrapping
**Vulnerability:** Calling `.single()` or `.single_mut()` on a Bevy ECS query panics and crashes the application (Denial of Service) if there is not exactly one entity that matches the query.
**Learning:** In interactive or dynamic contexts, entities like the main window or camera can occasionally un-mount or re-mount, or simply not be correctly matched. Panics in the engine loop cause an unrecoverable crash.
**Prevention:** Always use `.get_single()` or `.get_single_mut()` and handle the `Result` gracefully (e.g. via `if let Ok(...)`) to fail securely without crashing the application.
## 2024-04-07 - Denial of Service (DoS) Risk in Bevy ECS Queries
**Vulnerability:** The use of `.single_mut()` (and potentially `.single()`) on Bevy `Query` objects can cause unrecoverable thread panics if the query matches zero or more than one entities. This could happen temporarily during loading or when components dynamically unmount, leading to a Denial of Service (crash) in the main simulation thread.
**Learning:** Hard-failing assumptions in ECS queries introduces brittle constraints in games/simulations where entity counts can fluctuate temporarily.
**Prevention:** Always use `.get_single_mut()` or `.get_single()` combined with `if let Ok(...)` or appropriate error handling to safely update entities without panicking the application.

## 2024-04-10 - [Prevent DoS via Panics]
**Vulnerability:** Use of `unwrap()`, `expect()`, or `panic!()` can cause unrecoverable crashes (Denial of Service).
**Learning:** Even if the current codebase is clean of these macros, future code additions might introduce them.
**Prevention:** Use `#![forbid(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` at the crate level to enforce panic-free error handling at compile time.
## 2024-05-18 - Rand Dependency Unsoundness Vulnerability
**Vulnerability:** `rand` version 0.8.5 has an unsoundness vulnerability when used with a custom logger (RUSTSEC-2026-0097).
**Learning:** Vulnerabilities can exist in foundational dependencies. Upgrading to the latest major version (0.9.0) resolved the issue but required updating all call sites due to deprecations (e.g., `thread_rng` -> `rng`, `gen_range` -> `random_range`). Also, blindly updating other unrelated vulnerable dependencies like `kd-tree` may cause cascading breaking changes downstream and should be done with care.
**Prevention:** Regularly audit foundational crates using `cargo audit` and keep them up-to-date, making sure to carefully read compilation warnings or errors when major version bumps introduce API changes.

## 2026-04-16 - [Prevent DoS via Panic from Array Indexing]
**Vulnerability:** Use of direct array/slice indexing (e.g. `array[index]`) can cause out-of-bounds panics resulting in Denial of Service (DoS) if logic is flawed.
**Learning:** The existing codebase had `#![forbid(clippy::unwrap_used, clippy::expect_used, clippy::panic)]` but missed array indexing hazards.
**Prevention:** Added `clippy::indexing_slicing` to the `#![forbid(...)]` list and replaced all indexing with safe `.get()` and `.get_mut()` patterns with graceful fallback.

## 2024-05-24 - Fix Integer Underflow Panic in quickselect
**Vulnerability:** The `find_n_points_with_max_z` function delegated to a `quickselect` implementation which panicked with an integer underflow `attempt to subtract with overflow` when the requested `n` parameter was `0`. This is a DoS vulnerability as Rust panics terminate the process (and are strictly forbidden in this codebase).
**Learning:** Even internal helper methods must validate their arguments against boundary conditions like 0 before doing arithmetic involving subtraction, especially with unsigned integer types (`usize`).
**Prevention:** Always validate integer inputs before subtracting from them (e.g. `n - 1`), especially if those inputs can logically be `0`. Return early for degenerate cases (`n == 0`).
