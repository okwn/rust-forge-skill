# 08 — Security and Unsafe Audit

**Purpose:** Guide agents to audit Rust code for security vulnerabilities, memory safety issues, and supply chain risks. Rust provides strong guarantees — but only if unsafe code is correct and dependencies are trustworthy.

---

## Unsafe Inventory

**Every `unsafe` block is a potential vulnerability.** Audit all unsafe systematically.

### Required SAFETY Documentation

Every `unsafe` block must have a `SAFETY` comment with three elements:

```rust
unsafe {
    // SAFETY: [1] What invariant must hold
    //
    // [2] Why this invariant holds at this call site
    //
    // [3] What undefined behavior occurs if invariant is violated
    ...
}
```

### Unsafe Checklist

```
[ ] Every unsafe block has a SAFETY comment
[ ] SAFETY comment covers: invariant, hold, UB
[ ] No unsafe block exceeds 10 lines (refactor if larger)
[ ] Unsafe code is wrapped in safe public APIs immediately
[ ] Unsafe impl blocks have safety documentation
[ ] static mut is never used (use Mutex/OnceLock/atomic)
[ ] Raw pointer dereferences are validated before use
[ ] No uninitialized MaybeUninit values leaked
[ ] Unsafe code has tests for both valid and invalid inputs
```

---

## Dependency Security

### cargo audit

```bash
# Install
cargo install cargo-audit

# Run vulnerability scan
cargo audit

# CI integration
- name: Security audit
  run: cargo audit
```

**Run cargo audit in CI on every push.** Do not skip this.

### cargo deny

```bash
# Install
cargo install cargo-deny

# Check advisories, licenses, bans
cargo deny check advisories licenses ban
```

Add `cargo-deny` to CI:

```yaml
cargo-deny:
  name: Cargo Deny
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: taiki-e/cargo-deny-action@v1
      with:
        command: check advisories licenses ban
```

### Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated --workspace

# Update all dependencies
cargo update --workspace

# Update specific crate
cargo update -p serde
```

---

## Secret Handling

**Rule: No secrets in source code. No secrets in logs.**

### Bad Patterns

```rust
// BAD: API key in source
let api_key = "sk-1234567890abcdef";

// BAD: API key in log
tracing::info!("API key: {}", api_key);

// BAD: Password in error message
return Err(format!("password {} is incorrect", password));
```

### Good Patterns

```rust
// GOOD: API key from environment
let api_key = std::env::var("API_KEY")
    .map_err(|_| anyhow::anyhow!("API_KEY not set"))?;

// GOOD: Use a SecretString type
use zeroize::Zeroize;

struct SecretString(String);

impl Drop for SecretString {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

// GOOD: Mask sensitive fields in logging
tracing::info!(user_id = %user.id, "login attempt");  // No email in logs
```

### .env.example

```bash
# .env.example — no real secrets
DATABASE_URL=postgresql://user:password@host:port/db
REDIS_URL=redis://host:port
API_KEY=your-api-key-here
JWT_SECRET=minimum-32-characters-here
```

---

## Input Validation

**All external input must be validated before use.**

### String/Path Validation

```rust
// BAD: unsanitized path
let content = std::fs::read_to_string(&user_input_path)?;

// GOOD: validate and sanitize path
fn safe_path(input: &Path) -> Result<PathBuf> {
    let path = input.clean();
    // Reject paths that escape the allowed directory
    if path.starts_with("..") {
        return Err(anyhow::anyhow!("path traversal detected"));
    }
    Ok(path)
}
```

### Numeric Validation

```rust
// BAD: no overflow check
let size = width * height * channels;

// GOOD: checked multiplication
let size = width
    .checked_mul(height)
    .and_then(|h| h.checked_mul(channels))
    .ok_or_else(|| anyhow::anyhow!("size calculation overflow"))?;
```

### SQL Injection Prevention

```rust
// BAD: string interpolation in SQL
let query = format!("SELECT * FROM users WHERE id = {}", user_id);

// GOOD: parameterized query
let user = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = $1",
    user_id
).fetch_optional(&pool).await?;
```

### Deserialization Security

```rust
// Use serde deserialize_with for validation
#[derive(Deserialize)]
struct Config {
    #[serde(deserialize_with = "validate_port")]
    port: u16,

    #[serde(default)]
    max_connections: u32,
}

fn validate_port<'de, D>(de: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let port = u16::deserialize(de)?;
    if port == 0 {
        return Err(serde::de::Error::custom("port cannot be zero"));
    }
    Ok(port)
}
```

---

## Command Injection

**Never pass unsanitized user input to shell commands.**

```rust
// BAD: shell injection
let output = std::process::Command::new("sh")
    .arg("-c")
    .arg(&format!("ls {}", user_input))
    .output()?;

// GOOD: use shell arg, not shell -c
let output = std::process::Command::new("ls")
    .arg(user_input)  // Passed as argument, not shell command
    .output()?;
```

---

## Constant-Time Considerations

**For cryptography, use constant-time comparison to prevent timing attacks.**

```rust
// BAD: timing leak
if secret == user_input { ... }

// GOOD: constant-time comparison
use subtle::ConstantTimeEq;

fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    a.ct_eq(b).into()
}
```

**Use `subtle` crate for constant-time operations:**
```toml
subtle = "2.5"
```

---

## Supply Chain Security

### Auditing Crate Origins

```
[ ] All dependencies come from crates.io (or known-good registries)
[ ] No dependencies from git with loose refs (use exact revs)
[ ] No workspace dependencies on unmaintained crates
[ ] Lock file committed to version control
```

### Version Pinning

```toml
# BAD: loose version
serde = "1.0"

# GOOD: precise version with lock file
serde = "1.0.215"  # Or use cargo.lock to pin
```

### License Compliance

```toml
# cargo-deny configuration
[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-3-Clause",
    "Zlib",
]
deny = [
    "GPL-2.0",
    "GPL-3.0",
    "LGPL-2.0",
    "LGPL-3.0",
]
```

---

## Security Audit Checklist

```
[ ] cargo audit passes with no vulnerabilities
[ ] cargo deny check passes
[ ] No secrets in source code or logs
[ ] All user input validated before use
[ ] No shell injection vectors
[ ] SQL injection prevented (parameterized queries)
[ ] Path traversal prevented
[ ] Integer overflow checked in size calculations
[ ] Constant-time comparison for crypto secrets
[ ] TLS certificates validated
[ ] Rate limiting on public endpoints
[ ] .env.example documented, .env in .gitignore
[ ] No TODO with security implications left unreported
[ ] Dependencies use exact versions or lock file pinned
```

---

## Running Security Tools

```bash
# Full security check
cargo audit
cargo deny check advisories licenses ban
./scripts/audit_unsafe.sh

# Check for outdated deps (security relevant)
cargo outdated

# Check for dead code (may contain abandoned security fixes)
cargo machete --strict

# License compliance
cargo deny check licenses
```

---

## Unsafe Audit Script Reference

```bash
#!/usr/bin/env bash
# audit_unsafe.sh

echo "=== Unsafe Block Audit ==="

# Find unsafe blocks
echo "unsafe blocks found:"
grep -rn "unsafe" --include="*.rs" src/ \
  | grep -v "SAFETY:" \
  | grep -v "// SAFETY" \
  | grep -v "unsafe impl"

echo ""
echo "static mut declarations:"
grep -rn "static mut" --include="*.rs" src/

echo ""
echo "Missing SAFETY comments:"
grep -B2 -A5 "unsafe {" --include="*.rs" src/ \
  | grep -v "SAFETY" || echo "All safe"
```