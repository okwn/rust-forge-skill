# Agent Prompt: Refactor a Rust Library

You are an AI coding agent. Before writing any code, **read the rust-forge-skill** skill definition:

```
Read the skill at: rust-forge-skill/SKILL.md
```

---

## Task

Refactor an existing Rust library `{{crate_name}}` to meet idiomatic, production-grade standards.

**Current state:** Library using `anyhow` for all errors, `.unwrap()` scattered throughout, no domain error types, mixed layer responsibilities.

**Target state:** Idiomatic Rust library with `thiserror` domain errors, proper error propagation via `?`, clean layer separation, zero `.unwrap()` in library code.

---

## Steps

1. **Read** the relevant skill guides before writing any code:
   - `rust-forge-skill/guides/03-error-handling-anyhow-thiserror.md` — Error handling best practices
   - `rust-forge-skill/guides/12-rust-anti-patterns.md` — Forbidden patterns
   - `rust-forge-skill/guides/01-project-architecture.md` — Module structure

2. **Audit** the current codebase — do not skip this step:

   Find and record all instances of:
   ```bash
   grep -rn "\.unwrap()" --include="*.rs" src/
   grep -rn "\.expect(" --include="*.rs" src/
   grep -rn "anyhow::Result" --include="*.rs" src/
   grep -rn "unsafe {" --include="*.rs" src/
   ```

   Produce an audit table:
   | File | Line | Issue | Fix Needed |
   |---|---|---|---|
   | `src/lib.rs` | 42 | `.unwrap()` | Replace with `?` |

3. **Refactor error handling:**
   - Create `src/error.rs` with `thiserror` enum for domain errors
   - Replace all `anyhow::Error` in library code with typed domain errors
   - Keep `anyhow` only at the library boundary (if needed for FFI or plugin APIs)
   - Create proper error variants: `NotFound`, `ValidationError`, `SerializationError`, etc.
   - Replace every `.unwrap()` with `?` using the new error types

4. **Update module structure:**
   - Ensure `domain/` → `service/` → `repository/` layers are separate
   - Move error types to domain layer
   - Export clean public API via `lib.rs`
   - Add `pub mod prelude;` if it reduces import noise

5. **Add tests:**
   - Unit tests for domain validation logic
   - Error path tests for each error variant
   - Property-based tests with `proptest` for parsers/serializers

6. **Run validation commands** (see below). All must pass before delivery.

---

## Quality Requirements

These are non-negotiable. The deliverable is only accepted when all pass:

- **Zero `.unwrap()` or `.expect()` in library code**
- **All errors are typed with `thiserror`** (domain errors)
- **`anyhow` only at library boundary** (if explicitly needed for FFI)
- **Module structure follows layer architecture**
- **All `unsafe` blocks have `SAFETY` comments**
- **MSRV: 1.85.0** maintained (do not raise MSRV without justification)
- **All tests pass before delivery**

---

## Validation Commands

Run these commands in sequence. **All must pass.** Report the output of each.

```bash
cargo fmt --all -- --check
echo "=== FORMAT CHECK: PASS ==="

cargo clippy --workspace --all-targets --all-features -- -D warnings
echo "=== CLIPPY CHECK: PASS ==="

cargo test --workspace --all-features
echo "=== TEST CHECK: PASS ==="

cargo doc --workspace --all-features --no-deps
echo "=== DOC CHECK: PASS ==="

./scripts/check_msrv.sh
echo "=== MSRV CHECK: PASS ==="
```

---

## Deliverables

1. **Refactored library** with:
   - New `src/error.rs` module with `thiserror` enum
   - Updated domain models with validation
   - All `.unwrap()` replaced with proper error propagation
   - Updated `lib.rs` exports (clean public API)
   - All tests passing

2. **Audit report** showing:
   - Table of all issues found (file, line, issue type)
   - Table of all fixes applied
   - Remaining technical debt (if any) with justification

3. **Validation output** — copy the terminal output of each validation command

---

## Anti-Patterns That Fail Code Review

- Any `.unwrap()` or `.expect()` remaining in library code
- `anyhow::Error` used as domain error type
- `thiserror` used in application code (it's for libraries)
- Undocumented `unsafe` blocks
- MSRV raised without documented justification
- Service layer directly constructing domain types (bypass validation)
