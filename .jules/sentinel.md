## 2024-12-10 - [idna crate Vulnerability Fix]
**Vulnerability:** `idna` accepts Punycode labels that do not produce any non-ASCII when decoded (RUSTSEC-2024-0421), which can lead to privilege escalation. Found via `cargo audit`.
**Learning:** Upgrading `idna` directly failed due to conflicts with the `smallvec` dependency from other crates.
**Prevention:** In complex dependency trees with Cargo, updating intermediate dependencies (`smallvec` and `url` in this case) may be required to resolve version conflicts when upgrading an underlying vulnerable crate.

## 2024-04-06 - Unsafe ECS Query Unwrapping
**Vulnerability:** Calling `.single()` or `.single_mut()` on a Bevy ECS query panics and crashes the application (Denial of Service) if there is not exactly one entity that matches the query.
**Learning:** In interactive or dynamic contexts, entities like the main window or camera can occasionally un-mount or re-mount, or simply not be correctly matched. Panics in the engine loop cause an unrecoverable crash.
**Prevention:** Always use `.get_single()` or `.get_single_mut()` and handle the `Result` gracefully (e.g. via `if let Ok(...)`) to fail securely without crashing the application.
