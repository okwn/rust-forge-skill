# 12 — Rust Anti-Patterns

**Purpose:** A comprehensive catalog of Rust anti-patterns — what to avoid, why it hurts, the preferred fix, and what to tell the agent. This is the definitive reference for code quality enforcement.

---

## Error Handling

### 1. `.unwrap()` in Production Code

**Anti-pattern:**
```rust
pub fn parse(input: &str) -> Result<Data, ParseError> {
    Ok(serde_json::from_str(input).unwrap())  // Panics on invalid JSON
}
```

**Why it hurts:** Panics crash the process. No error context, no recovery.

**Preferred fix:**
```rust
pub fn parse(input: &str) -> Result<Data, ParseError> {
    serde_json::from_str(input)
        .map_err(|e| ParseError::InvalidJson(e.to_string()))
}
```

**Agent instruction:** Never use `.unwrap()` in library, service, or API handler code. Use `?` with typed error propagation.

---

### 2. `.expect()` in Production Code

**Anti-pattern:**
```rust
let config = std::env::var("CONFIG_PATH").expect("CONFIG_PATH must be set");
```

**Why it hurts:** Same as `.unwrap()` — panics on failure.

**Preferred fix:**
```rust
let config = std::env::var("CONFIG_PATH")
    .map_err(|_| ConfigError::Missing("CONFIG_PATH".into()))?;
```

**Agent instruction:** Replace all `.expect()` in production code with `?` + typed error.

---

### 3. `String` as Error Type

**Anti-pattern:**
```rust
pub fn parse(input: &str) -> Result<Data, String> {
    if input.is_empty() {
        Err("input cannot be empty".to_string())
    } else {
        Ok(Data {})
    }
}
```

**Why it hurts:** Callers cannot match exhaustively. Loses structure.

**Preferred fix:**
```rust
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("input cannot be empty")]
    EmptyInput,
}

pub fn parse(input: &str) -> Result<Data, ParseError> {
    if input.is_empty() {
        Err(ParseError::EmptyInput)
    } else {
        Ok(Data {})
    }
}
```

**Agent instruction:** Use `thiserror` for library error types. Never return `String` as an error type.

---

### 4. Panic for Recoverable Errors

**Anti-pattern:**
```rust
let value = config.get("key").unwrap();  // Panic if missing
```

**Why it hurts:** Process crash for recoverable condition.

**Preferred fix:**
```rust
let value = config.get("key")
    .ok_or_else(|| ConfigError::Missing("key".into()))?;
```

**Agent instruction:** Treat recoverable errors as errors. Never use unwrap/expect on external data.

---

### 5. `Box<dyn Error>` as Return Type

**Anti-pattern:**
```rust
pub fn run() -> Result<(), Box<dyn std::error::Error>> { ... }
```

**Why it hurts:** Loses type information, defeats exhaustive matching.

**Preferred fix:**
```rust
pub fn run() -> Result<(), MyError> { ... }
```

**Agent instruction:** Use concrete error enums. `Box<dyn Error>` is acceptable only at the absolute boundary (main), and even then `anyhow` is preferred.

---

### 6. Error Context Lost

**Anti-pattern:**
```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)  // No context about which file failed
}
```

**Preferred fix:**
```rust
fn read_file(path: &str) -> anyhow::Result<String> {
    std::fs::read_to_string(path)
        .context(format!("failed to read file: {}", path))?
}
```

**Agent instruction:** Use `.context()` to add context before propagating errors.

---

## Async

### 7. Blocking Sync I/O in Async Fn

**Anti-pattern:**
```rust
async fn bad() {
    let data = std::fs::read("file.txt").unwrap();  // Blocks executor!
}
```

**Why it hurts:** Blocks the async executor thread, starves other tasks.

**Preferred fix:**
```rust
async fn good() -> anyhow::Result<String> {
    tokio::fs::read_to_string("file.txt").await
}
```

**Agent instruction:** Use `tokio::fs` for async file I/O. Use `spawn_blocking` for truly blocking operations.

---

