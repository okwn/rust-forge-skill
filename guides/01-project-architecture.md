# 01 — Project Architecture

**Purpose:** Guide agents to design Rust projects with clean layer separation, explicit module boundaries, and idiomatic organization. Rust's module system enforces architecture — use it.

---

## The Binary-Library Split

Every non-trivial Rust project should have both a library and a binary.

```
my-project/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Public API, reusable
│   └── main.rs         # Binary entry, minimal
```

**Why:**
- `lib.rs` can be tested without a binary
- Other crates can depend on the library
- `main.rs` stays thin (handles CLI parsing, tracing init, config loading)

```rust
// src/lib.rs
pub mod domain;
pub mod service;
pub mod error;

pub use domain::models::Item;
pub use error::{Error, ValidationError};

// src/main.rs
use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("starting application");

    let state = my_project::service::AppState::new().await?;
    my_project::api::run(state).await
}
```

**Rule:** `main.rs` must be under 100 lines. If it's larger, extract logic to `lib.rs` modules.

---

## Layer Architecture

```
┌─────────────────────────────────────┐
│           API / Routes              │  ← HTTP handlers, CLI parsing, input validation
├─────────────────────────────────────┤
│          Service Layer              │  ← Business logic, orchestration
├─────────────────────────────────────┤
│     Repository / Data Access        │  ← DB queries, external HTTP calls
├─────────────────────────────────────┤
│          Domain / Models            │  ← Types, interfaces, domain rules, error enums
├─────────────────────────────────────┤
│      Infrastructure / Adapters      │  ← DB client, cache, email, file system
└─────────────────────────────────────┘
```

**Dependency rule:** Outer layers depend on inner layers. **Inner layers have zero dependencies on outer layers.**

```rust
// domain/ — no external runtime deps
pub mod models;
pub mod error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("name cannot be empty")]
    EmptyName,
    #[error("name too long: {actual} > {max}")]
    NameTooLong { max: usize, actual: usize },
}

// service/ — depends only on domain traits
pub struct ItemService<R: ItemRepository> {
    repo: R,
}

impl<R: ItemRepository> ItemService<R> {
    pub async fn create_item(&self, name: String) -> Result<Item, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation(ValidationError::EmptyName));
        }
        let item = Item::new(name);
        self.repo.insert(&item).await?;
        Ok(item)
    }
}

// repository/ — defines traits in domain, implements in infrastructure
pub trait ItemRepository: Send + Sync {
    async fn insert(&self, item: &Item) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Item>, DomainError>;
}

// infrastructure/ — implements repository trait with sqlx
pub struct SqlxItemRepository { pool: sqlx::PgPool }
```

---

## Recommended Layouts by Project Type

### CLI Application

```
my-cli/
├── src/
│   ├── main.rs              # Entry: parse CLI, init tracing, call run()
│   ├── lib.rs              # Re-exports for testing
│   ├── commands/           # Subcommand implementations
│   │   ├── mod.rs
│   │   ├── serve.rs
│   │   └── process.rs
│   ├── config.rs           # Config loading from env/files
│   └── error.rs            # anyhow-based app errors
├── Cargo.toml
└── README.md
```

- `main.rs`: 50–80 lines. Parses CLI, initializes logging, calls command.
- `commands/`: One file per subcommand. Keep each under 100 lines.
- `error.rs`: Uses `anyhow` for ergonomic error propagation at CLI boundary.
- Domain types live in `lib.rs` if shared, else in the relevant command module.

### HTTP API (Axum)

```
my-api/
├── src/
│   ├── main.rs              # Entry: app building, middleware, bind
│   ├── lib.rs               # App state, routes, error mapping
│   ├── api/
│   │   ├── mod.rs
│   │   ├── routes.rs         # Router::nest() structure
│   │   └── handlers.rs       # HTTP handlers (Path, Query, Json extractors)
│   ├── service/
│   │   ├── mod.rs
│   │   └── app_service.rs   # Business logic
│   ├── repository/
│   │   ├── mod.rs
│   │   └── item_repo.rs      # sqlx implementation
│   ├── domain/
│   │   ├── mod.rs
│   │   └── models.rs         # Item, CreateItemRequest, etc.
│   ├── config.rs            # Env variables (DATABASE_URL, SERVER_PORT)
│   └── error.rs             # AppError enum with IntoResponse impl
├── migrations/
│   └── 001_create_items.sql
└── Cargo.toml
```

### Library Crate

```
my-lib/
├── src/
│   ├── lib.rs               # Public re-exports, prelude
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   └── error.rs         # thiserror enum
│   ├── service/
│   │   ├── mod.rs
│   │   └── processor.rs
│   └── repository/
│       ├── mod.rs
│       └── traits.rs        # Repository trait definitions
├── tests/
│   └── integration.rs
└── Cargo.toml
```

