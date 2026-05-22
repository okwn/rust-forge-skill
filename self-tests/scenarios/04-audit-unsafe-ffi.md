# Self-Test Scenario 04: Audit Unsafe FFI Code

## Context

You are testing the Rust Forge skill pack. You will be given the skill pack files and asked to audit and safely wrap unsafe FFI code.

## Your Task

Audit a piece of unsafe FFI code and create a safe Rust wrapper around it.

## Instructions

### Step 1: Read the Skill Pack

Read these files and understand their guidance:
- `SKILL.md` - Overview of the skill pack
- `guides/04-audit-unsafe-code.md` - How to audit unsafe code
- `guides/11-ffi-wrapping-patterns.md` - How to wrap FFI safely
- `checklists/unsafe-audit-checklist.md` - What to check for unsafe code

### Step 2: The Unsafe Code

Here is the unsafe FFI code you must audit. Copy this into a file called `src/lib.rs` in `/tmp/rust-forge-test/unsafe-ffi/`:

```rust
use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct RawUser {
    pub id: u32,
    pub name: *const c_char,
    pub email: *const c_char,
    pub age: u32,
}

#[link(name = "example_lib")]
extern "C" {
    fn create_user(name: *const c_char, email: *const c_char, age: u32) -> *mut RawUser;
    fn get_user_name(user: *mut RawUser) -> *const c_char;
    fn free_user(user: *mut RawUser);
}

/// Get the user's name from a raw user pointer
/// SAFETY: caller must ensure user is a valid pointer
pub unsafe fn get_name(user: *mut RawUser) -> String {
    let c_str = get_user_name(user);
    if c_str.is_null() {
        return String::new();
    }
    String::from_utf8_unchecked(c_str::CStr::from_ptr(c_str).as_bytes().to_vec())
}

/// Create a new user
/// SAFETY: caller must ensure name and email are valid null-terminated strings
pub unsafe fn create(name: &str, email: &str, age: u32) -> *mut RawUser {
    let name_c = CString::new(name).unwrap();
    let email_c = CString::new(email).unwrap();
    create_user(name_c.as_ptr(), email_c.as_ptr(), age)
}
```

Also create a minimal `Cargo.toml`:

```toml
[package]
name = "unsafe-ffi"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"
```

### Step 3: Audit the Code

Using the unsafe audit checklist and FFI wrapping guide, identify all issues:

1. **List every `unsafe` block** and explain why it's needed
2. **Identify missing safety contracts** - preconditions that aren't documented
3. **Find potential undefined behavior** - null pointers, lifetime issues, etc.
4. **Check for memory safety issues** - double-free, use-after-free, leaks
5. **Identify missing error handling**
6. **Check for UTF-8 validation issues**

### Step 4: Create a Safe Wrapper

Using the FFI wrapping patterns guide, create a safe Rust wrapper that:

1. **Eliminates raw pointer returns** - use `Box<RawUser>` or `Arc<RawUser>`
2. **Adds proper lifetime management** - no/use-after-free
3. **Implements `Drop` for automatic cleanup** - no memory leaks
4. **Adds UTF-8 validation** - prevent invalid UTF-8 crashes
5. **Creates a safe public API** - no `unsafe` required to use the library
6. **Adds `Send`/`Sync` bounds** if applicable

### Step 5: Run Validation

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

# Check for unsafe code
cargo audit 2>&1 || true
```

### Step 6: Produce Report

Write a report (as `report.md` in the project root) that contains:

```markdown
## Summary
[2-3 sentences: what was done, how it was done, and the outcome]

## Unsafe Code Audit

### Unsafe Blocks Found
| Location | Reason for unsafe | Safety Contract |
|----------|-------------------|-----------------|
| get_name() | Dereferencing raw pointer | [contract] |
| create() | Passing raw pointers to C | [contract] |

### Issues Identified
| Issue | Severity | Description |
|-------|----------|-------------|
| Missing null checks | HIGH | [description] |
| UTF-8 validation missing | MEDIUM | [description] |
| Memory leak potential | HIGH | [description] |
| No Drop implementation | MEDIUM | [description] |

### Undefined Behavior Risks
- [List any potential UB risks]

## Safe Wrapper Design

### API Changes
| Old (unsafe) | New (safe) |
|--------------|------------|
| `*mut RawUser` return | [safe alternative] |
| No lifetime management | [how you fixed it] |
| Null pointer risk | [how you fixed it] |

### Implementation Details
- [Key implementation decisions]

## Files Changed
| File | Change |
|------|--------|
| src/lib.rs | [describe] |
| src/safe_wrapper.rs | [if created] |
| src/ffi_types.rs | [if created] |
| Cargo.toml | [describe] |

## Commands Run
| Command | Result | Output Summary |
|---------|--------|----------------|
| cargo build | PASS/FAIL | [summary] |
| cargo test | PASS/FAIL | [summary] |
| cargo clippy | PASS/FAIL | [summary] |
| cargo fmt | PASS/FAIL | [summary] |
| cargo audit | PASS/FAIL | [summary] |

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
- Unsafe code is properly documented with safety contracts
- Safe wrapper eliminates raw pointer exposure in public API
- `Drop` trait implemented for automatic cleanup
- UTF-8 validation added
- All tests pass
- Clippy passes with no warnings
- Format check passes
- Report contains all required sections