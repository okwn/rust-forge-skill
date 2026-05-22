# Self-Test Scenario 05: Create Workspace Service

## Context

You are testing the Rust Forge skill pack. You will be given the skill pack files and asked to create a workspace service following organizational patterns.

## Your Task

Create a new workspace service with domain operations following the Rust Forge organizational patterns.

## Instructions

### Step 1: Read the Skill Pack

Read these files and understand their guidance:
- `SKILL.md` - Overview of the skill pack
- `guides/05-workspace-structure.md` - How to structure workspace services
- `guides/09-add-domain-operation.md` - How to add domain operations
- `templates/workspace-service/` - The workspace service template
- `checklists/service-checklist.md` - What to verify for services

### Step 2: Create the Project

Using the guidance from the skill pack:

1. Create a new workspace service named `user-service` in `/tmp/rust-forge-test/`
2. Use the `templates/workspace-service/` template as the base
3. The workspace should have:
   - A `user-core` crate for domain logic
   - A `user-api` crate for the API layer
   - A `user-domain` crate for domain models
   - Shared `user-common` crate for common types

### Step 3: Define Domain Models

In the `user-domain` crate, define:

```rust
// User aggregate
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub profile: UserProfile,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Value objects
pub struct UserId(UUID);
pub struct Email(String);
pub struct UserProfile {
    pub display_name: String,
    pub bio: Option<String>,
}

pub enum UserStatus {
    Active,
    Suspended,
    PendingVerification,
}
```

### Step 4: Add Domain Operations

In the `user-core` crate, implement these domain operations:

1. **RegisterUser** - Creates a new user with pending verification status
2. **ActivateUser** - Changes status from PendingVerification to Active
3. **SuspendUser** - Changes status to Suspended (admin action)

Each operation should:
- Take input values (not domain objects directly)
- Validate business rules
- Return a result with the new state or an error
- Be documented with its invariants

### Step 5: Run Validation

Execute these commands and record the results:

```bash
# Build the entire workspace
cargo build --workspace 2>&1

# Check all crates
cargo check --workspace 2>&1

# Run all tests across workspace
cargo test --workspace 2>&1

# Run clippy on all crates
cargo clippy --workspace -- -D warnings 2>&1

# Format check
cargo fmt -- --check 2>&1

# Check for any missing documentation
cargo doc --workspace --no-deps 2>&1
```

### Step 6: Produce Report

Write a report (as `report.md` in the project root) that contains:

```markdown
## Summary
[2-3 sentences: what was done, how it was done, and the outcome]

## Workspace Structure
```
user-service/
├── user-domain/      # Domain models and value objects
├── user-core/        # Domain operations and business logic
├── user-api/         # API layer (if included)
├── user-common/      # Shared types and utilities
└── Cargo.toml       # Workspace manifest
```

## Domain Model

### Entities
| Entity | Responsibility |
|--------|----------------|
| User | [responsibility] |

### Value Objects
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
| [other files] | [describe] |

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
- [At least one risk or known limitation]

## Next Steps
1. [Actionable next step]
2. [Actionable next step]

## Final Verdict
[READEY / READY_WITH_LIMITATIONS / NOT_READY]
```

## Success Criteria

- Workspace builds without errors
- All domain models properly structured
- All three domain operations implemented
- Business rules validated in operations
- All tests pass
- Clippy passes with no warnings
- Format check passes
- Documentation builds without errors
- Report contains all required sections