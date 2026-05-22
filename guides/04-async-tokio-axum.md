# 04 — Async Runtime: Tokio + Axum

**Purpose:** Guide agents to build async Rust applications with tokio and axum. Async Rust has strict rules about blocking, cancellation, and state management. Violations cause deadlocks, starvation, and hard-to-debug failures.

---

## When Async Is Justified

**Use async when:**
- Building HTTP servers (axum, actix-web)
- Building database clients with connection pooling (sqlx, deadpool-postgres)
- Building clients that make many concurrent external calls (reqwest)
- Building long-running services with multiple concurrent tasks

**Do not use async when:**
- Building simple CLI tools that do one sequential task and exit
- Writing CPU-bound computation (use `rayon` or sync Rust instead)
- The complexity of async is not justified by the workload

**Rule:** Do not introduce `tokio` into a project unless the framework or workload requires it.

---

## Tokio Runtime Configuration

```toml
[dependencies]
tokio = { version = "1.40", features = ["full"] }
```

```rust
// main.rs
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Runtime is created implicitly
    run().await
}
```

**Runtime flavor selection:**

```toml
# Light async — no threads needed
tokio = { version = "1.40", features = ["rt"] }

# Multi-thread (default for servers)
tokio = { version = "1.40", features = ["full"] }

# Current-thread (embedded, simple tools)
tokio = { version = "1.40", features = ["rt", "macros"] }
```

**Never call `block_on` inside an async context** — it deadlocks the executor:

```rust
// BAD — blocks the executor thread
async fn bad() {
    let result = tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(1));
        42
    }).await.unwrap();
}

// GOOD — await the blocking task
async fn good() {
    let result = tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_secs(1));
        42
    }).await.unwrap();
}
```

---

## Axum Router Structure

```rust
use axum::{
    routing::{get, post, delete, patch},
    Router,
};
use tower_http::trace::TraceLayer;
use std::sync::Arc;

// Define state
struct AppState {
    db: DbPool,
    cache: CachePool,
}

// Build router
fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/api/v1", api_v1_routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn api_v1_routes() -> Router {
    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item).patch(update_item).delete(delete_item))
}
```

---

## State Management

**Rule:** State is passed via `.with_state(Arc<AppState>)`. Never use global mutable state.

```rust
// State definition
struct AppState {
    db: PgPool,
    redis: RedisPool,
    config: Config,
}

// In main
let state = Arc::new(AppState::new(&config).await?);
let app = Router::new()
    .route("/users", get(list_users))
    .with_state(state);

// In handler
async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = state.db.list_users().await?;
    Ok(Json(users))
}
```

**Never store non-`Send` types in `Arc<AppState>`** if using the multi-thread runtime.

---

## Graceful Shutdown

```rust
use tokio::signal;

async fn run_server() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .into_make_service();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("server shut down gracefully");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
```

---

## Timeouts

**Every async operation that can hang must have a timeout.**

```rust
use tokio::time::{timeout, Duration};

// Timeout a database query
let result = timeout(Duration::from_secs(5), state.db.query_user(id))
    .await
    .map_err(|_| AppError::Timeout)?;

// Timeout an HTTP call
let response = timeout(
    Duration::from_secs(10),
    reqwest.get(&url).send()
).await??;
```

**Common timeout values:**
- Database queries: 5–30s
- External HTTP calls: 10–60s
- Health checks: 2–5s

---

## Cancellation

**Async Rust tasks are cancellable.** When `.await` is interrupted, local variables drop, but spawned tasks keep running unless explicitly cancelled.

```rust
// BAD: spawned task outlives its intended scope
async fn spawn_without_tracking() {
    tokio::spawn(async {
        // This might run long after the function returns
        tokio::time::sleep(Duration::from_secs(100)).await;
        tracing::info!("done");
    });
}

// GOOD: track the handle and abort if needed
async fn spawn_with_tracking() -> anyhow::Result<()> {
    let handle = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(100)).await;
        tracing::info!("done");
    });

    // If we need to cancel:
    handle.abort();
    handle.await?;  // Calling abort() then await gives JoinError

    Ok(())
}
```

---

## Avoiding Blocking in Async

**The async executor has a limited number of threads.** Blocking a thread starves other tasks.

```rust
// BAD: blocking sync I/O in async fn
async fn bad_read() -> String {
    std::fs::read_to_string("file.txt").unwrap()  // BLOCKS the executor!
}

// GOOD: use tokio::fs for async file I/O
async fn good_read() -> anyhow::Result<String> {
    tokio::fs::read_to_string("file.txt").await
}

// GOOD: use spawn_blocking for truly blocking operations
async fn blocking_operation() -> anyhow::Result<()> {
    let result = tokio::task::spawn_blocking(|| {
        // CPU-intensive or blocking sync code here
        std::thread::sleep(std::time::Duration::from_secs(1));
        42
    }).await?;
    Ok(())
}
```

**When to use `spawn_blocking`:**
- File I/O with `std::fs` (use `tokio::fs` if possible)
- CPU-intensive computation (crypto, image processing)
- Calling blocking C libraries

---

## Async Traits

```rust
// BAD: async fn in trait without async_trait
trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
}

// GOOD: use async_trait
use async_trait::async_trait;

#[async_trait]
trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
    async fn insert(&self, user: &User) -> Result<(), Error>;
}

// Implementation
struct PostgresUserRepo { pool: PgPool }

#[async_trait]
impl UserRepository for PostgresUserRepo {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        sqlx::query_as!("SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }

    async fn insert(&self, user: &User) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO users (id, email) VALUES ($1, $2)",
            user.id,
            user.email
        ).execute(&self.pool).await
        .map_err(|e| Error::Database(e.to_string()))?;
        Ok(())
    }
}
```

---

## Connection Pooling

```rust
use sqlx::postgres::PgPoolOptions;

async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await?;
    Ok(pool)
}
```

**Pool configuration rules:**
- `max_connections`: 10–50 for typical workloads
- `min_connections`: 2–5 for avoiding cold start latency
- `acquire_timeout`: 30s — how long to wait for a connection
- `idle_timeout`: 300–600s — connection recycling

---

## Tracing in Async Context

```rust
use tracing::{instrument, info, error};

#[instrument(skip(repo), fields(user_id = %user_id))]
async fn create_user<R: UserRepository>(
    repo: &R,
    user_id: Uuid,
    email: String,
) -> Result<User, UserError> {
    info!("creating user");
    let user = User::new(user_id, email)?;
    repo.insert(&user).await?;
    info!(user_id = %user_id, "user created");
    Ok(user)
}
```

**Structured fields in tracing:**
```rust
tracing::info!(
    user_id = %user_id,
    email = %email,
    duration_ms = elapsed.as_millis() as u64,
    "operation completed"
);
```

---

## Checklist

```
[ ] Tokio used only when async is justified
[ ] No block_on inside async context
[ ] All async operations have timeouts
[ ] Graceful shutdown implemented
[ ] State passed via Arc<AppState>, not global
[ ] spawn_blocking for blocking sync operations
[ ] async_trait on all async traits
[ ] Connection pooling configured (PgPoolOptions)
[ ] No mutable global state (use Arc<Mutex<T>> if needed)
[ ] tracing used instead of println! in async code
[ ] spawned tasks are tracked or aborted explicitly
```