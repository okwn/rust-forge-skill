# 03 — Error Handling: anyhow and thiserror

**Purpose:** Guide agents to choose the correct error handling strategy based on where in the architecture the error originates. Wrong error strategy in Rust leads to silent failures, lost context, and unhelpful messages.

---

## The Core Rule

```
anyhow   → Application/CLI boundary (entry point, main, HTTP handlers)
thiserror → Library/domain layer (code that will be reused by other crates)
Never mix them in the same crate without explicit, documented boundaries.
```

**Why?** `anyhow` is for ergonomics at boundaries where any error can happen and you just need to propagate it. `thiserror` is for domain modeling where errors are part of the domain contract.

---

## When to Use thiserror (Library/Domain)

**Use `thiserror` when:**
- Writing a library that will be published to crates.io
- Defining domain error types that callers will match against
- You need compile-time exhaustiveness checking on error variants
- Error types are part of your public API contract

```toml
# Cargo.toml
thiserror = "2.0"
```

```rust
// domain/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("item not found: {0}")]
    NotFound(Uuid),

    #[error("invalid name: {reason}")]
    InvalidName { reason: String },

    #[error("name already exists: {0}")]
    DuplicateName(String),

    #[error("database error: {context}")]
    Database { context: String, #[source] source: sqlx::Error },
}

impl ItemError {
    pub fn not_found(id: Uuid) -> Self {
        ItemError::NotFound(id)
    }

    pub fn invalid_name(reason: impl Into<String>) -> Self {
        ItemError::InvalidName { reason: reason.into() }
    }
}
```

**Key properties:**
- `#[derive(Error)]` generates `std::error::Error` trait
- `#[error("...")]` formats the display message
- `#[source]` chains the underlying cause
- Variants can carry structured data (`Uuid`, `reason`, `context`)

---

## When to Use anyhow (Application Boundary)

**Use `anyhow` when:**
- Writing the `main` function of a CLI tool
- Writing HTTP handler entry points
- Any error can happen and you just need to report it up
- You need to propagate errors from many different sources (std::io, serde, domain errors)

```toml
# Cargo.toml
anyhow = "1.0"
```

```rust
// main.rs or api handler
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config().context("failed to load config")?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("failed to connect to database")?;

    run_server(pool, config).await?;
    Ok(())
}

async fn run_server(pool: sqlx::PgPool, config: Config) -> Result<()> {
    // ...
    Ok(())
}
```

**Key methods:**
- `.context("message")` — adds context to any `Result<T, E>`
- `.with_context(|| "lazy message")` — evaluates closure only on error
- `anyhow!("formatted {}", args)` — creates an anyhow error directly
- `bail!("message")` — early return with anyhow error

---

## Error Propagation at Boundaries

When a library's `thiserror` type reaches an application boundary, convert it explicitly:

```rust
// api/handlers.rs
use domain::ItemError;
use axum::{response::IntoResponse, Json, http::StatusCode};
use serde_json::json;

enum AppError {
    Domain(ItemError),
    Internal(String),
}

impl From<ItemError> for AppError {
    fn from(e: ItemError) -> Self {
        AppError::Domain(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> impl IntoResponse {
        match self {
            AppError::Domain(ItemError::NotFound(id)) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("item {} not found", id) })),
            ),
            AppError::Domain(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": e.to_string() })),
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "internal error" })),
            ),
        }
    }
}

// In handler:
async fn get_item(Path(id): Path<Uuid>) -> Result<Json<Item>, AppError> {
    let item = service.find_item(id).await
        .map_err(AppError::from)?;  // Convert domain error → app error
    Ok(Json(item))
}
```

**Never** let raw `thiserror` types leak into HTTP responses — they may expose internal implementation details.

---

## Error Enum Design

**Good error enum design:**

```rust
#[derive(Debug, Error)]
pub enum ItemError {
    // No-argument variants for simple errors
    #[error("item not found")]
    NotFound,

    // Structured variants for errors with data
    #[error("item not found: {id}")]
    NotFoundById { id: Uuid },

    #[error("validation failed: {reason}")]
    Validation { reason: String },

    // Chained errors for infrastructure failures
    #[error("database error")]
    Database {
        #[from]
        source: sqlx::Error,
    },
}
```

