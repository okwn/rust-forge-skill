# Self-Test Scenario 01: Scaffold CLI Application

## Context

You are testing the Rust Forge skill pack. You will be given the skill pack files and asked to complete a concrete task.

## Your Task

Create a new CLI application using the Rust Forge skill pack templates, then add a custom command to it.

## Instructions

### Step 1: Read the Skill Pack

Read these files and understand their guidance:
- `SKILL.md` - Overview of the skill pack
- `guides/01-scaffold-cli-project.md` - How to scaffold CLI projects
- `guides/06-add-cli-command.md` - How to add new commands
- `templates/cli-app/` - The CLI template to use

### Step 2: Create the Project

Using the guidance from the skill pack:

1. Create a new CLI project named `hello-rust-cli` in `/tmp/rust-forge-test/`
2. Use the `templates/cli-app/` template as the base
3. Initialize it properly (cargo init, git init if needed)

### Step 3: Add a Custom Command

Add a new command called `greet` that:
- Takes a `--name <NAME>` flag (required)
- Takes an optional `--formal` flag for formal greeting
- Outputs either "Hello, {name}!" or "Good day, {name}." based on the flag

### Step 4: Run Validation

Execute these commands and record the results:

```bash
# Build the project
cargo build 2>&1

# Run the new command (help)
cargo run -- greet --help 2>&1

# Run the new command with name
cargo run -- greet --name World 2>&1

# Run the new command with formal flag
cargo run -- greet --name Mr. Smith --formal 2>&1

# Run tests
cargo test 2>&1

# Run clippy for linting
cargo clippy -- -D warnings 2>&1
```

### Step 5: Produce Report

Write a report (as `report.md` in the project root) that contains:

```markdown
## Summary
[2-3 sentences: what was done, how it was done, and the outcome]

## Files Changed
| File | Change |
|------|--------|
| src/main.rs | [describe] |
| Cargo.toml | [describe] |
| [other files] | [describe] |

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
- [At least one risk or known limitation]

## Next Steps
1. [Actionable next step]
2. [Actionable next step]

## Final Verdict
[READEY / READY_WITH_LIMITATIONS / NOT_READY]
```

## Success Criteria

- Project builds without errors
- `greet` command works with `--name` flag
- `greet` command works with `--formal` flag
- All tests pass
- Clippy passes with no warnings
- Report contains all required sections