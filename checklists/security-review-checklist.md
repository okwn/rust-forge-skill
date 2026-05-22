# Security Review Checklist

Use this checklist when reviewing Rust code for security vulnerabilities using rust-forge-skill.

---

## Unsafe Blocks

- [ ] No `unsafe` blocks without `SAFETY` comments
- [ ] SAFETY comments explain all invariants
- [ ] SAFETY comments explain why invariant holds
- [ ] SAFETY comments state UB consequences if invariant is violated
- [ ] No buffer overflow possible in array/slice access
- [ ] No use-after-free (check raw pointer usage)
- [ ] No data races (check `static mut`, concurrent access)
- [ ] Uninitialized memory handled with `MaybeUninit`
- [ ] `unsafe impl` for `Send`/`Sync` documented with justification

---

## FFI Boundaries

- [ ] All pointers validated before dereference
- [ ] Memory ownership clear at boundary
- [ ] `CString`/`CStr` UTF-8 conversion errors handled
- [ ] Foreign memory freed correctly (ownership rule explicit)
- [ ] Thread safety documented for exported functions
- [ ] `#[repr(C)]` structs have consistent field ordering
- [ ] No assumption about struct padding without `#[repr(packed)]`

---

## Dependency Audit

- [ ] `cargo audit` passes (no known vulnerabilities in dependency tree)
- [ ] `cargo deny check advisories licenses ban` passes
- [ ] Dependencies are up-to-date (no known CVEs in current versions)
- [ ] No transitive dependencies on untrusted code
- [ ] No use of crates with `yanked = true` on crates.io
- [ ] `cargo outdated --workspace` reviewed (optional update review)

---

## Secrets Management

- [ ] No secrets in source code (API keys, tokens, passwords — use env vars)
- [ ] No secrets in logs (`tracing::info!` calls checked for sensitive data)
- [ ] Sensitive data cleared on drop (`Zeroize` or custom `Drop`)
- [ ] No API keys or tokens in error messages
- [ ] `.gitignore` excludes `.env` files
- [ ] `.env.example` contains only placeholder values
- [ ] `tracing` metadata does not include PII or credentials

---

## Input Validation

- [ ] All user input validated before use
- [ ] Length checks on strings/vectors before indexing
- [ ] Integer overflow checked in size calculations (use `saturating_*` or explicit checks)
- [ ] Path traversal prevented for file operations (`../` check or canonicalization)
- [ ] SQL injection prevented (parameterized queries only — no raw SQL concatenation)
- [ ] XSS prevented in any string output (HTML escaping for web contexts)
- [ ] Deserialization: validate before use (no `Skip` without type checks)
- [ ] Environment variable parsing with fallible conversion (no `unwrap()` on env vars)

---

## File Path Operations

- [ ] User-supplied paths are validated and canonicalized
- [ ] Symlink attacks considered (`std::fs::canonicalize` before access)
- [ ] Path separator differences handled (`/` vs `\`)
- [ ] File handles not left open across error boundaries
- [ ] Tempfile created with secure random suffix

---

## Network & I/O

- [ ] Network timeouts set on all outbound connections
- [ ] No unbounded request sizes (enforce `Content-Length` limits)
- [ ] Rate limiting on public-facing endpoints
- [ ] TLS configuration uses modern cipher suites (no TLS 1.0/1.1)
- [ ] Hostname verification enabled for TLS
- [ ] No connection string injection (URLs parsed with `Url` type, not string manipulation)

---

## Deserialization

- [ ] `serde` `deserialize_any` not used without allowlist
- [ ] No `#[serde(flatten)]` with conflicting keys
- [ ] Untrusted data deserialized into guarded enum variants
- [ ] `skip_serializing_none` considered to prevent `null` leakage
- [ ] TOML/YAML/JSON parsing errors produce user-friendly messages (not raw lexer output)

---

## Cryptography

- [ ] No custom crypto implementations (use `ring`, `rustls`, `bcrypt`, `argon2`, `sha2`)
- [ ] No deprecated algorithms (MD5, SHA1 for security purposes, DES, 3DES)
- [ ] CSPRNG used for random values (`rand::rngs::StdRng` with `rand::SeedableRng`)
- [ ] Passwords hashed (bcrypt, argon2, scrypt — not plain MD5/SHA)
- [ ] No hardcoded keys/seeds in source
- [ ] IVs/nonces are unique per encryption operation (never reused)
- [ ] MAC verification failure does not leak timing information

---

## Supply Chain

- [ ] `cargo deny` checks pass (no banned crates, no prohibited licenses)
- [ ] `Cargo.lock` is committed (reproducible builds)
- [ ] Build script (`build.rs`) sources are verified
- [ ] No `links` fields pointing to untrusted native libraries
- [ ] WASM build validated: no `unsafe` without documentation
- [ ] Lockfile hash verified in CI

---

## Error Handling & Information Leakage

- [ ] Error messages do not leak internal paths, crate names, or stack traces to users
- [ ] Internal errors not exposed to API clients
- [ ] Panic handlers in async code don't swallow errors
- [ ] `tracing` errors logged server-side, not sent to client
- [ ] Graceful degradation on external service failures
- [ ] Panic on startup with clear message (do not silently continue with bad config)

---

## Security Tool Commands

```bash
# Run all security checks
cargo audit
cargo deny check advisories licenses ban
./scripts/audit_unsafe.sh
cargo clippy --workspace --all-targets --all-features -- -D unsafe_code -D deprecated

# For FFI projects
bindgen --verify-cfg
./scripts/audit_unsafe.sh

# For WASM projects
wasm-pack build --check
```

| Check | Command | Pass Criteria |
|---|---|---|
| Vulns | `cargo audit` | Zero vulnerabilities |
| Policy | `cargo deny check advisories licenses ban` | No bans, no prohibited licenses |
| Unsafe | `./scripts/audit_unsafe.sh` | All unsafe documented |
| Lints | `cargo clippy -- -D unsafe_code` | Zero unsafe code lint denials |
| WASM | `wasm-pack build --check` | Builds without errors |
