# Contributing to rust-forge-skill

Thank you for your interest in improving `rust-forge-skill`. This document explains the standards, process, and structure for contributing to this skill pack.

---

## What Can Be Contributed

| Type | Examples |
|---|---|
| **Guides** | New guides for new Rust patterns (e.g., async streams, custom allocators) |
| **Templates** | New project templates (e.g., TUI app, gRPC service, no_std embedded) |
| **Scripts** | New validation scripts (e.g., `check_api_compliance.sh`) |
| **Checklists** | New checklists (e.g., "Performance Review Checklist") |
| **Agent Prompts** | New agent prompts for common workflows |
| **Bug fixes** | Fix incorrect code examples, broken links, outdated tool names |
| **CI improvements** | Add new CI stages, improve caching, add more runners |

---

## Contribution Standards

### Every File Must

- [ ] Use `rust-forge-skill` as the skill name (never variants)
- [ ] Reference only files that exist in the same version
- [ ] Be self-contained (an agent reading only this file should understand what to do)
- [ ] Avoid generic filler text — be specific and actionable
- [ ] Follow the existing directory structure

### Adding a Guide

1. Create `guides/XX-name-of-guide.md` (use a 2-digit prefix for ordering)
2. The guide must contain:
   - **Purpose** — what problem it solves
   - **When to use** — which project types or tasks need this guide
   - **Core patterns** — idiomatic code examples with annotations
   - **Anti-patterns** — what to avoid and why
   - **Validation** — how to verify correctness
   - **Agent notes** — any agent-specific caveats
3. Add it to the `guides[]` array in `mcp/manifest.example.json`
4. Reference it in `SKILL.md` and `mcp/tool-contract.md` where appropriate
5. Ensure `cargo fmt` and `shellcheck` pass (if shell script)

### Adding a Template

1. Create `templates/<template-name>/`
2. Required files:
   - `Cargo.toml` — with `edition = "2024"`, `rust-version = "1.85"`
   - `README.md` — build commands, usage, env vars
   - `AGENT_NOTES.md` — template-specific agent instructions
   - `rustfmt.toml`, `.clippy.toml`
   - `src/` with at least `main.rs` or `lib.rs`
   - `tests/` with at least one coherent test
3. All `{{placeholder}}` strings in source files must be documented in `AGENT_NOTES.md`
4. Test must use `{{crate_name}}` not a hardcoded name
5. Add to `templates[]` in `mcp/manifest.example.json`

### Adding a Script

1. Create `scripts/<script-name>.sh`
2. Requirements:
   - Executable (`chmod +x`)
   - `#!/usr/bin/env bash`
   - `set -euo pipefail`
   - `--help` / `-h` flag with usage
   - Graceful handling of optional tools (warn + skip, don't hard-fail)
   - Explicit `exit 1` on failure with a descriptive message
   - Descriptive `pass()` and `warn()` helper functions
3. Add to `scripts[]` in `mcp/manifest.example.json`
4. Document in `scripts/README.md`
5. If shellcheck is available, fix all warnings before submitting

### Adding a Checklist

1. Create `checklists/<checklist-name>.md`
2. Requirements:
   - Binary checkbox format (`[ ]` / `[x]`)
   - Grouped by phase or category
   - Each item must be actionable (not "consider X" — either do it or mark N/A)
   - Quality gate command table at the bottom
3. Add a row to the "When to Use" table in `README.md`

### Adding an Agent Prompt

1. Create `examples/agent-prompts/<prompt-name>.md`
2. Requirements:
   - Direct copy-pasteable (no agent preprocessing needed)
   - Reads `SKILL.md` first (instruction in prompt)
   - Enforces quality gates with exact commands
   - Requires a final report with verdict
   - Lists anti-patterns that fail review
3. Update `README.md` prompt table

### Version Bump

If your contribution changes the skill's protocol or manifest:

1. Update `VERSION` file (semver: `major.minor.patch`)
2. Add entry to `CHANGELOG.md`
3. If protocol version changes: update `agent_contract_version` in `manifest.example.json`

---

## Quality Expectations

Before submitting a contribution:

```bash
# Formatting
cargo fmt --all -- --check   # for any embedded Rust code

# Shell scripts
shellcheck scripts/*.sh       # must have zero warnings

# Markdown links
# Manually verify all relative links work:
# - guides/ references in SKILL.md
# - scripts/ references in README.md
# - templates/ references in checklists
```

Every guide must be **agent-instructive** — not just conceptual explanation. An agent reading only this guide should be able to complete the task correctly.

---

## File Structure Reference

```
rust-forge-skill/
├── SKILL.md                           # Primary agent entrypoint
├── README.md                          # Human-facing overview
├── VERSION                           # Semantic version
├── CHANGELOG.md                      # Change history
├── CONTRIBUTING.md                   # This file
├── SECURITY.md                       # Security reporting policy
├── deny.toml                         # cargo-deny config
├── guides/                           # One file per guide
│   └── XX-name.md
├── templates/                         # One directory per template
│   └── <template>/
│       ├── Cargo.toml
│       ├── README.md
│       ├── AGENT_NOTES.md
│       ├── src/
│       └── tests/
├── scripts/                           # One file per script
│   └── name.sh
├── ci/                               # GitHub Actions workflows
├── checklists/                       # Markdown checklists
├── examples/agent-prompts/          # Agent prompt templates
├── mcp/                              # MCP layer
│   ├── manifest.example.json
│   ├── tool-contract.md
│   ├── agent-loader-instructions.md
│   └── README.md
└── reports/                          # Release readiness reports
    └── RELEASE_READINESS_X_Y_Z.md
```

---

## Process

1. **Fork** the repository (or create a branch if you have push access)
2. **Create a branch**: `git checkout -b feat/new-guide-name`
3. **Make your changes** following the standards above
4. **Test locally** — run shellcheck, verify links, ensure code examples compile
5. **Submit a PR** with a description of what was added/changed and why

For bugs: open an issue first with the label `bug`. For new features: open an issue with `enhancement` before starting work.

---

## Not Accepted

- Contributions that add dependencies without clear justification
- Guides that explain concepts without giving actionable agent instructions
- Templates that use older Rust editions without documented justification
- Any contribution that introduces `.unwrap()` in template production code
- Any contribution that introduces `unsafe` blocks without SAFETY comments
