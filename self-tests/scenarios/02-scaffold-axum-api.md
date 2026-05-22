# Self-Test Scenario 02: Scaffold Axum API

## Context

You are testing the Rust Forge skill pack. You will be given the skill pack files and asked to complete a concrete task.

## Your Task

Create a new Axum-based REST API using the Rust Forge skill pack templates, add a versioned route, and include tests.

## Instructions

### Step 1: Read the Skill Pack

Read these files and understand their guidance:
- `SKILL.md` - Overview of the skill pack
- `guides/02-scaffold-axum-api.md` - How to scaffold Axum APIs
- `guides/07-add-api-endpoint.md` - How to add API endpoints
- `templates/axum-api/` - The Axum API template to use
- `checklists/api-checklist.md` - What to verify for APIs

### Step 2: Create the Project

Using the guidance from the skill pack:

1. Create a new API project named `hello-api` in `/tmp/rust-forge-test/`
2. Use the `templates/axum-api/` template as the base
3. Ensure the template has:
   - Proper error handling (Result-based)
   - Health check endpoint
   - OpenAPI/Swagger documentation setup

### Step 3: Add a Versioned Route

Add a new endpoint following the versioned route pattern:

**Route**: `GET /v1/ping`
**Handler**: Returns JSON `{"status": "ok", "version": "1.0.0"}`
**Location**: Should be in the v1 route group

### Step 4: Write Tests

Add tests for the new endpoint:
- Test that `GET /v1/ping` returns 200 OK
- Test that response body contains expected fields
- Test that health check still works at `/health`

### Step 5: Run Validation

Execute these commands and record the results:

```bash
# Build the project
cargo build 2>&1

# Check types
cargo check 2>&1

# Run tests
cargo test 2>&1

# Run clippy for linting
cargo clippy -- -D warnings 2>&1

# Format check
cargo fmt -- --check 2>&1

# Build docs
cargo doc --no-deps 2>&1
```

If the server can be started (it should), also test:

```bash
# Start the server in background
cargo run --bin hello-api &
sleep 2

# Test health endpoint
curl -s http://localhost:8080/health | jq .

# Test the new ping endpoint
curl -s http://localhost:8080/v1/ping | jq .

# Kill the server
pkill -f hello-api
```

### Step 6: Produce Report

Write a report (as `report.md` in the project root) that contains:

```markdown
## Summary
[2-3 sentences: what was done, how it was done, and the outcome]

## Files Changed
| File | Change |
|------|--------|
| src/main.rs | [describe] |
| src/routes/mod.rs | [describe] |
| src/routes/v1/mod.rs | [describe] |
| Cargo.toml | [describe] |
| [other files] | [describe] |

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
- [At least one risk or known limitation]

## Next Steps
1. [Actionable next step]
2. [Actionable next step]

## Final Verdict
[READEY / READY_WITH_LIMITATIONS / NOT_READY]
```

## Success Criteria

- Project builds without errors
- `GET /v1/ping` returns proper JSON response
- `GET /health` still works
- All tests pass
- Clippy passes with no warnings
- Format check passes
- Report contains all required sections