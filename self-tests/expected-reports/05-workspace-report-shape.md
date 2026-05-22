# Expected Report Shape: 05-create-workspace-service.md

This file defines the required structure and content for a passing report from Scenario 05.

## Summary

Must contain:
- What was done (created workspace service with domain operations)
- How it was done (using guides 05 and 09)
- Outcome (success/failure)

## Workspace Structure

Must show the workspace layout in tree format:
```
user-service/
├── user-domain/      # [description]
├── user-core/        # [description]
├── user-api/         # [description or "not included"]
├── user-common/      # [description or "not included"]
└── Cargo.toml       # Workspace manifest
```

## Domain Model

### Entities

Must include a table:
| Entity | Responsibility |
|--------|----------------|
| User | [responsibility] |

### Value Objects

Must include a table:
| Value Object | Validation |
|--------------|------------|
| UserId | [validation] |
| Email | [validation] |
| UserProfile | [validation] |
| UserStatus | [allowed transitions] |

## Domain Operations

### RegisterUser
- **Input**: [parameters]
- **Preconditions**: [business rules]
- **Postconditions**: [guarantees]
- **Errors**: [error cases]

### ActivateUser
- **Input**: [parameters]
- **Preconditions**: [business rules]
- **Postconditions**: [guarantees]
- **Errors**: [error cases]

### SuspendUser
- **Input**: [parameters]
- **Preconditions**: [business rules]
- **Postconditions**: [guarantees]
- **Errors**: [error cases]

## Files Changed

| File | Change |
|------|--------|
| Cargo.toml | [describe] |
| user-domain/src/lib.rs | [describe] |
| user-core/src/lib.rs | [describe] |

## Commands Run

| Command | Result | Output Summary |
|---------|--------|----------------|
| cargo build --workspace | PASS/FAIL | [summary] |
| cargo check --workspace | PASS/FAIL | [summary] |
| cargo test --workspace | PASS/FAIL | [summary] |
| cargo clippy --workspace | PASS/FAIL | [summary] |
| cargo fmt | PASS/FAIL | [summary] |
| cargo doc --workspace | PASS/FAIL | [summary] |

## Risks

Must contain at least one identified risk or limitation.

## Next Steps

Must contain at least two actionable next steps.

## Final Verdict

Must be one of: `READY` / `READY_WITH_LIMITATIONS` / `NOT_READY`

## Failure Conditions

A report fails validation if:
1. Missing required section
2. Wrong workspace structure
3. Missing domain operations (less than 3)
4. No business rule validation
5. False claims about command results
6. Missing value objects