# Agent Prompt: Scaffold a CLI Project

You are an AI coding agent. Before writing any code, **read the rust-forge-skill** skill definition:

```
Read the skill at: rust-forge-skill/SKILL.md
```

---

## Task

Create a new Rust CLI project named `{{project_name}}` with the following requirements:

- **Project type:** CLI tool
- **Description:** {{description}}
- **Author:** {{author}}
- **Subcommands:** `serve`, `process`, `help`
- **Async:** Optional — use `tokio` if async features are needed

---

## Steps

1. **Read** the relevant skill guides before writing any code:
   - `rust-forge-skill/guides/05-cli-clap-tracing.md` — CLI patterns
   - `rust-forge-skill/guides/03-error-handling-anyhow-thiserror.md` — Error handling
   - `rust-forge-skill/guides/01-project-architecture.md` — Project structure

2. **Select template:** Copy `rust-forge-skill/templates/cli-app/` to the target location

3. **Customize** the template:
   - Update `Cargo.toml` with project name, description, author, MSRV
   - Update `README.md` with usage instructions
   - Replace all `{{placeholder}}` strings in source files
   - Add project-specific subcommand implementations

4. **Implement** the CLI structure:
   - Entry point: `src/main.rs`
   - Commands module: `src/commands/mod.rs`
   - Subcommand implementations: `src/commands/serve.rs`, `src/commands/process.rs`
   - Error handling: `anyhow::Result<()>`

5. **Add** observability:
   - Configure `tracing-subscriber` with `RUST_LOG` parsing
   - Replace all `println!`/`eprintln!` with `tracing::info!`/`tracing::error!`

---

## Quality Requirements

These are non-negotiable. The deliverable is only accepted when all pass:

- **No `.unwrap()` or `.expect()` in production code** — use `?` + `anyhow`
- **No `println!`/`eprintln!`** — use `tracing` only
- **All `unsafe` blocks have `SAFETY` comments**
- **MSRV: 1.85.0** (Rust 2024 edition)
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

cargo build --release
echo "=== BUILD: PASS ==="
```

---

## Deliverables

1. **Project structure** with:
   - `Cargo.toml` — with name, version, description, authors, MSRV
   - `README.md` — with build commands, usage, env vars
   - `.gitignore`, `rustfmt.toml`, `.clippy.toml`
   - `src/main.rs`, `src/commands/mod.rs`, `src/commands/serve.rs`, `src/commands/process.rs`
   - `src/lib.rs`, `tests/basic.rs`

2. **Validation output** — copy the terminal output of each validation command

3. **Summary** — 3–5 sentences describing what was created and how to run it

---

## Anti-Patterns That Fail Code Review

- `.unwrap()` in any non-test source file
- `println!`/`eprintln!` in production code
- `unsafe` without `SAFETY` comment
- `anyhow::Error` in a library crate
- MSRV lower than 1.85 without documented justification
