🛡️ Sentinel: [MEDIUM] Fix idna vulnerability

🚨 Severity: MEDIUM
💡 Vulnerability: `idna` accepts Punycode labels that do not produce any non-ASCII when decoded (RUSTSEC-2024-0421), which can lead to privilege escalation.
🎯 Impact: This could allow privilege escalation when host name comparison is part of a privilege check.
🔧 Fix: Updated the `url` crate to `2.5.4` which patches the underlying `idna` dependency, along with required `smallvec` bumps.
✅ Verification: Ran `cargo audit` to confirm the vulnerability is gone. Ran `cargo test` to ensure functionality.