### 8. `block_on` Inside Async Context

**Anti-pattern:**
```rust
async fn bad() {
    let result = tokio::runtime::block_on(async { ... });  // Deadlock risk
}
```

**Why it hurts:** Causes deadlock when called from within an async context.

**Preferred fix:** Remove `block_on`. Restructure with `.await`.

---

### 9. `async` Without `#[async_trait]`

**Anti-pattern:**
```rust
trait Repository {
    async fn find(&self, id: Uuid) -> Result<Option<Item>, Error>;  // Wrong!
}
```

**Why it hurts:** Object-safe async traits require `#[async_trait]`.

**Preferred fix:**
```rust
use async_trait::async_trait;

#[async_trait]
trait Repository {
    async fn find(&self, id: Uuid) -> Result<Option<Item>, Error>;
}
```

**Agent instruction:** Use `#[async_trait]` on all async trait definitions and implementations.

---

### 10. Untracked Spawned Tasks

**Anti-pattern:**
```rust
async fn spawn_without_tracking() {
    tokio::spawn(async {  // Handle dropped, task may outlive scope
        do_work().await;
    });
}
```

**Why it hurts:** Task runs to completion even if the parent scope aborts.

**Preferred fix:**
```rust
async fn spawn_with_tracking() -> anyhow::Result<()> {
    let handle = tokio::spawn(async { do_work().await });
    handle.await?;  // Wait for completion or abort
    Ok(())
}
```

**Agent instruction:** Always await or abort spawned tasks. Store the JoinHandle.

---

## Ownership

### 11. `clone()` in Hot Paths

**Anti-pattern:**
```rust
for item in &items {
    let data = process(item.clone());  // Unnecessary allocation
}
```

**Why it hurts:** Allocation and copy overhead in tight loop.

**Preferred fix:**
```rust
for item in &items {
    let data = process(item);  // Borrow, no allocation
}
```

**Agent instruction:** Profile before cloning. Prefer borrowing in hot paths.

---

### 12. `static mut` Global State

**Anti-pattern:**
```rust
static mut GLOBAL_STATE: u64 = 0;
unsafe { GLOBAL_STATE += 1; }
```

**Why it hurts:** Undefined behavior under Rust's memory model.

**Preferred fix:**
```rust
use std::sync::atomic::{AtomicU64, Ordering};
static COUNTER: AtomicU64 = AtomicU64::new(0);
COUNTER.fetch_add(1, Ordering::SeqCst);
```

**Agent instruction:** Never use `static mut`. Use `Mutex`, `OnceLock`, or atomics.

---

### 13. `Rc<RefCell<T>>` in Async Code

**Anti-pattern:**
```rust
let data = Rc::new(RefCell::new(0));
// Not Send, not Sync — breaks async
```

**Why it hurts:** `Rc` and `RefCell` are not `Send` or `Sync`, cannot be used in async contexts.

**Preferred fix:**
```rust
use std::sync::{Arc, Mutex};
let data = Arc::new(Mutex::new(0));
// Arc<Mutex<T>> is Send + Sync
```

**Agent instruction:** Use `Arc<Mutex<T>>` or `Arc<RwLock<T>>` for shared mutation in async.

---

### 14. Unwrap on `get()` / `first()`

**Anti-pattern:**
```rust
let first = vec.first().unwrap();  // Panic if empty
let val = map.get(&key).unwrap();  // Panic if missing
```

**Why it hurts:** Panics on common edge cases.

**Preferred fix:**
```rust
if let Some(first) = vec.first() { ... }
if let Some(val) = map.get(&key) { ... }
```

**Agent instruction:** Use `if let` or `and_then` on `Option` from `get()` / `first()`.

---

## Lifetimes

### 15. Hidden Lifetime Elision

**Anti-pattern:**
```rust
fn process(data: &str) -> &str {  // Lifetime elided but not obvious
    data
}
```

**Why it hurts:** When lifetimes are wrong, error messages are confusing.

**Preferred fix:**
```rust
fn process<'a>(data: &'a str) -> &'a str {
    data
}
```