- **No `tokio`, `axum`, `sqlx` in domain/service layers.** Only `thiserror` and `serde`.
- Feature flags gate optional dependencies (DB, async runtime).
- `Cargo.toml` declares MSRV explicitly.

### Worker / Background Task

```
my-worker/
├── src/
│   ├── main.rs              # Entry: tokio::main, graceful shutdown
│   ├── lib.rs
│   ├── processor.rs         # Worker logic (sync or async)
│   ├── queue/               # Job queue consumer
│   │   ├── mod.rs
│   │   └── consumer.rs
│   ├── metrics.rs          # Prometheus / OpenTelemetry metrics
│   └── error.rs
└── Cargo.toml
```

- Workers often run sync code in async context — use `tokio::task::spawn_blocking`.
- Implement graceful shutdown via `tokio::select!` + shutdown signal.
- Expose `/health` and `/metrics` endpoints for orchestration.

---

## Small Modules Rule

**Maximum file size: 300 lines.** Maximum function size: 50 lines.

If a file exceeds 300 lines, split it. Common splits:

```
main.rs (80 lines)
  → extract CLI parsing to commands/mod.rs
  → extract config loading to config.rs
  → extract error mapping to error.rs

item_service.rs (400 lines)
  → extract validation to domain/validators.rs
  → extract persistence logic to repository/traits.rs
  → extract serialization to domain/dto.rs
```

**Rule:** If a module name would be `utils`, it's a code smell. Split `utils.rs` into `config.rs`, `parser.rs`, `formatter.rs`.

---

## Avoiding God Objects

A "god object" is a struct that knows too much, does too much, and has too many fields.

**Anti-pattern:**
```rust
struct App {
    db: PgPool,
    cache: RedisPool,
    http: Client,
    config: Config,
    logger: Logger,
    // 20 more fields...
}
```

**Preferred: Explicit state passing**

```rust
struct AppState {
    db: DbPool,
    cache: CachePool,
}

// Passed explicitly to handlers
async fn get_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Item>, AppError> { ... }
```

**Or: Group related state into sub-structs**

```rust
struct DatabaseState { pool: PgPool }
struct CacheState { pool: RedisPool }
struct AppState { db: DatabaseState, cache: CacheState }
```

---

## Config Boundaries

**All configuration comes from environment variables.** No hardcoded values.

```rust
// config.rs — use envy for typed env loading
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing environment variable: {0}")]
    Missing(String),
    #[error("invalid value: {0}")]
    Invalid(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| ConfigError::Missing("DATABASE_URL".into()))?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".into()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .map_err(|_| ConfigError::Invalid("SERVER_PORT".into()))?,
            log_level: std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".into()),
        })
    }
}
```

**Never use `std::env::var("...").unwrap()`** in production code — always handle the error.

---

## IO Boundaries

IO operations (DB, HTTP, file system) must be isolated:

1. **Repository pattern**: Define a trait in `domain/`, implement it in `infrastructure/`.
2. **No IO in domain/service layers**: Business logic operates on in-memory types.
3. **Async boundaries**: Use `async_trait` for async repository methods.

```rust
// domain/repository/traits.rs
#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Item>, DomainError>;
    async fn insert(&self, item: &Item) -> Result<(), DomainError>;
}

// infrastructure/db.rs
pub struct PostgresItemRepository { pool: PgPool }

#[async_trait]
impl ItemRepository for PostgresItemRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Item>, DomainError> {
        sqlx::query_as!(Item, "SELECT * FROM items WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::Database(e.to_string()))
    }
    // ...
}
```

---

## Ports and Adapters (Hexagonal)

For complex domains, apply hexagonal architecture:

```
src/
├── domain/           # Pure business logic, no external deps
│   ├── models.rs     # Entity definitions
│   ├── ports.rs      # Trait definitions (interfaces)
│   └── errors.rs      # Domain error types
├── application/      # Use cases / services
│   └── use_cases.rs
├── adapters/         # Implementations of ports
│   ├── primary/      # Inbound (HTTP, CLI, gRPC)
│   │   └── axum_adapter.rs
│   └── secondary/    # Outbound (DB, cache, external APIs)
│       └── postgres_adapter.rs
└── lib.rs
```

**Benefit:** Domain logic is testable without any infrastructure. Swap adapters without changing business rules.

---

## Checklist

```
[ ] Binary/library split exists (src/lib.rs + src/main.rs)
[ ] Layer dependencies flow inward only
[ ] Domain layer has zero tokio/sqlx/axum dependencies
[ ] No file exceeds 300 lines
[ ] No function exceeds 50 lines
[ ] Config loaded from environment variables
[ ] IO operations use repository pattern
[ ] State passed explicitly (no global mutable state)
[ ] No god objects (structs with 20+ fields)
```