# Project Start Checklist

Use this checklist before starting any new Rust project generated with rust-forge-skill.

---

## 1. Project Type

- [ ] Identify correct template
  - CLI tool → `templates/cli-app/`
  - HTTP REST API → `templates/axum-api/`
  - Publishable library → `templates/library-crate/`
  - C/C++ FFI wrapper → `templates/ffi-wrapper/`
  - WebAssembly module → `templates/wasm-module/`
  - Multi-crate service → `templates/workspace-service/`

## 2. Template Choice & Customisation

- [ ] Copy template to target location
- [ ] Update `Cargo.toml`:
  - `name` — kebab-case crate name
  - `version` — start at `0.1.0`
  - `description` — one-line summary
  - `authors` — `[ "Name <email>" ]`
  - `license` — MIT OR Apache-2.0
  - `rust-version` — MSRV field (default: `1.85`)
- [ ] Replace all `{{placeholder}}` strings in source files
- [ ] Run `cargo generate-lockfile`

## 3. Rust Version & Edition

- [ ] Confirm MSRV: `1.85` (default) or specify lower for library compatibility
- [ ] Confirm edition: Rust 2024 (default, set in `Cargo.toml`)
- [ ] Record MSRV in project README

## 4. Async Runtime

- [ ] Async required? → Add `tokio` with appropriate features
- [ ] Async not required? → Use blocking I/O only
- [ ] If async: choose `#[tokio::main]` vs `#[async_rt::main]`

## 5. Error Model

- [ ] Library crate → `thiserror` for domain errors
- [ ] Application/CLI → `anyhow` for boundary errors
- [ ] Both types present? → Layer accordingly: library uses `thiserror`, app uses `anyhow`
- [ ] Error variants defined and documented

## 6. Logging & Observability

- [ ] Add `tracing` (preferred over `log`)
- [ ] Add `tracing-subscriber` with sensible default format
- [ ] Configure `RUST_LOG` env var parsing
- [ ] Add `tracing-OpenTelemetry` if distributed tracing needed

## 7. Test Strategy

- [ ] Unit tests: all public functions covered
- [ ] Integration tests: API / repository layer
- [ ] Error path tests: not just happy path
- [ ] Property-based tests: for parsers, serializers (add `proptest`)
- [ ] Fuzz tests: for deserializers (add `cargo-fuzz`)
- [ ] No `.unwrap()` in test utility functions

## 8. Security Gates

- [ ] `cargo fmt --all -- --check` in pre-commit
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` in CI
- [ ] `cargo audit` in CI
- [ ] `cargo deny check advisories licenses` in CI
- [ ] Unsafe audit if FFI/WASM template used

## 9. Module Structure

- [ ] Create `domain/` — types, models, error types
- [ ] Create `service/` — business logic
- [ ] Create `repository/` — data access
- [ ] Create `api/` or `infrastructure/` — HTTP / external I/O
- [ ] Verify layer dependencies flow inward only

## 10. Initial Commit

- [ ] `git init`
- [ ] Add `.gitignore` (copy from template or generate with gitignore.io)
- [ ] Initial commit: `"Initial project structure from rust-forge-skill"`
- [ ] Verify CI passes on initial commit

## 11. CI Setup

- [ ] Copy `ci/github-actions-rust.yml` to `.github/workflows/rust.yml`
- [ ] For security-sensitive projects: also copy `ci/github-actions-rust-security.yml`
- [ ] Enable Codecov token (optional)
- [ ] Verify CI passes on first push

## Next Steps

| Template | Next Action |
|---|---|
| `cli-app/` | Implement clap subcommands, wire up tracing |
| `axum-api/` | Set up routes, handlers, DB connection pool |
| `library-crate/` | Implement core functionality, add doctests |
| `ffi-wrapper/` | Verify `bindgen` generates bindings, test unsafe boundary |
| `wasm-module/` | Verify `wasm-pack build` succeeds |
| `workspace-service/` | Define workspace members, shared crates |
