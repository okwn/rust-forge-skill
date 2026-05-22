# Expected Report Shape: 04-audit-unsafe-ffi.md

This file defines the required structure and content for a passing report from Scenario 04.

## Summary

Must contain:
- What was done (audited unsafe FFI code and created safe wrapper)
- How it was done (using guides 04 and 11)
- Outcome (success/failure)

## Unsafe Code Audit

### Unsafe Blocks Found

Must include a table:
| Location | Reason for unsafe | Safety Contract |
|----------|-------------------|-----------------|
| get_name() | [reason] | [contract] |
| create() | [reason] | [contract] |

### Issues Identified

Must include a table with severity levels:
| Issue | Severity | Description |
|-------|----------|-------------|
| [issue] | HIGH/MEDIUM/LOW | [description] |

### Undefined Behavior Risks

Must list potential UB risks.

## Safe Wrapper Design

### API Changes

Must include a table:
| Old (unsafe) | New (safe) |
|--------------|------------|
| [unsafe API] | [safe replacement] |

### Implementation Details

Must explain Drop, UTF-8 validation, lifetime management.

## Files Changed

| File | Change |
|------|--------|
| src/lib.rs | [describe] |
| src/safe_wrapper.rs | [if created] |

## Commands Run

| Command | Result | Output Summary |
|---------|--------|----------------|
| cargo build | PASS/FAIL | [summary] |
| cargo test | PASS/FAIL | [summary] |
| cargo clippy | PASS/FAIL | [summary] |
| cargo fmt | PASS/FAIL | [summary] |
| cargo audit | PASS/FAIL | [summary] |

## Risks

Must contain at least one identified risk or limitation.

## Next Steps

Must contain at least two actionable next steps.

## Final Verdict

Must be one of: `READY` / `READY_WITH_LIMITATIONS` / `NOT_READY`

## Failure Conditions

A report fails validation if:
1. Missing required section
2. Unsafe blocks not documented
3. No safe wrapper created
4. No Drop implemented
5. False claims about command results
6. Missing issues in audit