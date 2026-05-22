# Expected Report Shape: 02-api-report-shape.md

This file defines the required structure and content for a passing report from Scenario 02.

## Summary

Must contain:
- What was done (scaffolded Axum API + added /v1/ping endpoint)
- How it was done (using templates/guide 02 and guide 07)
- Outcome (success/failure)

## Files Changed

| File | Change |
|------|--------|
| Cargo.toml | Initial project manifest with axum dependencies |
| src/main.rs | Main entry point with route setup |
| src/routes/mod.rs | Routes module |
| src/routes/v1/mod.rs | Versioned v1 routes with ping endpoint |

## Commands Run

| Command | Result | Output Summary |
|---------|--------|----------------|
| cargo build | PASS/FAIL | [summary] |
| cargo check | PASS/FAIL | [summary] |
| cargo test | PASS/FAIL | [summary] |
| cargo clippy | PASS/FAIL | [summary] |
| cargo fmt | PASS/FAIL | [summary] |
| cargo doc | PASS/FAIL | [summary] |
| curl /health | PASS/FAIL | [summary] |
| curl /v1/ping | PASS/FAIL | [summary] |

## Risks

Must contain at least one identified risk or limitation.

## Next Steps

Must contain at least two actionable next steps.

## Final Verdict

Must be one of: `READY` / `READY_WITH_LIMITATIONS` / `NOT_READY`

## Failure Conditions

A report fails validation if:
1. Missing required section
2. Wrong table columns
3. Route not versioned (at /ping instead of /v1/ping)
4. False claims about command results
5. Missing required commands