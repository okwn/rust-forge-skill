# 11 — Performance and Profiling

**Purpose:** Guide agents to write performant Rust code by measuring first, optimizing second, and following idiomatic patterns that don't sacrifice safety for speed. Premature optimization is the root of much evil — but uninformed coding choices can also create bottlenecks.

---

## The Measure-First Rule

**Never optimize without profiling data.** Rust code that looks inefficient may be optimized away by the compiler. Code that looks simple may hide a cache miss.

```bash
# Install profiling tools
cargo install cargo-flamegraph    # Linux perf-based
cargo install cargo-profdata       # For bedprofile data analysis
cargo install cargo-llvm-cov       # Code coverage

# Install system tools (Linux)
sudo apt-get install linux-perf  # for perf
```

---

## Benchmarking with Criterion

```toml
[[bench]]
name = "my_benchmark"
harness = false

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_process(c: &mut Criterion) {
    let mut group = c.benchmark_group("process");

    // Vary input size
    for size in [64, 256, 1024, 4096].iter() {
        let data: Vec<u8> = (0..*size).map(|i| i as u8).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let input = black_box(&data[..size]);
                process_data(input)
            })
        });
    }

    group.finish();
}

criterion_group![benches, bench_process];
criterion_main!(benches);
```

Run:
```bash
cargo bench
# Results in target/criterion/report/index.html
```

---

## Flamegraph Profiling

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flamegraph (requires Linux perf)
sudo perf record -a -- sleep 60 &
cargo flamegraph --bin my-binary -- --arg value

# For short-running processes
cargo flamegraph --bin my-binary -- input.json

# Custom frequency
cargo flamegraph --freq 1000 --bin my-binary
```

---

## Common Performance Mistakes

### 1. Allocating in Hot Paths

```rust
// BAD: allocation in loop
for item in items {
    results.push(format!("{}: {}", item.name, item.value));
}

// GOOD: pre-allocate or use write!
use std::fmt::Write;
let mut s = String::with_capacity(items.len() * 20);
for item in &items {
    write!(&mut s, "{}: {}; ", item.name, item.value).unwrap();
}
```

### 2. Cloning in Hot Paths

```rust
// BAD: clone in loop
for item in &items {
    let processed = heavy_function(item.clone());  // Unnecessary clone
    results.push(processed);
}

// GOOD: borrow
for item in &items {
    let processed = heavy_function(item);  // Borrow instead
    results.push(processed);
}

// Only clone when you genuinely need ownership:
for item in items {
    let processed = heavy_function(item);  // item is moved, no clone
    results.push(processed);
}
```

### 3. Using String Where &str Suffices

```rust
// BAD: String allocation in function signature
fn process(name: String) { ... }

// GOOD: borrow when you don't need ownership
fn process(name: &str) { ... }
```

### 4. Using format! in Logging

```rust
// BAD: allocation on every log call
tracing::info!("user {} logged in with email {}", user_id, email);

// GOOD: structured fields (zero allocation)
tracing::info!(
    user_id = %user_id,
    email = %email,
    "user logged in"
);
```

---

## Data Structure Choices

### Vec vs LinkedList

```rust
// BAD: LinkedList has poor cache locality
let mut list = LinkedList::new();
for i in 0..10000 { list.push_back(i); }
// Sequential iteration is pointer-chasing, not cache-friendly

// GOOD: Vec has excellent cache locality
let mut vec = Vec::new();
for i in 0..10000 { vec.push_back(i); }
// Sequential memory, cache-friendly iteration
```

### HashMap vs BTreeMap

```rust
// HashMap: O(1) average, O(n) worst case, requires Hash trait
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", 42);

// BTreeMap: O(log n) guaranteed, requires Ord trait
use std::collections::BTreeMap;
let mut map = BTreeMap::new();
map.insert("key", 42);

// Choose HashMap for single-key lookups, BTreeMap for ordered iteration
```

### SmallVec for Small Collections

```rust
// When you know most collections are small (< 4 elements)
use smallvec::{smallvec, SmallVec};

let mut vec: SmallVec<[u32; 4]> = smallvec![1, 2, 3];
vec.push(4);  // Still on stack
vec.push(5);  // Heap allocation only now
```

---

## Cow/Arc/Rc Guidance

```rust
// Cow: Clone-on-write for deferring allocations
fn normalize(input: &str) -> String {
    // If already lowercase, return borrowed; otherwise allocate
    if input.chars().all(|c| c.is_lowercase()) {
        input.to_string()  // Unnecessary allocation
    } else {
        input.to_lowercase()  // Necessary allocation
    }
}

// GOOD: use Cow to avoid unnecessary allocation
use std::borrow::Cow;

fn normalize(input: &str) -> Cow<str> {
    if input.chars().all(|c| c.is_lowercase()) {
        Cow::Borrowed(input)  // Zero allocation
    } else {
        Cow::Owned(input.to_lowercase())  // Allocate only when needed
    }
}
```

```rust
// Arc: shared ownership across threads
use std::sync::Arc;

struct Config {
    database_url: String,
    max_connections: u32,
}

let config = Arc::new(Config { ... });
// Multiple threads can read config without cloning
```

---

## Async Performance Mistakes

### Blocking in Async

```rust
// BAD: blocking sync I/O in async fn
async fn bad_read() -> String {
    std::fs::read_to_string("file.txt").unwrap()  // Blocks executor!
}

// GOOD: tokio::fs for async file I/O
async fn good_read() -> anyhow::Result<String> {
    tokio::fs::read_to_string("file.txt").await
}

// GOOD: spawn_blocking for CPU-bound work
async fn cpu_work() -> anyhow::Result<u64> {
    let result = tokio::task::spawn_blocking(|| {
        // CPU-intensive computation
        compute_primes(1_000_000)
    }).await?;
    Ok(result)
}
```

### Unnecessary Allocation in Async

```rust
// BAD: collecting to Vec when you can stream
async fn fetch_all(urls: Vec<Url>) -> Vec<Response> {
    let futures = urls.iter().map(|url| reqwest::get(url));
    let results: Vec<_> = futures.collect();  // Collects all futures first
    futures::future::join_all(results).await
}

// GOOD: stream with buffering
async fn fetch_all(urls: Vec<Url>) -> Vec<Response> {
    let client = reqwest::Client::new();
    let futures: Vec<_> = urls.into_iter()
        .map(|url| client.get(url).send())
        .map(|r| async move { r.await.unwrap() })
        .collect();
    futures::future::join_all(futures).await
}
```

---

## Allocation Tracking

```rust
// Use allocator API to track allocations in debug builds
#[global_allocator]
static ALLOC: std::alloc::System = std::alloc::System;

// Or use dhat for heap profiling
// dhat = "0.3"
// In code:
let _profiler = dhat::Profiler::new();
```

---

## Profiling Checklist

```
[ ] Benchmark before optimizing (criterion)
[ ] Flamegraph shows where time is spent
[ ] No allocations in hot paths (use with_capacity, write!)
[ ] Borrow instead of clone in hot paths
[ ] String vs &str used correctly (ownership semantics)
[ ] tracing uses structured fields, not format!()
[ ] tokio::fs used for async file I/O
[ ] spawn_blocking for CPU-bound or blocking sync work
[ ] Correct data structure for access pattern (Vec vs HashMap vs BTreeMap)
[ ] SmallVec for small collections
[ ] Cow used for clone-on-write patterns
[ ] Release build uses opt-level = 3, LTO
[ ] Profile with --release flag, not --dev
```