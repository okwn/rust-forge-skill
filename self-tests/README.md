# Self-Tests for Rust Forge Skill Pack

This directory contains self-test scenarios that prove an AI agent can effectively use the Rust Forge skill pack to complete real development tasks.

## What These Tests Verify

These tests validate that the skill pack actually works when given to an AI agent. They verify:
- The agent can read and follow SKILL.md guidance
- The agent can scaffold projects from templates
- The agent can refactor code following the guides
- The agent can audit unsafe code and produce compliant fixes
- The agent can create workspace services following organizational patterns

## Test Structure

```
self-tests/
├── README.md                  ← This file
├── scenarios/                 ← Copy-pasteable agent tasks
│   ├── 01-scaffold-cli.md
│   ├── 02-scaffold-axum-api.md
│   ├── 03-refactor-bad-library.md
│   ├── 04-audit-unsafe-ffi.md
│   └── 05-create-workspace-service.md
└── expected-reports/          ← Expected output shapes for validation
    ├── 01-cli-report-shape.md
    ├── 02-api-report-shape.md
    ├── 03-refactor-report-shape.md
    ├── 04-unsafe-audit-report-shape.md
    └── 05-workspace-report-shape.md
```

## Running the Tests Manually

### Prerequisites

1. Access to the Rust Forge skill pack files:
   - `rust-forge-skill/SKILL.md`
   - `rust-forge-skill/guides/` (all guides)
   - `rust-forge-skill/templates/` (all templates)
   - `rust-forge-skill/checklists/` (all checklists)

2. A Claude Code session or compatible AI agent interface

3. Ability to create files and run commands in a test workspace directory

### Test Execution Steps

For each scenario:

1. **Prepare the test workspace**:
   ```bash
   mkdir -p /tmp/rust-forge-test
   cd /tmp/rust-forge-test
   ```

2. **Give the agent the skill pack**:
   - Share the full contents of `rust-forge-skill/SKILL.md`
   - Share the relevant guides for that scenario
   - Share the relevant templates directory

3. **Give the agent the scenario task**:
   - Copy the scenario file contents
   - Paste into the agent conversation

4. **Wait for the agent to complete**

5. **Validate the output**:
   - Compare the agent's report against the expected report shape
   - Check that required sections exist
   - Check that required commands were run
   - Run validation commands yourself to verify correctness

6. **Record results**:
   ```bash
   # Document test results
   cp /tmp/rust-forge-test/report.md reports/test-results/$(basename .md .md)-$(date +%Y%m%d).md
   ```

## Scenario Index

| # | Scenario | What It Tests | Duration |
|---|----------|----------------|----------|
| 01 | `01-scaffold-cli.md` | CLI project from template, new command addition | ~5 min |
| 02 | `02-scaffold-axum-api.md` | API project from template, route addition, tests | ~10 min |
| 03 | `03-refactor-bad-library.md` | Code refactoring following guides 03 and 12 | ~15 min |
| 04 | `04-audit-unsafe-ffi.md` | Unsafe code audit and safe wrapping | ~10 min |
| 05 | `05-create-workspace-service.md` | Workspace service creation, domain operations | ~15 min |

## Validation Checklist

For each scenario, verify the agent's report contains:

- [ ] **Summary section** with what was done and why
- [ ] **Files Changed** table with path and change reason
- [ ] **Commands Run** table with command and result (PASS/FAIL)
- [ ] **Risks section** with at least one identified risk
- [ ] **Next Steps section** with actionable follow-ups
- [ ] **Final Verdict** with READY / READY_WITH_LIMITATIONS / NOT_READY

### Failure Conditions

A scenario fails if any of these occur:

1. **Build fails** - `cargo build` returns non-zero exit code
2. **Tests fail** - `cargo test` returns non-zero exit code
3. **Missing required sections** - Any section from the checklist above is absent
4. **Wrong structure** - Files created don't match expected project layout
5. **Skill guidance ignored** - Agent should have used a guide but didn't

## Interpreting Results

| Result | Meaning |
|--------|---------|
| ✅ PASS | Agent completed task, all validations pass |
| ⚠️ PARTIAL | Task completed but with minor issues (document them) |
| ❌ FAIL | Task not completed, or critical validations failed |

## Test Maintenance

When to update these tests:

- **SKILL.md changes** - Ensure scenarios still align with skill guidance
- **Guide updates** - Update scenario expectations to match new guidance
- **Template changes** - Update scenario instructions to reflect new templates
- **New scenarios** - Add new scenario files and corresponding expected shapes

## CI Integration

These tests can be automated using the `ci/self-test.yml` workflow.

Run manually:
```bash
cd rust-forge-skill
./scripts/run-self-tests.sh
```

Run in CI:
```bash
gh workflow run self-test.yml -f scenario=01-scaffold-cli
```