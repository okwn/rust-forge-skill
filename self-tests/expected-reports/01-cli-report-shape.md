# Expected Report Shape: 01-cli-report-shape.md

This file defines the required structure and content for a passing report from Scenario 01.

## Summary

Must contain:
- What was done (scaffolded CLI app + added greet command)
- How it was done (using templates/guide 01 and guide 06)
- Outcome (success/failure)

## Files Changed

| File | Change |
|------|--------|
| Cargo.toml | Initial project manifest with clap dependency |
| src/main.rs | Main entry point with CLI setup and greet command |

## Commands Run

| Command | Result | Output Summary |
|---------|--------|----------------|
| cargo build | PASS/FAIL | [summary] |
| cargo run greet --help | PASS/FAIL | [summary] |
| cargo run greet --name World | PASS/FAIL | [summary] |
| cargo run greet --name X --formal | PASS/FAIL | [summary] |
| cargo test | PASS/FAIL | [summary] |
| cargo clippy | PASS/FAIL | [summary] |

## Risks

Must contain at least one identified risk or limitation.

## Next Steps

Must contain at least two actionable next steps.

## Final Verdict

Must be one of: `READY` / `READY_WITH_LIMITATIONS` / `NOT_READY`

## Failure Conditions

A report fails validation if:
1. Missing required section
2. Wrong table columns in Files Changed or Commands Run
3. Empty content in required sections
4. False claims about command results
5. Missing required commands