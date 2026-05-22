# Agent Loader Instructions — rust-forge-skill

These are step-by-step instructions for any AI coding agent that needs to load and use `rust-forge-skill` to complete a Rust-related task.

Follow these steps **in order**. Skipping steps produces non-conforming output.

---

## Before You Start

You are an AI coding agent. Your task involves Rust — scaffolding, auditing, refactoring, or validating a Rust project. `rust-forge-skill` is the skill pack that makes you produce idiomatic, production-grade Rust code.

The skill is located at a path we'll call `SKILL_ROOT`. Before writing any code, load the skill.

---

## Loading Sequence (7 Steps)

### Step 1 — Read SKILL.md

**Do this first.** Do not write any code, do not make any plans, do not ask questions.

```bash
# Locate and read the skill entrypoint
# Assumes SKILL_ROOT is set, or use the relative path from the task context
cat {SKILL_ROOT}/SKILL.md
# Or if no path is given, look for it:
find . -name "SKILL.md" -path "*/rust-forge-skill/SKILL.md" 2>/dev/null | head -1
```

Read the entire `SKILL.md`. It contains:
- Decision trees for project type selection
- Forbidden anti-patterns (zero-tolerance rules)
- Quality gates that must be enforced
- Module structure conventions

**Confirm you have read it** by summarizing its key constraints in your own words before proceeding.

---

### Step 2 — Identify Project Type

Use `SKILL.md`'s decision tree to determine the project type:

| Project Type | Indicators | Template |
|---|---|---|
| `cli` | "CLI", "command-line", "tool", "bin" | `templates/cli-app/` |
| `api` | "HTTP", "REST", "API", "server", "axum", "web" | `templates/axum-api/` |
| `library` | "crate", "library", "publish to crates.io", "sdk" | `templates/library-crate/` |
| `ffi` | "FFI", "C interop", "bindgen", "C ABI", "C++" | `templates/ffi-wrapper/` |
| `wasm` | "WASM", "WebAssembly", "wasm-bindgen", "browser" | `templates/wasm-module/` |
| `workspace` | "workspace", "multi-crate", "monorepo" | `templates/workspace-service/` |

If the task is an **audit** or **refactor** (not scaffolding), the project type is whatever already exists at `project_path`.

For audits, run inspection commands:

```bash
cd {project_path}
ls -la Cargo.toml 2>/dev/null || { echo "NO_CARGO_TOML"; exit 1; }
cargo metadata --format-version=1 --no-deps 2>/dev/null | jq -r '.packages[].name' 2>/dev/null | head -20
grep -rn "unsafe {" --include="*.rs" src/ 2>/dev/null | wc -l
grep -rn "\.unwrap()" --include="*.rs" src/ 2>/dev/null | wc -l
cat Cargo.toml | grep -E "^name|^version|^edition"
```

---

### Step 3 — Read the Matching Guide(s)

Based on the project type identified in Step 2, read the relevant guides **before** writing any code:

| Project Type | Required Guides |
|---|---|
| `cli` | `guides/05-cli-clap-tracing.md`, `guides/03-error-handling-anyhow-thiserror.md` |
| `api` | `guides/04-async-tokio-axum.md`, `guides/01-project-architecture.md`, `guides/03-error-handling-anyhow-thiserror.md` |
| `library` | `guides/03-error-handling-anyhow-thiserror.md`, `guides/12-rust-anti-patterns.md`, `guides/01-project-architecture.md` |
| `ffi` | `guides/07-ffi-c-cpp.md`, `guides/08-security-unsafe-audit.md` |
| `wasm` | `guides/09-wasm-module-patterns.md`, `guides/08-security-unsafe-audit.md` |
| `workspace` | `guides/02-crate-workspace-patterns.md`, `guides/01-project-architecture.md` |

For **all project types**, also read:
- `guides/00-agent-operating-model.md` — your operating contract with the user
- `guides/10-release-ci-msrv.md` — CI setup and MSRV policy

```bash
cat {SKILL_ROOT}/guides/01-project-architecture.md
cat {SKILL_ROOT}/guides/03-error-handling-anyhow-thiserror.md
# etc.
```

Take notes on the specific patterns required for this project type. Do not proceed to Step 4 until you can describe the architecture in one sentence.

---

### Step 4 — Choose and Customize Template

Do not write project code from scratch. Use the template.

```bash
# List available templates
ls {SKILL_ROOT}/templates/

# Copy the correct template
TEMPLATE_TYPE="cli-app"   # replace with your project type
TARGET_PATH="/path/to/my-project"
cp -r {SKILL_ROOT}/templates/$TEMPLATE_TYPE "$TARGET_PATH"

# Customize template fields
cd "$TARGET_PATH"
# Update these fields in Cargo.toml:
#   name        = "{{kebab-case-name}}"
#   version     = "0.1.0"
#   description = "{{one-line summary}}"
#   authors     = ["Name <email>"]
#   license     = "MIT OR Apache-2.0"
#   rust-version = "1.85"
```