**Agent instruction:** Write explicit lifetimes when the relationship matters.

---

### 16. `'static` Bound When Not Needed

**Anti-pattern:**
```rust
fn make_string() -> &'static str {
    Box::leak("owned".into())
}
```

**Why it hurts:** Unnecessarily restricts callers, leaks memory.

**Preferred fix:** Return `String` or `Cow<'_, str>` instead.

---

## Unsafe

### 17. Unsafe Without SAFETY Comment

**Anti-pattern:**
```rust
unsafe {
    let val = *ptr;  // No safety documentation
}
```

**Why it hurts:** Unverifiable invariants, potential UB.

**Preferred fix:**
```rust
unsafe {
    // SAFETY: ptr must be non-null and point to initialized T.
    // Valid for reads of size std::mem::size_of::<T>().
    // The Vec3Wrapper type owns this pointer and manages its lifetime.
    let val = *ptr;
}
```

**Agent instruction:** Every `unsafe` block requires a `SAFETY` comment with: (1) invariant, (2) why it holds, (3) UB if violated.

---

### 18. Raw Pointer Dereference Without Validation

**Anti-pattern:**
```rust
unsafe {
    let val = *raw_ptr;  // No null check
}
```

**Preferred fix:**
```rust
if raw_ptr.is_null() {
    return Err(FfiError::NullPointer);
}
unsafe {
    let val = *raw_ptr;
}
```

**Agent instruction:** Validate all raw pointers before dereferencing.

---

### 19. Unsafe Block Exceeding 10 Lines

**Anti-pattern:**
```rust
unsafe {
    // 50 lines of complex FFI code
    ...
}
```

**Why it hurts:** Hard to audit, easy to miss invariant violations.

**Preferred fix:** Refactor into a separate safe wrapper function.

**Agent instruction:** If an `unsafe` block exceeds 10 lines, refactor.

---

## FFI

### 20. Missing Null Check on FFI Boundary

**Anti-pattern:**
```rust
unsafe {
    lib.some_function(ptr);  // ptr could be null
}
```

**Preferred fix:**
```rust
if ptr.is_null() {
    return Err(FfiError::NullPointer);
}
unsafe {
    lib.some_function(ptr);
}
```

**Agent instruction:** Always validate pointers from C before use.

---

### 21. CString Without Validation

**Anti-pattern:**
```rust
let c_str = CString::new(user_input)?;  // User input might contain null byte
```

**Preferred fix:**
```rust
let c_str = CString::new(user_input)
    .map_err(|_| FfiError::InvalidInput("null byte in string".into()))?;
```

**Agent instruction:** Validate strings before converting to CString.

---

## Modules

### 22. Giant `main.rs`

**Anti-pattern:**
```rust
// src/main.rs — 400 lines
// Contains: CLI parsing, business logic, config loading, DB queries
```

**Why it hurts:** Violates single responsibility. Hard to test. Impossible to reuse.

**Preferred fix:** Split into `lib.rs` + `commands/` + `config.rs`.

**Agent instruction:** `main.rs` must be under 100 lines.

---

### 23. `mod utils`

**Anti-pattern:**
```rust
mod utils;  // 500 lines of unrelated helpers
```

**Why it hurts:** `utils` is a code smell — missing domain types.

**Preferred fix:** Split `utils.rs` into `config.rs`, `parser.rs`, `formatter.rs` with clear responsibilities.

---

### 24. Circular Module Dependencies

**Anti-pattern:**
```rust
// a.rs
mod b;

// b.rs
mod a;  // Compile error: cyclic dependency
```

**Why it hurts:** Rust's module system does not support cycles.

**Preferred fix:** Extract shared code to a third module `c.rs`. A and B both depend on C.

---

## Dependencies

### 25. `features = ["full"]` All-Features-On

**Anti-pattern:**
```toml
tokio = { version = "1.40", features = ["full"] }  # Everything enabled
```

**Why it hurts:** Bloats binary, slows compile, enables unwanted behavior.

