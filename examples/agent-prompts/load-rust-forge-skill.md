# Agent Prompt: Load and Use rust-forge-skill

Give this entire prompt to any coding agent to make it load and use `rust-forge-skill` for a Rust task.

---

## Copy-Paste Prompt

```
You are a coding agent. A user wants you to work on a Rust project.

Your task: {DESCRIBE_YOUR_TASK_HERE}

To do this job correctly, you MUST use the rust-forge-skill skill pack.

## Step 1 — Find and read the skill

Look for rust-forge-skill in your environment. It should contain a file called SKILL.md.

Common locations to search:
  - ./rust-forge-skill/SKILL.md
  - ~/.claude/skills/rust-forge-skill/SKILL.md
  - /skills/rust-forge-skill/SKILL.md
  - {SKILL_ROOT}/SKILL.md

Once found, read the entire SKILL.md file.

## Step 2 — Identify what you need to do

The SKILL.md file has a decision tree. Use it to determine:
- What kind of Rust project you are working with
- Which template to use (if scaffolding)
- Which guides to read before writing code
- Which quality gates must pass

## Step 3 — Read the relevant guides

Before writing any code, read these files from the skill pack:
- guides/00-agent-operating-model.md
- guides/01-project-architecture.md
- guides/03-error-handling-anyhow-thiserror.md
- Any guide that matches your project type (e.g., guides/04-async-tokio-axum.md for an API)
- guides/12-rust-anti-patterns.md

## Step 4 — Do the work

Scaffold, audit, refactor, or fix the Rust project as described in the agent-loader-instructions.md file.

Use the correct template from templates/ directory.
Use the checklists in checklists/ directory to check your work.
Use the scripts in scripts/ directory to validate your work.

## Step 5 — Quality gates

Before you say anything is "done", you MUST run these commands and show me the output:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

If this is a library project, also run:
```bash
cargo doc --workspace --all-features --no-deps
./scripts/check_msrv.sh
```

If this is a FFI or WASM project, also run:
```bash
bash ./scripts/audit_unsafe.sh
```

## Step 6 — Final report

Produce a final report with:
1. Summary (3–5 sentences: what was done, how, what was produced)
2. Files changed (table: path, change, reason)
3. Commands run (table: command, output, PASS/FAIL)
4. Risks (what is not covered, what needs attention)
5. Next steps (what the user or next agent should do)
6. Verdict: READY | READY_WITH_LIMITATIONS | NOT_READY

## Rules you must follow

- Read SKILL.md before writing any code
- Use a template, do not write project code from scratch
- No .unwrap() in production code — use ? and anyhow
- No println! in production code — use tracing
- No unsafe without a SAFETY comment
- Run validation commands and show real output
- Do not fake or skip test results
- Report actual risks, do not pretend everything is perfect

If you cannot find rust-forge-skill, search the filesystem for SKILL.md files and check if any contain "rust-forge-skill".
```

---

## Customisation

Replace `{DESCRIBE_YOUR_TASK_HERE}` with the actual task. Examples:

```
You are a coding agent. A user wants you to work on a Rust project.

Your task: Scaffold a new CLI tool project called "mytool" with subcommands serve and process

To do this job correctly, you MUST use the rust-forge-skill skill pack.
[... rest of prompt ...]
```

```
Your task: Refactor an existing Rust library that uses anyhow everywhere to use thiserror for domain errors, eliminate all .unwrap() calls, and add proper error variants.

To do this job correctly, you MUST use the rust-forge-skill skill pack.
[... rest of prompt ...]
```

```
Your task: Audit the unsafe code in an FFI wrapper crate and produce a SAFETY audit report with fixes.

To do this job correctly, you MUST use the rust-forge-skill skill pack.
[... rest of prompt ...]
```

---

## What This Produces

When given to an agent that follows these instructions, you get:

1. **Correct project type selection** — agent picks the right template
2. **Idiomatic code** — no anti-patterns, proper error handling, layered architecture
3. **Validated output** — all quality gates run and reported
4. **Audit trail** — every file changed is explained
5. **Honest verdict** — READY only when it genuinely passes

---

## Variations

### Minimal Version (for experienced agents)

If the agent already knows `rust-forge-skill` and just needs a reminder:

```
Use rust-forge-skill (SKILL.md at {path}) to complete this task:
{TASK}
Read SKILL.md, identify project type, read matching guides, scaffold/refactor,
run quality gates, produce final report.
No .unwrap() in production. No unsafe without SAFETY comment.
Run: cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace --all-features
```

### Security-Focused Version (for security audits)

Add before the final rules section:

```
This task is a SECURITY audit. Additionally:
- Run cargo audit and cargo deny check advisories licenses ban
- Use checklists/security-review-checklist.md as the audit rubric
- Run bash ./scripts/audit_unsafe.sh (for FFI/WASM)
- Every unsafe block must have a SAFETY comment
- Check for secrets in source code and logs
```

### Library Author Version (for crate development)

Add before the final rules section:

```
This task is a LIBRARY project. Additionally:
- Use thiserror for all domain errors (NOT anyhow in library code)
- Run cargo doc --workspace --all-features --no-deps
- Run ./scripts/check_msrv.sh
- All public API items must have doc comments
- MSRV is 1.85.0 unless otherwise specified
```
