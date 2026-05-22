# Rust Forge Skill

[![MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/Rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Push](https://img.shields.io/badge/status-active-brightgreen.svg)]()

**A reusable agent skill pack for generating idiomatic, production-ready Rust projects.**

> This is not a Rust application. It is a skill pack for AI coding agents that need to scaffold, generate, and validate production-grade Rust code.

## What This Is

`rust-forge-skill` provides:

- **SKILL.md** — Agent entrypoint with decision trees, quality gates, and forbidden anti-patterns
- **Guides** — Deep dives into error handling, async, FFI, WASM, performance, and more
- **Templates** — Production-ready project scaffolds for CLI, API, library, FFI, WASM, and workspace patterns
- **Scripts** — CI-grade validation: format, clippy, tests, MSRV enforcement, unsafe audits
  - `validate_rust_project.sh` — full quality gate (fmt, clippy, test, audit, deny)
  - `generate_cargo_workspace.sh` — scaffold a multi-crate workspace
  - `audit_unsafe.sh` — find unsafe blocks, generate audit report
  - `check_msrv.sh` — verify MSRV declared in Cargo.toml matches reality

See `scripts/README.md` for detailed usage.
- **Checklists** — Project start, code review, security review, release gates
- **Examples** — Agent prompts for common scaffolding tasks

## Quick Start

```bash
# List available templates
ls rust-forge-skill/templates/

# Generate a new CLI project
cp -r rust-forge-skill/templates/cli-app ./my-project
cd my-project && cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace --all-features

# Generate a workspace
bash rust-forge-skill/scripts/generate_cargo_workspace.sh my-workspace

# Validate any project
bash rust-forge-skill/scripts/validate_rust_project.sh
```

## Directory Structure

```
rust-forge-skill/
├── SKILL.md                           # Agent entrypoint
├── README.md                          # This file
├── LICENSE                            # MIT OR Apache-2.0
├── guides/                            # 12 deep-dive guides
│   ├── 00-agent-operating-model.md    # Agent operating contract
│   ├── 01-project-architecture.md     # Layer architecture, module design
│   ├── 02-crate-workspace-patterns.md # Workspace setup, inter-crate deps
│   ├── 03-error-handling-anyhow-thiserror.md  # Error strategy selection
│   ├── 04-async-tokio-axum.md         # Async runtime, Web framework
│   ├── 05-cli-clap-tracing.md         # CLI patterns, argument parsing
│   ├── 06-testing-bench-fuzz.md       # Unit, integration, bench, fuzz
│   ├── 07-ffi-c-cpp.md                # C interop, bindgen, unsafe boundaries
│   ├── 08-security-unsafe-audit.md    # Security auditing, safety invariants
│   ├── 09-wasm-module-patterns.md     # WASM target, web-sys, wasm-bindgen
│   ├── 10-release-ci-msrv.md          # Release pipeline, CI, MSRV policy
│   ├── 11-performance-profiling.md    # Bench, flamegraph, cache optimization
│   └── 12-rust-anti-patterns.md       # Forbidden patterns, code smells
├── templates/                         # 6 production-ready templates
│   ├── cli-app/                       # CLI with clap + tracing
│   ├── axum-api/                      # REST API with axum + sqlx
│   ├── library-crate/                  # Publishable library with thiserror
│   ├── ffi-wrapper/                   # C FFI with bindgen
│   ├── wasm-module/                   # WASM with wasm-bindgen
│   └── workspace-service/              # Multi-crate workspace
├── scripts/                           # 4 validation scripts
│   ├── validate_rust_project.sh       # Full quality gate
│   ├── generate_cargo_workspace.sh     # Workspace scaffold generator
│   ├── audit_unsafe.sh                 # Unsafe block auditing
│   └── check_msrv.sh                  # MSRV enforcement
├── ci/                                # 2 GitHub Actions workflows
│   ├── github-actions-rust.yml         # Standard CI pipeline
│   └── github-actions-rust-security.yml # Security-focused CI
├── checklists/                         # 4 procedural checklists
│   ├── project-start-checklist.md
│   ├── code-review-checklist.md
│   ├── security-review-checklist.md
│   └── release-checklist.md
└── examples/
    └── agent-prompts/                  # 4 agent prompt examples
        ├── scaffold-cli.md
        ├── scaffold-api.md
        ├── refactor-library.md
        └── unsafe-audit.md
```

## Template Selection Guide

| Use Case | Template | Key Dependencies |
|---|---|---|
| CLI tool | `cli-app/` | clap, tracing, tokio (optional) |
| HTTP API | `axum-api/` | axum, tower, tower-http, sqlx |
| Publishable library | `library-crate/` | thiserror, serde |
| C/C++ FFI | `ffi-wrapper/` | cbindgen, ffi-nalgebra |
| WebAssembly | `wasm-module/` | wasm-bindgen, web-sys |
| Multi-crate service | `workspace-service/` | workspace-aware |

## Quality Gates

Every template, when instantiated, must pass:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Libraries additionally must pass:

```bash
cargo doc --workspace --all-features --no-deps
./scripts/check_msrv.sh
```

FFI crates additionally must pass:

```bash
./scripts/audit_unsafe.sh
```

## MSRV Policy

- **Default MSRV: 1.85.0** (Rust 2024 edition default)
- Libraries must declare MSRV in `Cargo.toml` `[package]` metadata
- CI enforces MSRV via `scripts/check_msrv.sh`
- Override per-project: `RUST_MSRV=1.82.0 ./scripts/check_msrv.sh`


## CI Usage

Copy a workflow file to your repository's `.github/workflows/` directory:

```bash
# Standard CI pipeline (fmt, clippy, tests, doc, MSRV, benches)
cp rust-forge-skill/ci/github-actions-rust.yml .github/workflows/rust.yml

# Security-focused pipeline (audit, deny, unsafe audit, dependency review)
cp rust-forge-skill/ci/github-actions-rust-security.yml .github/workflows/rust-security.yml
```

Both workflows are self-documenting with inline comments. The standard CI workflow covers formatting, linting, testing, documentation, MSRV enforcement, and benchmarks. The security workflow adds `cargo audit`, `cargo deny`, unsafe block auditing, and dependency size/outdated checks.

### Enabling Both Workflows

For most projects, enable both:

```yaml
# .github/workflows/rust.yml        — triggers on push + PR
# .github/workflows/rust-security.yml  — triggers on push + PR
```

### Optional: Native Dependency Review

GitHub's native [Dependency Review action](https://github.com/actions/dependency-review-action) is a complementary check that blocks PRs introducing known vulnerable dependencies. Enable it separately at: **Settings → Security → Security management → Dependency Review**.

---

## Checklist Usage

Each checklist is a standalone Markdown file. Use them at the appropriate phase:

| Checklist | When to Use |
|---|---|
| `checklists/project-start-checklist.md` | Before scaffolding a new project |
| `checklists/code-review-checklist.md` | During code review (before merging) |
| `checklists/security-review-checklist.md` | Before any release or security-sensitive change |
| `checklists/release-checklist.md` | Before publishing to crates.io or cutting a release |

### Using Checklists in Code Review

Reference the relevant sections in PR reviews. For example, after an agent delivers code, verify:

```
- [ ] Anti-patterns (checklists/code-review-checklist.md)
- [ ] cargo clippy -- -D warnings passes
- [ ] All tests pass
```

### Security Review Workflow

For security-sensitive changes:

```bash
# 1. Agent runs self-audit using security-review-checklist.md
# 2. Human reviewer verifies the checklist
# 3. cargo audit && cargo deny check advisories licenses ban passes
# 4. Unsafe audit ./scripts/audit_unsafe.sh passes (if FFI/WASM)
```

---

## Example Agent Prompts Usage

The `examples/agent-prompts/` directory contains ready-to-use prompts for common agent tasks. Each prompt is copy-pasteable and enforces the same quality gates from the skill.

### Available Prompts

| Prompt File | Use Case |
|---|---|
| `examples/agent-prompts/scaffold-cli.md` | Scaffolding a new CLI tool project |
| `examples/agent-prompts/scaffold-api.md` | Scaffolding an Axum REST API project |
| `examples/agent-prompts/refactor-library.md` | Refactoring an existing library to idiomatic patterns |
| `examples/agent-prompts/unsafe-audit.md` | Auditing unsafe code in a FFI or WASM project |

### How to Use

1. Copy the prompt file content
2. Replace `{{placeholder}}` values with your project specifics
3. Give the full prompt to an AI coding agent
4. The agent will read `SKILL.md`, follow the steps, and produce a final report

### Prompt Structure

Every prompt enforces:

1. **Skill read** — agent must read `rust-forge-skill/SKILL.md` before writing code
2. **Guide reads** — agent must read the relevant guides for the task
3. **Anti-pattern enforcement** — `.unwrap()`, `unsafe` without `SAFETY`, etc. are forbidden
4. **Validation commands** — agent must run all quality gates and report output
5. **Final report** — agent must produce an audit table or summary

### Example: Scaffolding a CLI

```bash
# Copy and customize the prompt
cat rust-forge-skill/examples/agent-prompts/scaffold-cli.md
# Edit {{project_name}}, {{description}}, {{author}}

# Give to an agent with the full content
```

The agent will scaffold the project, run validation commands, and deliver:
1. Complete project structure
2. Terminal output of each validation command
3. Summary of what was created

---


## MCP Layer

This skill includes an MCP/agent integration layer for programmatic loading by AI agents and tool runners:

| File | Purpose |
|---|---|
| `mcp/manifest.example.json` | Machine-readable skill manifest with capabilities, validation commands, and expected outputs |
| `mcp/tool-contract.md` | Formal agent protocol: input schema, output schema, phase definitions, safety invariants |
| `mcp/agent-loader-instructions.md` | 7-step loading sequence for agents that need explicit guidance |
| `mcp/README.md` | Integration guide for Claude Code, IDE plugins, MCP servers, CLI agents, and repo-scanning agents |
| `examples/agent-prompts/load-rust-forge-skill.md` | Universal copy-paste prompt to give to any coding agent |

### Quick Load

Give this to any agent:

```
Use rust-forge-skill to complete this task: {YOUR_TASK}
Read SKILL.md, identify project type, read matching guides,
scaffold/audit/refactor, run quality gates (cargo fmt --all -- --check &&
cargo clippy --workspace --all-targets --all-features -- -D warnings &&
cargo test --workspace --all-features), produce final report.
No .unwrap() in production. No unsafe without SAFETY comment.
```

See `mcp/README.md` for full loading instructions for Claude Code, IDE agents, MCP servers, CLI agents, and repo-scanning agents.

---

## Self-Testing the Skill Pack

The skill pack includes self-test scenarios that prove an AI agent can effectively use the skill. See `self-tests/README.md` for how to run them.

---
## License

MIT OR Apache-2.0 — choose whichever suits your project.