# 06 — Testing, Benchmarks, and Fuzzing

**Purpose:** Guide agents to write comprehensive Rust tests — unit tests, integration tests, doc tests, property-based tests, benchmarks, and fuzz targets. Testing is not optional in production Rust code.

---

## Test Organization

### Unit Tests (In-File)

```rust
// src/domain/models.rs
pub struct Item {
    pub name: String,
    pub quantity: u32,
}

impl Item {
    pub fn new(name: String, quantity: u32) -> Self {
        Self { name, quantity }
    }

    /// Validates the item is in a valid state.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if name is empty or quantity is zero.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.name.trim().is_empty() {
            return Err(ValidationError::EmptyName);
        }
        if self.quantity == 0 {
            return Err(ValidationError::ZeroQuantity);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_item() {
        let item = Item::new("Widget".to_string(), 10);
        assert!(item.validate().is_ok());
    }

    #[test]
    fn test_empty_name_rejected() {
        let item = Item::new("".to_string(), 10);
        assert!(matches!(item.validate(), Err(ValidationError::EmptyName)));
    }

    #[test]
    fn test_zero_quantity_rejected() {
        let item = Item::new("Widget".to_string(), 0);
        assert!(matches!(item.validate(), Err(ValidationError::ZeroQuantity)));
    }
}
```

### Integration Tests (tests/ directory)

```rust
// tests/integration.rs
use my_crate::{Item, Processor, Config};
use std::path::PathBuf;

#[test]
fn test_processor_batch() {
    let processor = Processor::new(Config::default());
    let items = vec![
        Item::new("Item 1".to_string(), 5),
        Item::new("Item 2".to_string(), 10),
    ];
    let results = processor.process_batch(items).unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_config_from_env() {
    std::env::set_var("MAX_ITEMS", "100");
    let config = Config::from_env().unwrap();
    assert_eq!(config.max_items, 100);
    std::env::remove_var("MAX_ITEMS");
}
```

### Doc Tests

```rust
/// Adds two integers.
///
/// # Examples
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

Run doc tests:
```bash
cargo test --doc
```

---

## Property-Based Testing with Proptest

```toml
[dev-dependencies]
proptest = "1.5"
```

```rust
// Property test for email validation
proptest! {
    #[test]
    fn test_email_validation_rejects_invalid(email: String) {
        // Only valid emails contain @
        if !email.contains('@') {
            let result = validate_email(&email);
            prop_assert!(result.is_err());
        }
    }
}

#[test]
fn test_email_validation_accepts_valid() {
    let valid_emails = [
        "user@example.com",
        "test+tag@domain.org",
        "a@b.co",
    ];
    for email in valid_emails {
        assert!(validate_email(email).is_ok(), "{}", email);
    }
}

// Custom strategies for complex types
use proptest::prelude::*;

prop_compose! {
    fn valid_email() -> String {
        let local = "[a-z]+";
        let domain = "[a-z]+\\.[a-z]+";
        format!("{}@{}", local, domain)
    }
}

proptest! {
    #[test]
    fn test_email_round_trip(email in valid_email()) {
        let parsed = parse_email(&email).unwrap();
        prop_assert_eq!(parsed.to_string(), email);
    }
}
```

---

## Async Tests

```rust
#[tokio::test]
async fn test_user_repository() {
    // Set up test database
    let pool = test_pool().await;
    let repo = PostgresUserRepository::new(pool);

    // Create a user
    let user = User::new("test@example.com".to_string());
    repo.insert(&user).await.unwrap();

    // Retrieve it
    let found = repo.find_by_email("test@example.com").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email, "test@example.com");
}

#[tokio::test]
async fn test_not_found_returns_none() {
    let pool = test_pool().await;
    let repo = PostgresUserRepository::new(pool);
    let found = repo.find_by_email("nonexistent@example.com").await.unwrap();
    assert!(found.is_none());
}
```

---

## Mocking with mockall

```toml
[dev-dependencies]
mockall = "0.13"
```

```rust
// repository/traits.rs
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError>;
    async fn insert(&self, user: &User) -> Result<(), UserError>;
}

// tests/mocks.rs
mockall::mock! {
    pub UserRepo {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError>;
        async fn insert(&self, user: &User) -> Result<(), UserError>;
    }
}