**Preferred fix:**
```toml
tokio = { version = "1.40", features = ["rt", "sync"] }  # Only what you need
```

**Agent instruction:** Enable only the features you use.

---

### 26. Adding Crate for Trivial Function

**Anti-pattern:**
```rust
use some_crate::join_strings;  // Just str::join(" ", &[])
```

**Preferred fix:** Implement inline or use `std::iter::join()`.

---

### 27. `lazy_static` When `OnceLock` Exists

**Anti-pattern:**
```rust
lazy_static! {
    static ref CONFIG: Config = { ... };
}
```

**Preferred fix:**
```rust
static CONFIG: OnceLock<Config> = OnceLock::new();
fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::load())
}
```

**Agent instruction:** Use `std::sync::OnceLock` instead of `lazy_static`.

---

## Performance

### 28. `format!()` in Hot Logging

**Anti-pattern:**
```rust
tracing::info!("user {} logged in at {}", user_id, timestamp);
```

**Why it hurts:** `format!` allocation on every log call, even when disabled.

**Preferred fix:**
```rust
tracing::info!(
    user_id = %user_id,
    timestamp = %timestamp,
    "user logged in"
);
```

**Agent instruction:** Use structured fields in tracing, not format! strings.

---

### 29. `collect::<Vec<_>>()` Without Capacity Hint

**Anti-pattern:**
```rust
let results: Vec<_> = (0..1000).map(square).collect();
```

**Preferred fix:**
```rust
let mut results = Vec::with_capacity(1000);
for i in 0..1000 {
    results.push(square(i));
}
```

**Agent instruction:** Use `with_capacity` when you know the size.

---

### 30. Premature SIMD

**Anti-pattern:**
```rust
// Let's use SIMD! But we haven't profiled.
use std::simd::f32x4;
```

**Why it hurts:** Complexity without measurement. May actually be slower.

**Preferred fix:** Profile first. Confirm SIMD is the bottleneck.

---

## Tests

### 31. `unwrap()` in Test Helpers Called by Production Code

**Anti-pattern:**
```rust
// Test helper that unwraps — called from production code path
pub fn parse_test_data() -> Data {
    serde_json::from_str(TEST_JSON).unwrap()  // Panic in test helper
}
```

**Why it hurts:** Helper's panic is cryptic when called from production.

**Preferred fix:**
```rust
pub fn parse_test_data() -> Result<Data, ParseError> {
    serde_json::from_str(TEST_JSON).map_err(ParseError::from)
}
```

**Agent instruction:** Test utilities that may be called from production paths must not use `unwrap()`.

---

### 32. Only Happy Path Tested

**Anti-pattern:**
```rust
#[test]
fn test_parse_valid() {
    let data = parse("valid json");
    assert!(data.is_ok());
}
```

**Preferred fix:** Also test error cases.
```rust
#[test]
fn test_parse_empty() {
    let data = parse("");
    assert!(matches!(data, Err(ParseError::EmptyInput)));
}
```

**Agent instruction:** Every function needs error path tests.

---

### 33. `#[ignore]` Without Issue

**Anti-pattern:**
```rust
#[test]
#[ignore]
fn test_flaky_integration() { ... }
```

**Preferred fix:** Fix the flaky test or open an issue tracking it.

---

## Workspaces

### 34. Different MSRV Without Justification

**Anti-pattern:**
```toml
[workspace.package]
rust-version = "1.85"

[package]
rust-version = "1.70"  # But workspace floor is 1.85!
```

**Why it hurts:** Can't depend on higher MSRV crates.

**Preferred fix:** Align MSRV with workspace floor.

---

### 35. Glob Imports in Production

**Anti-pattern:**
```rust
use crate::*;  // Obscures what's actually used
```

**Preferred fix:** Explicit imports only.
```rust
use crate::{Item, User, Error};
```

---

## CI

### 36. `cargo build` in CI Instead of `cargo check`

**Anti-pattern:**
```yaml
# In CI — cargo build is slow
cargo build --workspace
```

