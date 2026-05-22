# Self-Test Scenario 03: Refactor Bad Library Code

## Context

You are testing the Rust Forge skill pack. You will be given the skill pack files and asked to refactor deliberately bad code following proper Rust patterns.

## Your Task

Refactor a poorly-written Rust library into idiomatic, safe, testable code.

## Instructions

### Step 1: Read the Skill Pack

Read these files and understand their guidance:
- `SKILL.md` - Overview of the skill pack
- `guides/03-define-module-boundaries.md` - How to structure modules
- `guides/12-error-handling-patterns.md` - Error handling best practices
- `checklists/rust-code-quality.md` - Code quality checklist

### Step 2: The Bad Code

Here is the code you must refactor. Copy this into a file called `src/lib.rs` in `/tmp/rust-forge-test/bad-library/`:

```rust
use std::fs::File;
use std::io::Read;

pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub age: u32,
}

pub fn get_users() -> Vec<User> {
    let mut file = File::open("users.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let mut users = Vec::new();
    for (key, value) in data["users"].as_array().unwrap().iter().enumerate() {
        let id = value["id"].as_u64().unwrap() as u32;
        let name = value["name"].as_str().unwrap().to_string();
        let email = value["email"].as_str().unwrap().to_string();
        let age = value["age"].as_u64().unwrap() as u32;
        let user = User { id, name, email, age };
        users.push(user);
    }
    return users;
}

pub fn validate_user(user: &User) -> bool {
    if user.name.len() < 2 {
        return false;
    }
    if !user.email.contains("@") {
        return false;
    }
    if user.age < 18 {
        return false;
    }
    return true;
}

pub fn process_users() -> String {
    let users = get_users();
    let mut result = String::new();
    for user in users.iter() {
        if validate_user(user) {
            result.push_str(&format!("{} <{}>\n", user.name, user.email));
        }
    }
    return result;
}

pub fn find_user_by_id(id: u32) -> Option<User> {
    let users = get_users();
    for user in users.iter() {
        if user.id == id {
            return Some(User { id: user.id, name: user.name.clone(), email: user.email.clone(), age: user.age });
        }
    }
    return None;
}
```

Also create a minimal `Cargo.toml`:

```toml
[package]
name = "bad-library"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

And a sample `users.json`:

```json
{
  "users": [
    {"id": 1, "name": "Alice", "email": "alice@example.com", "age": 30},
    {"id": 2, "name": "Bob", "email": "bob@example.com", "age": 17},
    {"id": 3, "name": "Charlie", "email": "charlie@test", "age": 25}
  ]
}
```

### Step 3: Refactor the Code

Using the guidance from guides 03 and 12, refactor this code to:

1. **Replace all `unwrap()` calls** with proper error handling using `thiserror` or `anyhow`
2. **Split into proper modules**: separate data loading, validation, and processing
3. **Add a custom error type** for the library
4. **Add unit tests** for `validate_user` and other pure functions
5. **Use idiomatic Rust**: iterators, `?` operator, proper error propagation
6. **Remove the global file dependency** - pass data in or use a trait

### Step 4: Run Validation

Execute these commands and record the results:

```bash
# Build the project
cargo build 2>&1

# Run tests
cargo test 2>&1

# Run clippy for linting
cargo clippy -- -D warnings 2>&1

# Format check
cargo fmt -- --check 2>&1

# Audit for unsafe code
cargo audit 2>&1 || true
```

### Step 5: Produce Report

Write a report (as `report.md` in the project root) that contains:

```markdown
## Summary
[2-3 sentences: what was done, how it was done, and the outcome]

## Problems Identified
- [List all the problems in the original code]

## Refactoring Changes
| Original Problem | Solution Applied |
|-----------------|------------------|
| unwrap() on fallible operations | [how you fixed it] |
| Giant function | [how you split it] |
| String errors | [how you fixed it] |
| No tests | [what tests you added] |
| Global file dependency | [how you fixed it] |

## Files Changed
| File | Change |
|------|--------|
| src/lib.rs | [describe] |
| Cargo.toml | [describe] |
| src/error.rs | [if you created it] |
| src/validation.rs | [if you created it] |
| src/models.rs | [if you created it] |

## Commands Run
| Command | Result | Output Summary |
|---------|--------|----------------|
| cargo build | PASS/FAIL | [summary] |
| cargo test | PASS/FAIL | [summary] |
| cargo clippy | PASS/FAIL | [summary] |
| cargo fmt | PASS/FAIL | [summary] |

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
- All `unwrap()` calls replaced with proper error handling
- Code split into logical modules
- Custom error type defined and used
- Unit tests added for validation logic
- All tests pass
- Clippy passes with no warnings
- Format check passes
- Report contains all required sections