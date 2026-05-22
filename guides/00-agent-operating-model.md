# 00 — Agent Operating Model

**Purpose:** Define how agents must reason before touching any Rust code. Rust rewards careful, deliberate changes — not impulsive rewrites.

---

## The Inspection-First Rule

**Every Rust modification begins with inspection.** Never write code before understanding what exists.

### Required Inspection Steps

**1. Read `Cargo.toml`**

```bash
# Always start here
cat Cargo.toml
```

Check:
- `edition` — what Rust edition is active? (default: 2024)
- `rust-version` — what MSRV is declared?
- `workspace` — is this part of a workspace?
- `[dependencies]` — what crates are already in use?
- `[dev-dependencies]` — what test/mock tools exist?
- `[features]` — what feature flags exist?

**2. Identify Edition and MSRV**

```toml
[package]
edition = "2024"
rust-version = "1.85"
```

- **Rust 2024** is the default. Older projects may use 2021.
- MSRV of 1.85 means the project targets a modern Rust toolchain.
- If MSRV is lower (e.g., 1.70), certain features and crates are unavailable.
- **Do not bump edition without consulting the project's contribution guidelines.**

**3. Inspect Workspace Layout**

```bash
ls -la
find . -name "Cargo.toml" -not -path "*/target/*"
```

A workspace has a root `Cargo.toml` with `[workspace]` section and `members = [...]`.

```
my-workspace/
├── Cargo.toml           # workspace root
├── crates/
│   ├── core/           # library crate
│   ├── api/            # binary crate
│   └── client/         # library crate
```

**4. Inspect Feature Flags**

```toml
[features]
default = []
sqlite = ["sqlx/sqlite"]
postgres = ["sqlx/postgres"]
full = ["sqlite", "postgres", "tracing"]
```

- Never add dependencies without understanding existing feature flags.
- If a crate supports `default = []`, prefer explicit features over all-features-on.

**5. Inspect Existing Error Style**

Look at `src/error.rs` or `src/lib.rs`:

```rust
// Does it use thiserror (library style)?
#[derive(Error, Debug)]
pub enum Error {
    #[error("not found: {0}")]
    NotFound(String),
}

// Or anyhow (application boundary style)?
pub type Result<T> = std::result::Result<T, anyhow::Error>;
```

**Match the existing pattern.** If the project uses `thiserror`, do not introduce `anyhow` in the same crate.

**6. Inspect Async Runtime**

```bash
grep -rn "tokio\|async\|spawn" src/ --include="*.rs" | head -20
```

- Is `tokio` already in dependencies?
- Are there existing `async fn` signatures?
- Does the project use `#[async_trait]`?
- Is there a `#[tokio::main]` entry point?

**7. Inspect Logging Style**

```bash
grep -rn "tracing\|log\|println\|env_logger" src/ --include="*.rs" | head -20
```

- Does the project use `tracing` (structured) or `log` + `env_logger`?
- Are there any `println!` / `eprintln!` calls in production code? (flag these as anti-patterns)
- Is tracing initialized in `main.rs` or `lib.rs`?

**8. Inspect Test Infrastructure**

```bash
grep -rn "mockall\|wiremock\|proptest\|quickcheck" Cargo.toml
ls tests/
```

- What testing tools are already in `[dev-dependencies]`?
- Are there existing integration tests?
- Is `mockall` used for mocking traits?

---

## The Do-Not-Rewrite-Everything Rule

Rust projects resist large rewrites. The type system, borrow checker, and ownership model make mass refactoring error-prone and slow.

**When you encounter a messy file:**

```
DO:
- Add a new small module for new functionality
- Extract a function or type to a clearly-named helper
- Add a thin wrapper around existing behavior
- Write tests for the existing behavior before changing it

DO NOT:
- Rewrite main.rs from scratch
- Replace all error types simultaneously
- Convert sync code to async in one pass
- Change the module structure without a migration plan
- Delete code and replace with "better" code without tests
```

**The correct Rust modification pattern:**

```
1. Understand the existing code (read it twice)
2. Write a test that captures current behavior
3. Make the smallest possible change
4. Verify the test still passes
5. Run clippy and fmt
6. Commit the isolated change
```

---

## Pre-Change Checklist

Before any modification, confirm:

```
[ ] Read existing Cargo.toml (edition, MSRV, workspace?)
[ ] Identified the crate's layer (domain/service/repository/api)
[ ] Understood existing error handling pattern (thiserror vs anyhow)
[ ] Checked if async is already in use
[ ] Checked logging style (tracing vs log)
[ ] Identified test infrastructure
[ ] Understood module boundaries
[ ] Confirmed the change fits the existing architecture
[ ] Noted any constraints (MSRV, edition, no-std, wasm target)
```

---

## Agent Communication Protocol

When reporting Rust changes, always state:

```
## Inspection Summary
- Edition: 2024 | MSRV: 1.85
- Workspace: yes (3 crates: core, api, client)
- Error style: thiserror (domain layer)
- Async runtime: tokio (full)
- Logging: tracing with JSON layer
- Testing: mockall + proptest

## Change Made
File: src/domain/models.rs
Change: Added validate() to Item struct, returns ValidationError

## Validation
cargo fmt --all -- --check  # PASS
cargo clippy --workspace --all-targets --all-features -- -D warnings  # PASS (0 warnings)
cargo test --workspace --all-features  # PASS (12/12)

## Known Limitations
- MSRV bump from 1.82 to 1.85 due to thiserror 2.0 requirement
```

---

## Summary

- **Inspect before edit.** Seven mandatory inspection steps.
- **Match existing patterns.** Don't introduce `anyhow` into a `thiserror` codebase.
- **Small changes only.** Rust resists rewrites; prefer incremental additions.
- **Always validate.** Format, lint, test after every change.