**Preferred fix:** Use `cargo check` for speed, `cargo build` only for artifact creation.

---

## Formatting and Style

### 37. Magic Numbers

**Anti-pattern:**
```rust
if items.len() > 3600 { ... }
```

**Preferred fix:**
```rust
const MAX_ITEMS_PER_BATCH: usize = 3600;
if items.len() > MAX_ITEMS_PER_BATCH { ... }
```

---

### 38. Comments Explaining What Instead of Why

**Anti-pattern:**
```rust
// Set count to 1
count = 1;
```

**Preferred fix:** Explain why.
```rust
// Reinitialize to handle retry scenario
count = 1;
```

---

### 39. `// TODO` Without Issue Reference

**Anti-pattern:**
```rust
// TODO: fix this later
```

**Preferred fix:**
```rust
// TODO: fix this later (issue #123)
```

---

### 40. Dead Code Left in Production

**Anti-pattern:**
```rust
// fn old_function() { ... }  // Commented out, not removed
```

**Preferred fix:** Remove dead code. Use git history if needed.

---

## Async (additional)

### 41. Missing Timeout on Async Operations

**Anti-pattern:**
```rust
async fn connect() {
    database.connect().await;  // Could hang forever
}
```

**Preferred fix:**
```rust
use tokio::time::{timeout, Duration};

async fn connect() -> anyhow::Result<()> {
    timeout(Duration::from_secs(30), database.connect()).await??;
}
```

---

### 42. No Backpressure on Unbounded Channels

**Anti-pattern:**
```rust
let (tx, rx) = mpsc::channel::<Job>(0);  // Unbounded
```

**Why it hurts:** Memory exhaustion if producers outpace consumers.

**Preferred fix:** Use bounded channels with backpressure or `Semaphore`.

---

## FFI (additional)

### 43. Panic Crossing FFI Boundary

**Anti-pattern:**
```rust
#[no_mangle]
pub extern "C" fn rust_callback() {
    something_that_panics();  // UB in C
}
```

**Preferred fix:**
```rust
#[no_mangle]
pub extern "C" fn rust_callback() {
    std::panic::catch_unwind(|| {
        something_that_panics();
    });
}
```

---

## Traits

### 44. Overly Generic Bounds Scattered

**Anti-pattern:**
```rust
impl<R: Repository + Send + Sync, S: Service<R> + Clone> for SomeType<R, S>
where R::Error: Into<MyError>
```

**Preferred fix:** Consolidate bounds in trait definitions.

---

### 45. Blanket Impl Without Coherence Plan

**Anti-pattern:**
```rust
impl<T> MyTrait for T
where T: SomeConstraint
```

**Why it hurts:** Downstream crates can't implement the same trait.

**Preferred fix:** Explicit impls per type or documented coherence strategy.

---

## Performance (additional)

### 46. `String` Where `&str` Works in Function Signatures

**Anti-pattern:**
```rust
fn process(data: String) { ... }  // Takes ownership when borrowing suffices
```

**Preferred fix:**
```rust
fn process(data: &str) { ... }  // Borrows
```

---

### 47. Iterator Chain Without Review

**Anti-pattern:**
```rust
let result = data.iter()
    .filter(|x| complex_predicate(x))
    .map(|x| transform(x))
    .collect::<Vec<_>>();
```

**Preferred fix:** Check if the chain is optimized or if explicit loop is faster.

---

## Security

### 48. Secrets in Error Messages

**Anti-pattern:**
```rust
return Err(format!("password {} is incorrect", password));
```

**Preferred fix:**
```rust
return Err("invalid credentials".into());
```

---

### 49. Path Traversal Not Checked

**Anti-pattern:**
```rust
let content = std::fs::read_to_string(&user_path)?;
```

**Preferred fix:**
```rust
fn safe_path(input: &Path) -> Result<PathBuf> {
    let path = input.clean();
    if path.starts_with("..") {
        return Err(anyhow::anyhow!("path traversal"));
    }
    Ok(path)
}
```

---

### 50. No Rate Limiting on Public Endpoints

