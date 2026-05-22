# Tool Contract — rust-forge-skill Agent Protocol

This document defines the **formal contract** between a calling agent and the `rust-forge-skill` skill. Any agent that implements this contract is guaranteed to produce conforming output.

Agents **must** implement all fields in this contract. Skipping a field is a protocol violation.

---

## Protocol Version

`rust-forge-skill-v1` — versioned to allow future incompatible changes.

All manifests and loader instructions reference this protocol version.

---

## Input Schema

The calling agent (or MCP server routing a task) must provide the following fields:

```typescript
interface SkillInput {
  // ─── Project Identification ────────────────────────────────────────────────
  project_path:   string;    // Absolute or relative path to the target project
  project_type:   "cli" | "api" | "library" | "ffi" | "wasm" | "workspace";

  // ─── Project Metadata ──────────────────────────────────────────────────────
  project_name?:  string;    // Desired crate name (kebab-case). Auto-detected if omitted.
  msrv?:          string;    // Minimum Supported Rust Version (e.g. "1.85.0"). Default: "1.85.0"
  edition?:       "2024";    // Rust edition. Default: "2024"
  async_runtime?: "tokio";   // Async runtime. Default: "tokio" for api/cli, none for library

  // ─── Task Constraints ─────────────────────────────────────────────────────
  constraints?: {
    no_unsafe?:       boolean;  // Reject any task that would introduce unsafe code
    no_async?:       boolean;  // Reject async; use blocking I/O only
    max_dependencies?: number; // Fail if dependency count exceeds this number
    msrv_override?:   string;  // Force a specific MSRV
  };

  // ─── Output Control ────────────────────────────────────────────────────────
  output_mode: "full" | "minimal"; // full = complete report, minimal = just verdict
  audit_scope?: "quick" | "deep";  // quick = anti-patterns only, deep = full checklist
}
```

### Input Field Requirements

| Field | Required | Default | Notes |
|---|---|---|---|
| `project_path` | **Yes** | — | Must be writable if scaffolding |
| `project_type` | **Yes** | — | Determines template and guide selection |
| `project_name` | No | Derived from `Cargo.toml` | Used for template customization |
| `msrv` | No | `"1.85.0"` | Must match a valid Rust toolchain |
| `edition` | No | `"2024"` | Must be a valid Rust edition |
| `async_runtime` | No | `"tokio"` for api/cli | Ignored for library type |
| `constraints` | No | `{}` | Used for safety filtering |
| `output_mode` | No | `"full"` | `minimal` suppresses verbose output |
| `audit_scope` | No | `"deep"` | `quick` skips some checklist sections |

---

## Actions

The agent must execute tasks in the following **mandatory phases**, in order. Skipping or reordering phases is a protocol violation.

### Phase 1: Inspect

**Goal:** Understand the current state of the project (existing or empty).

```bash
# 1a. Verify project exists at project_path
ls -la {project_path}/Cargo.toml 2>/dev/null && echo "EXISTS" || echo "EMPTY"

# 1b. If project exists, run inspection commands
cd {project_path}
cargo metadata --format-version=1 --no-deps 2>/dev/null | jq -r '.packages[].name'  # list deps
grep -rn "unsafe {" --include="*.rs" src/ 2>/dev/null | head -20  # count unsafe blocks
grep -rn "\.unwrap()" --include="*.rs" src/ 2>/dev/null | wc -l   # count unwraps
grep -rn "SAFETY" --include="*.rs" src/ 2>/dev/null | wc -l        # count SAFETY comments

# 1c. Check CI configuration
ls .github/workflows/ 2>/dev/null || echo "NO_CI"
cat .github/workflows/*.yml 2>/dev/null | grep "cargo audit\|cargo-deny" || echo "NO_SECURITY_CI"
```

**Deliverable:** Inspection report (1–2 sentences) stating what exists and what is missing.

---

### Phase 2: Plan

**Goal:** Select the correct template, guides, and approach before writing any code.

1. **Select template** based on `project_type`:

   | Type | Template |
   |---|---|
   | `cli` | `templates/cli-app/` |
   | `api` | `templates/axum-api/` |
   | `library` | `templates/library-crate/` |
   | `ffi` | `templates/ffi-wrapper/` |
   | `wasm` | `templates/wasm-module/` |
   | `workspace` | `templates/workspace-service/` |

2. **Select guides** based on `project_type` and task:
   - Always: `guides/00-agent-operating-model.md`, `guides/01-project-architecture.md`
   - `cli`: +`guides/05-cli-clap-tracing.md`
   - `api`: +`guides/04-async-tokio-axum.md`, `guides/03-error-handling-anyhow-thiserror.md`
   - `library`: +`guides/03-error-handling-anyhow-thiserror.md`, `guides/12-rust-anti-patterns.md`
   - `ffi`/`wasm`: +`guides/07-ffi-c-cpp.md`, `guides/08-security-unsafe-audit.md`
   - All: +`guides/10-release-ci-msrv.md`

3. **Check constraints**:
   - If `no_unsafe` and task involves `ffi`/`wasm`: abort with explanation
   - If `no_async` and task involves `api`/`cli`: suggest using blocking I/O instead

4. **Verify MSRV** is achievable (no dependency requires newer Rust than declared MSRV)

**Deliverable:** A short plan (3–5 bullet points) describing what will be done and in what order.

---

### Phase 3: Scaffold / Refactor

**Goal:** Produce the actual project or refactored code.

