# Scripts

This directory contains production-ready scripts for Rust project maintenance.

## validate_rust_project.sh

Run a strict validation suite against any Rust project path.

```bash
./scripts/validate_rust_project.sh [path]
```

**What it checks:**
- Required commands: `cargo`, `rustc`, `rustfmt`
- `cargo fmt --all -- --check` — formatting
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — lints
- `cargo test --workspace --all-features` (or `cargo nextest run` if available)
- `cargo audit` if `cargo-audit` is installed
- `cargo deny check` if `cargo-deny` is installed

**Exit codes:**
- `0` — all checks passed
- `1` — one or more checks failed

---

## generate_cargo_workspace.sh

Generate a clean Rust workspace skeleton.

```bash
./scripts/generate_cargo_workspace.sh <name>
```

**Creates:**
```
<name>/
├── Cargo.toml              (workspace root, workspace.package, workspace.dependencies)
├── rust-toolchain.toml     (stable channel)
├── rustfmt.toml           (2024 edition)
├── .gitignore
├── .clippy.toml
├── crates/
│   ├── domain/             (core business logic)
│   ├── infra/             (database, external services)
│   └── api/               (HTTP / API layer)
├── apps/
│   └── server/             (binary)
└── xtask/                  (build helper tasks)
```

**Validation:** Runs `cargo check --workspace` to verify the generated workspace compiles.

---

## audit_unsafe.sh

Find unsafe usage and produce an audit checklist.

```bash
./scripts/audit_unsafe.sh [path]
```

**Searches for:**
- `unsafe { }` blocks
- `extern "C"` declarations
- `#[no_mangle]` attributes
- `#[repr(C)]` structs
- `transmute`, `from_raw`, `into_raw`, `get_unchecked`, `unwrap_unchecked`
- `static mut` declarations
- `.unwrap()` / `.expect()` in production code
- `println!` / `eprintln!` in production code

**Output:** Creates `unsafe-audit-report.md` in the target directory with:
- Inventory table
- Missing SAFETY comment list
- FFI boundary checklist
- Static mut findings
- Reviewer sign-off section

**Exit codes:**
- `0` — no unsafe issues found
- `1` — unsafe issues found (report written for review)

---

## check_msrv.sh

Check MSRV consistency between declared and actual toolchain.

```bash
./scripts/check_msrv.sh [path]
```

**What it reads:**
- `rust-version` field from `[package]` in `Cargo.toml` (required)
- `channel` from `rust-toolchain.toml` (informational)
- `RUST_MSRV` environment variable (override)

**What it does:**
- Runs `cargo check --workspace --all-features`
- Clearly warns when MSRV cannot be fully proven (no pinned toolchain)

**Important:** Without a pinned `rust-toolchain.toml` (e.g., `channel = "1.85"`), this script uses the current toolchain and cannot fully prove MSRV. It only proves the project compiles with the current toolchain, not with the declared MSRV.

**Exit codes:**
- `0` — check passed
- `1` — `rust-version` missing or `cargo check` failed

---

## run-self-tests.sh

Run self-test scenarios to validate the Rust Forge skill pack.

```bash
./scripts/run-self-tests.sh [scenario]
```

**Arguments:**
- `scenario` (optional) — Run specific scenario: `01-scaffold-cli`, `02-scaffold-axum-api`, `03-refactor-bad-library`, `04-audit-unsafe-ffi`, `05-create-workspace-service`. Default: all

**Options:**
- `--skip-validation` — Skip report validation step
- `--clean` — Clean test artifacts before running

**What it does:**
1. Reads scenario file from `self-tests/scenarios/`
2. Validates expected report shape exists
3. Runs structure validation (checks all required sections)
4. Can validate a report if one exists at `report.md` in test workspace

**Exit codes:**
- `0` — all validations passed
- `1` — one or more validations failed

---

## validate-self-test-report.sh

Validate an agent's `report.md` against the expected shape for a scenario.

```bash
./scripts/validate-self-test-report.sh <scenario> [report_file]
```

**Arguments:**
- `scenario` — Scenario name (e.g., `01-scaffold-cli`)
- `report_file` — Path to report.md (default: `./report.md`)

**What it validates:**
- All 6 required sections present (Summary, Files Changed, Commands Run, Risks, Next Steps, Final Verdict)
- Table structure correct (| File | Change | and | Command | Result |)
- Final Verdict is valid value (READY/READY_WITH_LIMITATIONS/NOT_READY)
- Required commands appear in report
- PASS/FAIL results reported

**Exit codes:**
- `0` — validation passed
- `1` — validation failed

---

## Testing the scripts

From the project root:

```bash
# Validate current project
./scripts/validate_rust_project.sh .

# Generate a new workspace
./scripts/generate_cargo_workspace.sh my-workspace

# Audit unsafe in current project
./scripts/audit_unsafe.sh .

# Check MSRV
./scripts/check_msrv.sh .
```