Replace all `{{placeholder}}` strings in the copied template:

```bash
# Find all placeholders
grep -rn "{{" "$TARGET_PATH/" --include="*.rs" --include="*.toml" --include="*.md"
# Replace each one
sed -i 's/{{crate_name}}/my-crate/g' "$TARGET_PATH/Cargo.toml"
sed -i 's/{{project_name}}/my-crate/g' "$TARGET_PATH/README.md"
# ... etc.
```

Run `cargo generate-lockfile` after customizing.

---

### Step 5 — Apply Checklists

Before marking anything complete, verify against the relevant checklist:

| Task | Checklist |
|---|---|
| Starting a new project | `checklists/project-start-checklist.md` |
| Code review / before merge | `checklists/code-review-checklist.md` |
| Security-sensitive change | `checklists/security-review-checklist.md` |
| Release / publish | `checklists/release-checklist.md` |

```bash
cat {SKILL_ROOT}/checklists/code-review-checklist.md
# Walk through every item. Mark each [ ] as done or N/A.
# If any item cannot be marked done, that is a finding — report it.
```

For audits: use `checklists/code-review-checklist.md` as the audit rubric. For each checklist item, state whether it PASS, FAIL, or N/A.

---

### Step 6 — Run Scripts

Run the validation scripts in the correct order:

```bash
cd {project_path}

# 1. Full quality gate (always)
bash {SKILL_ROOT}/scripts/validate_rust_project.sh

# 2. MSRV check (libraries only)
bash {SKILL_ROOT}/scripts/check_msrv.sh

# 3. Unsafe audit (FFI/WASM only)
bash {SKILL_ROOT}/scripts/audit_unsafe.sh
```

If any script fails, **stop**. Report the failure. Do not proceed.

The scripts are the authoritative quality gates. If a script says FAIL, the verdict is `NOT_READY` regardless of what the agent thinks.

---

### Step 7 — Return Final Report

Produce a structured final report matching the `SkillReport` output schema from `tool-contract.md`.

```
# Final Report: {project_name}

## Summary
[3–5 sentences: what was done, why, what was produced]

## Files Changed
| Path | Change | Reason |
|---|---|---|
| src/main.rs | created | CLI entry point with clap |
| src/cli.rs | created | Argument parsing |
| ... | ... | ... |

## Commands Run
| Command | Output | Verdict |
|---|---|---|
| cargo fmt --all -- --check | No diff | PASS |
| cargo clippy --workspace -- -D warnings | Finished | PASS |
| cargo test --workspace --all-features | 12 passed | PASS |

## Risks
- MSRV check not run (application, not library)
- No integration tests (missing from template — add before v1.0)

## Next Steps
1. Add integration tests for the repository layer
2. Set up CI with {SKILL_ROOT}/ci/github-actions-rust.yml
3. Run cargo publish --dry-run before first publish

## Verdict
READY | READY_WITH_LIMITATIONS | NOT_READY
```

---

## Quick Reference

### Command Aliases

```bash
# Fast: run all quality gates
cd {project_path}
cargo fmt --all -- --check \
  && cargo clippy --workspace --all-targets --all-features -- -D warnings \
  && cargo test --workspace --all-features \
  && echo "=== ALL GATES PASSED ==="

# For libraries: add doc + MSRV
cargo doc --workspace --all-features --no-deps && ./scripts/check_msrv.sh

# For FFI/WASM: add unsafe audit
bash ./scripts/audit_unsafe.sh
```

### Template Quick Select

```
cli      → templates/cli-app/
api      → templates/axum-api/
library  → templates/library-crate/
ffi      → templates/ffi-wrapper/
wasm     → templates/wasm-module/
workspace → templates/workspace-service/
```

### Anti-Patterns (Zero Tolerance)

```
NO .unwrap() / .expect() in non-test source
NO println! / eprintln! in production
NO unsafe without SAFETY comment
NO anyhow::Error in library crates
NO static mut
```

### Error Model by Project Type

```
Library  → thiserror (domain errors)
CLI/API  → anyhow (application boundary)
```

### MSRV

```
Default: 1.85.0 (Rust 2024 edition)
Override: RUST_MSRV=1.82.0 ./scripts/check_msrv.sh
```

---

## Troubleshooting

| Problem | Solution |
|---|---|
| "No such file or directory: SKILL.md" | Set SKILL_ROOT env var or find the skill directory first |
| Template not found | Check project type is one of: cli, api, library, ffi, wasm, workspace |
| `cargo generate-lockfile` fails | Run `cargo fetch` first, then retry |
| `check_msrv.sh` fails | The toolchain at MSRV cannot build the project — raise MSRV or remove incompatible deps |
| `audit_unsafe.sh` finds undocumented unsafe | Add SAFETY comment, then re-run |
| Clippy fails with "unknown lint" | Update rust-toolchain to stable or 1.85+ |
| `cargo doc` fails with broken links | Fix or remove broken doc links before proceeding |