**Anti-pattern:**
```rust
async fn public_api() { ... }  // No rate limiting
```

**Preferred fix:** Use `tower::limit::ConcurrencyLimit` or similar middleware.

---

## Summary Table

| # | Anti-Pattern | Why It Hurts | Preferred Fix |
|---|---|---|---|
| 1 | `.unwrap()` in prod | Panic crash | `?` + typed error |
| 2 | `.expect()` in prod | Panic crash | `?` + typed error |
| 3 | `String` error | Loses structure | `thiserror` enum |
| 4 | Panic for recoverable | Process crash | `Result` propagation |
| 5 | `Box<dyn Error>` | Loses type info | Typed error enum |
| 6 | Lost context | Debugging hard | `.context()` |
| 7 | Blocking in async | Executor starvation | `tokio::fs` / `spawn_blocking` |
| 8 | `block_on` in async | Deadlock | Restructure with `.await` |
| 9 | `async` without `#[async_trait]` | Object-safe broken | Add `#[async_trait]` |
| 10 | Untracked spawned tasks | Task outlives scope | Store JoinHandle |
| 11 | `clone()` in hot path | Allocation overhead | Borrow |
| 12 | `static mut` | UB | Mutex/atomic |
| 13 | `Rc<RefCell>` in async | Not Send/Sync | `Arc<Mutex>` |
| 14 | Unwrap on `get()` | Panic on empty | `if let` |
| 15 | Hidden lifetimes | Confusing errors | Explicit lifetimes |
| 16 | Unnecessary `'static` | Restricts callers | Infer or specific lifetime |
| 17 | Unsafe without SAFETY | Unverifiable | Add SAFETY comment |
| 18 | Raw ptr without validation | UB | Check null first |
| 19 | Unsafe > 10 lines | Hard to audit | Refactor into safe wrapper |
| 20 | Missing null check FFI | UB | Check before use |
| 21 | CString without validation | Silent corruption | Validate before convert |
| 22 | Giant `main.rs` | Violates SRP | Extract to `lib.rs` + modules |
| 23 | `mod utils` | Code smell | Split by domain |
| 24 | Circular deps | Compile error | Extract to third crate |
| 25 | `features = ["full"]` | Bloats binary | Explicit feature list |
| 26 | Crate for trivial fn | Maintenance burden | Inline or std |
| 27 | `lazy_static` | External dep | `OnceLock` |
| 28 | `format!` in hot logs | Allocation overhead | Structured fields |
| 29 | `collect()` no capacity | Reallocation | `with_capacity` |
| 30 | Premature SIMD | Complexity without need | Profile first |
| 31 | `unwrap()` in test helper | Cryptic panic | Return `Result` |
| 32 | Only happy path tested | Untested errors | Test error paths |
| 33 | `#[ignore]` without issue | Rotting test | Fix or track |
| 34 | Different MSRV | Compatibility breaks | Align to workspace floor |
| 35 | Glob imports | Obscures usage | Explicit imports |
| 36 | `cargo build` in CI | Slow | `cargo check` |
| 37 | Magic numbers | Unclear intent | Named `const` |
| 38 | Comments explain what | Rot misleads | Explain why |
| 39 | `TODO` without issue | Permanent debt | Link to tracker |
| 40 | Dead code left in | Confusion | Remove |
| 41 | Missing timeout | Hang forever | `tokio::time::timeout` |
| 42 | Unbounded channels | Memory exhaustion | Bounded channels |
| 43 | Panic crosses FFI | UB in C | `catch_unwind` |
| 44 | Scattered generic bounds | Hard to read | Consolidate in trait |
| 45 | Blanket impl | Orphan rule issues | Explicit impls |
| 46 | `String` where `&str` | Unnecessary ownership | Borrow |
| 47 | Iterator chain unchecked | Potential slowdown | Profile |
| 48 | Secrets in errors | Info leak | Generic message |
| 49 | Path traversal unchecked | File access | Sanitize path |
| 50 | No rate limiting | DoS risk | ConcurrencyLimit middleware |