#[tokio::test]
async fn test_user_service_with_mock() {
    let mut repo = MockUserRepo::new();
    let user = User::new("test@example.com");

    repo.expect_insert()
        .times(1)
        .returning(|_| Ok(()));

    repo.expect_find_by_email()
        .returning(|_| Ok(Some(user.clone())));

    let service = UserService::new(&repo);
    let found = service.find_by_email("test@example.com").await.unwrap();
    assert_eq!(found.email, "test@example.com");
}
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

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort");

    // Baseline: sort_unstable
    group.bench_function("sort_unstable", |b| {
        let mut data: Vec<i32> = (0..1000).collect();
        b.iter(|| {
            let mut d = data.clone();
            d.sort_unstable();
            black_box(&mut d)
        })
    });

    // Compare with sort
    group.bench_function("sort", |b| {
        let mut data: Vec<i32> = (0..1000).collect();
        b.iter(|| {
            let mut d = data.clone();
            d.sort();
            black_box(&mut d)
        })
    });

    // Vary input size
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data: Vec<i32> = (0..*size).collect();
            b.iter(|| {
                let mut d = data.clone();
                d.sort_unstable();
                black_box(d)
            })
        });
    }

    group.finish();
}

criterion_group![benches, bench_sort];
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
# Results in target/criterion/report/index.html
```

---

## Fuzzing with cargo-fuzz

```toml
[package]
name = "my-fuzz"
edition = "2024"

[[bin]]
name = "fuzz_target"
path = "fuzz/fuzz_target.rs"
```

```toml
# Install cargo-fuzz
cargo install cargo-fuzz
```

```rust
// fuzz/fuzz_target.rs
#![no_main]

use libfuzzer_sys::fuzz_target;
use my_crate::{parse_config, Config};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Test parsing valid UTF-8
        let _ = parse_config(s);
    }
});

fuzz_target!(|data: &[u8]| {
    // Test binary protocol parsing
    let _ = my_crate::parse_binary_protocol(data);
});
```

```bash
# List fuzz targets
cargo fuzz list

# Run a specific target
cargo fuzz run parse_config

# Run with corpus
cargo fuzz run parse_config fuzz/corpus/
```

---

## Test Fixtures

```rust
// tests/fixtures.rs
use my_crate::*;

pub fn test_config() -> Config {
    Config {
        max_items: 100,
        timeout_secs: 5,
        ..Config::default()
    }
}

pub fn test_item() -> Item {
    Item::new("Test Item".to_string(), 10)
}

pub fn test_item_list() -> Vec<Item> {
    vec![
        Item::new("Item 1".to_string(), 5),
        Item::new("Item 2".to_string(), 10),
        Item::new("Item 3".to_string(), 15),
    ]
}

// tests/integration.rs
#[path = "fixtures.rs"]
mod fixtures;

#[test]
fn test_batch_process() {
    let config = fixtures::test_config();
    let processor = Processor::new(config);
    let items = fixtures::test_item_list();
    let results = processor.process_batch(items).unwrap();
    assert_eq!(results.len(), 3);
}
```

---

## Test Layout Reference

```
src/
├── lib.rs
├── domain/
│   ├── mod.rs
│   ├── models.rs      # Unit tests in #[cfg(test)] mod
│   └── error.rs
├── service/
│   ├── mod.rs
│   └── app_service.rs
└── repository/
    ├── mod.rs
    └── traits.rs
tests/
├── fixtures.rs       # Shared test utilities
├── integration.rs     # Integration tests
└── api_tests.rs       # API-level integration tests
benches/
├── bench_main.rs
├── sorting.rs
└── parsing.rs
fuzz/
├── fuzz_target.rs
└── corpus/
```

---

## Running Tests

```bash
# All tests
cargo test --workspace --all-features

# With output
cargo test --workspace --all-features -- --nocapture

# With nextest (faster)
cargo nextest run --workspace

# Specific crate
cargo test -p my-core --all-features

# Doc tests only
cargo test --workspace --doc

# Integration tests only
cargo test --workspace --test '*' -- --test-threads=4

# With coverage
cargo llvm-cov --workspace --html
```

---

## Checklist

```
[ ] Unit tests cover all public functions
[ ] Integration tests cover API/repository layer
[ ] Async tests use tokio::test
[ ] Error cases tested (not just happy path)
[ ] Property-based tests for input validation
[ ] Property-based tests for parsing functions
[ ] Benchmarks measure realistic workloads
[ ] Fuzz targets for parsers and input handlers
[ ] Test utilities do not use unwrap() in production-callable functions
[ ] Tests run in CI (cargo test --workspace --all-features)
[ ] Mocking done with mockall (not manual stubs)
```