For **scaffolding**:
1. Copy template to `project_path`
2. Update `Cargo.toml` fields: name, version, description, authors, license, `rust-version`
3. Replace all `{{placeholder}}` strings
4. Run `cargo generate-lockfile`

For **refactoring**:
1. Audit existing code (produce issue table)
2. Refactor layer by layer: domain → service → repository → API
3. Add `thiserror` error types to domain layer
4. Replace all `.unwrap()` with `?` propagation
5. Add tests for new error paths

---

### Phase 4: Validate

**Goal:** Run all quality gates and collect evidence.

The agent must run **all applicable validation commands** from the manifest:

```bash
cd {project_path}

# Always run
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features

# Library: also run
cargo doc --workspace --all-features --no-deps
./scripts/check_msrv.sh

# FFI/WASM: also run
bash ./scripts/audit_unsafe.sh

# Security: also run
cargo audit
cargo deny check advisories licenses ban
```

Collect the **full output** of each command — do not truncate. Report PASS/FAIL for each.

---

### Phase 5: Report

**Goal:** Deliver a structured final report to the caller.

The report must contain:

```typescript
interface SkillReport {
  summary:        string;    // 3–5 sentences: what was done and why
  files_changed:  Array<{ path: string; change: string; reason: string }>;
  commands_run:   Array<{ command: string; output: string; verdict: "PASS" | "FAIL" }>;
  risks:          string[];  // Known limitations, areas with low coverage
  next_steps:     string[];  // Actionable next steps for user or next agent
  verdict:        "READY" | "READY_WITH_LIMITATIONS" | "NOT_READY";
}
```

---

## Output Schema

```
┌─────────────────────────────────────────────────────────────┐
│                      FINAL REPORT                           │
├─────────────────────────────────────────────────────────────┤
│  Summary: [3–5 sentences]                                  │
│                                                             │
│  Files Changed:                                              │
│  │ path          │ change    │ reason                       │
│  │───────────────│───────────│───────────────────────────────│
│  │ src/lib.rs   │ refactored│ replaced anyhow with thiserror│
│  │ src/error.rs │ created  │ domain error enum added       │
│                                                             │
│  Commands Run:                                              │
│  │ command                                  │ verdict       │
│  │──────────────────────────────────────────│──────────────│
│  │ cargo fmt --all -- --check               │ PASS         │
│  │ cargo clippy --workspace -- -D warnings │ PASS         │
│  │ cargo test --workspace                  │ PASS         │
│                                                             │
│  Risks:                                                     │
│  • Unsafe audit not run (no unsafe present)                 │
│  • MSRV check skipped (application type)                   │
│                                                             │
│  Next Steps:                                                │
│  1. Run cargo publish --dry-run before publishing           │
│  2. Add integration tests for repository layer             │
│                                                             │
│  Verdict: READY                                            │
└─────────────────────────────────────────────────────────────┘
```

---

## Safety Invariants

Agents **must not** violate these invariants. Violations result in a `NOT_READY` verdict regardless of test results.

### 1. No Destructive Rewrite Without Explicit Request

- Agents must not delete files outside `project_path`
- Agents must not `DROP TABLE`, `TRUNCATE`, or otherwise destroy data
- If a destructive operation is required (e.g., database migration), the agent must:
  1. Describe the operation
  2. Wait for explicit user confirmation
  3. Then proceed

### 2. No Unsafe Without Audit

- If any `unsafe` block is introduced or modified, the agent must:
  1. Add a `SAFETY` comment with invariant, why it holds, and UB consequences
  2. Run `./scripts/audit_unsafe.sh`
  3. Report the audit result in the final report

### 3. No Dependency Bloat

- Before adding a dependency, the agent must:
  1. Check if a stdlib alternative exists
  2. Verify the crate has no known vulnerabilities (`cargo audit`)
  3. Confirm the crate is actively maintained
  4. Add it only to `[dependencies]`, not `[dev-dependencies]` unless it's a test tool

### 4. No Secret Exposure

- No API keys, tokens, passwords, or secrets in:
  - Source code
  - `tracing` log output
  - stdout/stderr
  - Error messages
  - Commit messages
- Use environment variables (`std::env::var`) and `.env` files only

### 5. No Unwitnessed Test Passing

- Agents must run tests and report **actual** output
- Do not simulate, truncate, or selectively report test output
- If a test failure is encountered, report it accurately and stop the pipeline

### 6. No Unvalidated Deploy

- Never mark a project `READY` without running all applicable quality gates
- The verdict must match the actual validation results

---

## Error Handling in the Agent Protocol

If any phase fails, the agent must:

1. **Stop** at the failing phase
2. **Report** the failure with the command that failed and the error output
3. **Proceed** only if the user explicitly asks to continue despite the failure
4. **Output** a final report with `verdict: "NOT_READY"` or `verdict: "READY_WITH_LIMITATIONS"`

---

## Capability Routing

The MCP server or calling agent uses the manifest's `capabilities` array to route tasks:

| Capability ID | Triggers | Skipped Guides |
|---|---|---|
| `rust_project_scaffold` | "scaffold", "create", "new" | — |
| `rust_project_audit` | "audit", "review", "check" | `01-architecture`, `04-async`, `05-cli` |
| `rust_error_refactor` | "refactor errors", "thiserror" | — |
| `rust_async_service` | "async api", "axum" | — |
| `rust_ffi_audit` | "unsafe audit", "ffi" | — |
| `rust_security_validation` | "security", "cargo audit" | — |

---

## Protocol Version History

| Version | Date | Change |
|---|---|---|
| 1.0 | 2026-05 | Initial contract |