**Anti-patterns in error enum design:**

```rust
// BAD: String-based error loses structure
#[error("{}", 0)]  // Never use index-based string formatting

// BAD: No variant differentiation
#[error("error occurred")]  // All variants same message

// BAD: Exposing internal paths in error messages
#[error("failed to query database at /path/to/file")]
```

---

## Combining Multiple Error Sources

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing environment variable: {0}")]
    MissingEnv(String),

    #[error("invalid integer for {key}: {value}")]
    InvalidInt { key: String, value: String },

    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("parse error")]
    Parse {
        #[from]
        source: std::num::ParseIntError,
    },
}

fn parse_config() -> Result<Config, ConfigError> {
    let port: u16 = std::env::var("PORT")
        .map_err(|_| ConfigError::MissingEnv("PORT".into()))?
        .parse()
        .map_err(|_| ConfigError::InvalidInt {
            key: "PORT".into(),
            value: std::env::var("PORT").unwrap(),
        })?;
    Ok(Config { port })
}
```

---

## Anti-Patterns in Error Handling

### 1. String as Error Type

```rust
// BAD — String loses type information
pub fn parse(input: &str) -> Result<Data, String> {
    if input.is_empty() {
        Err("input cannot be empty".to_string())
    } else {
        Ok(Data {})
    }
}

// GOOD — typed error enum
pub fn parse(input: &str) -> Result<Data, ParseError> {
    if input.is_empty() {
        Err(ParseError::EmptyInput)
    } else {
        Ok(Data {})
    }
}
```

### 2. Panic for Recoverable Failures

```rust
// BAD — panic is not error handling
let value = config.get("key").unwrap();  // panics if missing

// GOOD — propagate as Result
let value = config.get("key")
    .ok_or_else(|| ConfigError::Missing("key".into()))?;

// GOOD — provide default with logging
let value = config.get("key")
    .unwrap_or_else(|| {
        tracing::warn!("key not found, using default");
        Default::default()
    });
```

### 3. Unwrap Chains

```rust
// BAD — nested unwraps
let value = data.foo.bar.baz.unwrap();

// GOOD — use ? operator with clear error types
let value = data.foo
    .ok_or(DomainError::MissingFoo)?
    .bar
    .ok_or(DomainError::MissingBar)?
    .baz
    .ok_or(DomainError::MissingBaz)?;
```

### 4. Losing Error Context

```rust
// BAD — error chain broken
fn read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("file.txt")  // No context about which file
}

// GOOD — context added at boundary
fn read_config() -> anyhow::Result<Config> {
    let contents = std::fs::read_to_string("config.toml")
        .context("failed to read config.toml")?;
    // ...
}
```

### 5. Using `anyhow` in Library Code

```rust
// BAD — library using anyhow loses type info for consumers
pub fn parse(input: &str) -> Result<Data, anyhow::Error> { ... }

// GOOD — library uses typed errors
pub fn parse(input: &str) -> Result<Data, ParseError> { ... }
```

### 6. Bare `Box<dyn Error>`

```rust
// BAD — loses all type information
pub fn foo() -> Result<(), Box<dyn std::error::Error>> { ... }

// GOOD — concrete error enum
pub fn foo() -> Result<(), FooError> { ... }
```

### 7. Swallowing Errors

```rust
// BAD — error silently ignored
fn process(data: &[u8]) {
    parse(data).ok();  // Error lost!
}

// GOOD — errors are propagated or handled explicitly
fn process(data: &[u8]) -> Result<(), ProcessError> {
    parse(data).map_err(|e| ProcessError::Parse(e))?;
    // ...
}
```

---

## Error Handling Checklist

```
[ ] Library code uses thiserror for domain errors
[ ] Application/CLI boundary uses anyhow
[ ] Error conversion at boundaries is explicit (Into / From)
[ ] No String-based error types in domain layer
[ ] No .unwrap() in production code paths
[ ] No panic for recoverable errors
[ ] Error messages are user-friendly (no internal paths)
[ ] Error enums have structured variants where data matters
[ ] #[source] chains are used for infrastructure errors
[ ] Error context is preserved through ? propagation
[ ] Tests cover error paths, not just happy path
```