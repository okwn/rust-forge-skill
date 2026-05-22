# Self-Test Plan for Rust Forge Skill Pack

## Purpose

This document describes how the self-test scenarios verify that the Rust Forge skill pack actually works when given to an AI agent.

## What Is Being Tested

The self-tests prove that an AI agent can:

1. **Read and follow** the SKILL.md guidance
2. **Use the decision tree** to select the correct template
3. **Apply guide knowledge** to complete tasks (e.g., guides 03 + 12 for refactoring)
4. **Enforce anti-patterns** (no `.unwrap()`, `unsafe` with SAFETY comments)
5. **Run validation commands** and report actual output
6. **Produce properly structured reports** matching the expected format

## Test Verification Strategy

Each scenario uses a **report shape validation** approach:

```
Scenario → Agent → Report → Compare against Expected Shape
                              ↓
                     All required sections present?
                     All required commands run?
                     Required structure maintained?
                              ↓
                        PASS / FAIL
```

This is **not** testing that the agent produces identical output — it's testing that the agent follows the correct process and produces structurally valid reports.

## Scenario Coverage

| Scenario | What It Validates |
|----------|-------------------|
| 01-scaffold-cli | Template selection (cli-app), command addition (guide 06), basic validation |
| 02-scaffold-axum-api | Template selection (axum-api), versioned routes, API testing |
| 03-refactor-bad-library | Guide application (03 + 12), anti-pattern enforcement, error handling |
| 04-audit-unsafe-ffi | Guide application (04 + 11), unsafe audit methodology, safe wrapper design |
| 05-create-workspace-service | Guide application (05 + 09), workspace structure, domain-driven design |

## What Each Scenario Tests

### 01 - Scaffold CLI
- Agent selects `cli-app` template (correct project type identification)
- Agent adds a command following guide 06
- Agent runs `cargo build`, `cargo test`, `cargo clippy`
- Agent produces a report with all 6 required sections

### 02 - Scaffold Axum API
- Agent selects `axum-api` template
- Agent adds a versioned route `/v1/ping` (not just `/ping`)
- Agent writes tests for the new endpoint
- Agent runs full validation suite
- Agent produces a report with all 6 required sections

### 03 - Refactor Bad Library
- Agent identifies all `unwrap()` calls in the bad code
- Agent replaces them with proper error handling (thiserror)
- Agent splits code into proper modules
- Agent adds unit tests
- Agent produces a report with problems identified + solutions applied

### 04 - Audit Unsafe FFI
- Agent documents every `unsafe` block with SAFETY comments
- Agent identifies undefined behavior risks
- Agent creates a safe wrapper (no raw pointers in public API)
- Agent implements `Drop` for automatic cleanup
- Agent produces a report with audit findings + safe wrapper design

### 05 - Create Workspace Service
- Agent creates a proper workspace with multiple crates
- Agent defines domain models and value objects
- Agent implements three domain operations with business rules
- Agent runs `cargo build --workspace` successfully
- Agent produces a report with workspace structure and domain model

## Expected Report Shapes

Each expected report shape file (in `expected-reports/`) defines:

### Required Sections
1. **Summary** — What was done, how, outcome
2. **Files Changed** — Table with File + Change columns
3. **Commands Run** — Table with Command + Result + Output Summary
4. **Risks** — At least one identified risk
5. **Next Steps** — At least two actionable items
6. **Final Verdict** — READY / READY_WITH_LIMITATIONS / NOT_READY

### Failure Conditions
Each expected shape also defines failure conditions:
- Missing required sections → FAIL
- Wrong format (wrong table columns, etc.) → FAIL
- Empty content in required sections → FAIL
- False claims (claims PASS but actually FAILED) → FAIL
- Missing required commands → FAIL

## Manual Test Execution

```bash
# 1. Start with a fresh test directory
mkdir -p /tmp/rust-forge-test
cd /tmp/rust-forge-test

# 2. Give the agent the skill pack (SKILL.md content)
# 3. Give the agent the scenario (scenario file content)
# 4. Agent produces report.md
# 5. Validate report.md against expected-reports/XX-*-report-shape.md

# Validation checklist:
# - All required sections present?
# - Table columns correct?
# - Commands actually run and passed?
# - Final verdict matches actual results?
```

## CI Integration

The self-tests can be run in CI using `ci/self-test.yml`:

```bash
# Run a specific scenario
gh workflow run self-test.yml -f scenario=01-scaffold-cli

# Run all scenarios
for s in 01-scaffold-cli 02-scaffold-axum-api 03-refactor-bad-library 04-audit-unsafe-ffi 05-create-workspace-service; do
  gh workflow run self-test.yml -f scenario=$s
done
```

## How This Validates the Skill Pack

### Without Self-Tests
You might believe the skill pack works because:
- The guides look correct
- The templates look correct
- You tested it yourself once

But you don't actually know if an agent can follow the guidance.

### With Self-Tests
You **prove** the skill pack works because:
- Each scenario gives an agent the same materials you have
- The agent independently completes the task
- The output is validated against a known-good shape
- Real code is built, tested, and reported

## Maintenance

Update self-tests when:
- SKILL.md changes — scenarios may need adjustment
- Guides are updated — expected report shapes may need adjustment
- Templates change — scenario instructions may need adjustment
- New capabilities added — new scenarios added

## Test Limitations

These self-tests verify:
- ✅ Agent can follow the skill guidance
- ✅ Agent can produce structurally correct reports
- ✅ Agent can run and report validation commands
- ✅ Agent can apply guide knowledge to code

These self-tests do NOT verify:
- ❌ The generated code is semantically correct (requires human review)
- ❌ Edge cases in the generated code (requires property-based testing)
- ❌ Performance of generated code (requires benchmarking)
- ❌ Security of generated code (requires dedicated security audit)