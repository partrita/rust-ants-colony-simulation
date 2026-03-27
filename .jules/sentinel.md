## 2024-12-10 - [idna crate Vulnerability Fix]
**Vulnerability:** `idna` accepts Punycode labels that do not produce any non-ASCII when decoded (RUSTSEC-2024-0421), which can lead to privilege escalation. Found via `cargo audit`.
**Learning:** Upgrading `idna` directly failed due to conflicts with the `smallvec` dependency from other crates.
**Prevention:** In complex dependency trees with Cargo, updating intermediate dependencies (`smallvec` and `url` in this case) may be required to resolve version conflicts when upgrading an underlying vulnerable crate.
