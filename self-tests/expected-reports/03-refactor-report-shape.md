# Expected Report Shape: 03-refactor-report-shape.md

This file defines the required structure and content for a passing report from Scenario 03.

## Summary

Must contain:
- What was done (refactored bad library code)
- How it was done (using guides 03 and 12)
- Outcome (success/failure)

## Problems Identified

Must list all problems found in the original code.

## Refactoring Changes

| Original Problem | Solution Applied |
|-----------------|------------------|
| unwrap() on fallible operations | [error handling approach] |
| Giant function | [how it was split] |
| String errors | [custom error type] |
| No tests | [what tests added] |
| Global file dependency | [how dependency inverted] |

## Files Changed

| File | Change |
|------|--------|
| src/lib.rs | Refactored to use proper error handling |
| Cargo.toml | Added thiserror/anyhow dependency |
| src/error.rs | (if created) Custom error type |
| src/validation.rs | (if created) Validation logic module |

## Commands Run

| Command | Result | Output Summary |
|---------|--------|----------------|
| cargo build | PASS/FAIL | [summary] |
| cargo test | PASS/FAIL | [summary] |
| cargo clippy | PASS/FAIL | [summary] |
| cargo fmt | PASS/FAIL | [summary] |

## Risks

Must contain at least one identified risk or limitation.

## Next Steps

Must contain at least two actionable next steps.

## Final Verdict

Must be one of: `READY` / `READY_WITH_LIMITATIONS` / `NOT_READY`

## Failure Conditions

A report fails validation if:
1. Missing required section
2. unwrap() still present in code
3. No tests (cargo test shows 0 tests)
4. No custom error type
5. False claims about command results
6. Major problems not identified