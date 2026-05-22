# Changelog

All notable changes to `rust-forge-skill` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-05-22

### Added

#### Skill Infrastructure
- **SKILL.md** — Agent entrypoint with decision trees, zero-tolerance anti-patterns, quality gates, and forbidden patterns (585 lines)
- **mcp/** — Machine-readable MCP layer for agent integration
  - `manifest.example.json` — 6 capabilities, 8 validation commands, 6 safety invariants
  - `tool-contract.md` — Formal agent protocol (input/output schemas, 5 phases, safety invariants)
  - `agent-loader-instructions.md` — 7-step loading sequence for AI agents
  - `README.md` — Integration guide for 5 agent types (Claude Code, IDE plugins, MCP servers, CLI agents, repo-scanning)

#### Guides (13 total)
- `guides/00-agent-operating-model.md` — Agent operating contract
- `guides/01-project-architecture.md` — Layer architecture, module design, dependency rules
- `guides/02-crate-workspace-patterns.md` — Workspace setup, inter-crate dependencies
- `guides/03-error-handling-anyhow-thiserror.md` — Error strategy: thiserror for libraries, anyhow for applications
- `guides/04-async-tokio-axum.md` — Async runtime, Axum handlers, graceful shutdown
- `guides/05-cli-clap-tracing.md` — CLI argument parsing, clap setup, tracing
- `guides/06-testing-bench-fuzz.md` — Unit, integration, bench, fuzz testing
- `guides/07-ffi-c-cpp.md` — FFI boundaries, bindgen, unsafe invariants
- `guides/08-security-unsafe-audit.md` — Security auditing, SAFETY comments
- `guides/09-wasm-module-patterns.md` — WASM target, wasm-bindgen, web-sys
- `guides/10-release-ci-msrv.md` — Release pipeline, CI, MSRV policy
- `guides/11-performance-profiling.md` — Benchmarks, flamegraph, cache optimization
- `guides/12-rust-anti-patterns.md` — Forbidden patterns, code smells

#### Templates (6 total)
- `templates/cli-app/` — CLI with clap + tracing (Rust 2024, MSRV 1.85)
- `templates/axum-api/` — REST API with Axum + sqlx (Rust 2024, MSRV 1.85)
- `templates/library-crate/` — Publishable library with thiserror (Rust 2024, MSRV 1.85)
- `templates/ffi-wrapper/` — C FFI with bindgen + SAFETY invariants
- `templates/wasm-module/` — WASM with wasm-bindgen + web-sys
- `templates/workspace-service/` — Multi-crate Cargo workspace

#### Scripts (4 total)
- `scripts/validate_rust_project.sh` — Full quality gate (fmt, clippy, test, optional audit/deny)
- `scripts/generate_cargo_workspace.sh` — Workspace scaffold generator
- `scripts/audit_unsafe.sh` — Unsafe block audit with SAFETY comment verification
- `scripts/check_msrv.sh` — MSRV enforcement (Rust 2024 default, configurable)

#### CI Workflows (2 total)
- `ci/github-actions-rust.yml` — 7-stage pipeline: fmt, clippy, test, nextest, doc, MSRV, benches
- `ci/github-actions-rust-security.yml` — 6-stage security pipeline: audit, deny, unsafe audit, dep audit, unsafe clippy, dep-review

#### Checklists (4 total)
- `checklists/project-start-checklist.md` — 11-section pre-project checklist
- `checklists/code-review-checklist.md` — 11-section code review checklist with quality gate table
- `checklists/security-review-checklist.md` — 13-section security review checklist
- `checklists/release-checklist.md` — 10-section release checklist with verification table

#### Agent Prompts (5 total)
- `examples/agent-prompts/scaffold-cli.md` — Scaffold a CLI project
- `examples/agent-prompts/scaffold-api.md` — Scaffold an Axum API project
- `examples/agent-prompts/refactor-library.md` — Refactor a library to idiomatic patterns
- `examples/agent-prompts/unsafe-audit.md` — Unsafe code audit with report template
- `examples/agent-prompts/load-rust-forge-skill.md` — Universal load prompt for any agent

#### Package Infrastructure
- `deny.toml` — `cargo-deny` configuration (advisories, licenses, banned crates)
- `.github/dependabot.yml` — Automated dependency updates (cargo + GitHub Actions)
- `VERSION` — `0.1.0`
- `CHANGELOG.md` — This file

### Template Instantiation Issues Fixed (pre-release)
- `library-crate/tests/integration.rs` — Replaced hardcoded `mylib` with `{{crate_name}}` template variable
- `ffi-wrapper/tests/basic.rs` — Replaced hardcoded `ffi_wrapper` with `{{crate_name}}` template variable
- `ffi-wrapper/AGENT_NOTES.md` — Added (was missing)
- `wasm-module/AGENT_NOTES.md` — Added (was missing)
- `workspace-service/AGENT_NOTES.md` — Added (was missing)

### Template Fixes (final audit 0.1.0)
- `templates/axum-api/src/main.rs` — Removed unused `TraceLayer` import
- `templates/axum-api/src/app.rs` — Removed unused `middleware` import, fixed `let_and_return` clippy warning
- `templates/axum-api/.clippy.toml` — Fixed field names: `cognitive_complexity` → `cognitive-complexity-threshold`, `too_many_arguments_threshold` → `too-many-arguments-threshold`
- `templates/axum-api/src/error.rs` — Added `#[allow(dead_code)]` to suppress unused enum warning
- `templates/axum-api/src/state.rs` — Added `#[allow(dead_code)]` to suppress unused field warning
- `templates/ffi-wrapper/src-c/` — Added `lib.h` and `lib.c` (were referenced but missing)
- `templates/library-crate/rust-toolchain.toml` — Fixed malformed TOML (missing `[toolchain]` table header)

### Policy
- **MSRV:** 1.85.0 (Rust 2024 edition)
- **License:** MIT OR Apache-2.0
- **Protocol:** `rust-forge-skill-v1`
