# Agent Prompt: Scaffold an Axum API Project

You are an AI coding agent. Before writing any code, **read the rust-forge-skill** skill definition:

```
Read the skill at: rust-forge-skill/SKILL.md
```

---

## Task

Create a new Rust HTTP API project named `{{project_name}}` with the following requirements:

- **Project type:** REST API server
- **Description:** {{description}}
- **Framework:** Axum with `tokio`
- **Database:** PostgreSQL with `sqlx`
- **Features:**
  - CRUD for `Item` resource (id, name, description, created_at, updated_at)
  - Health endpoint at `GET /health`
  - `tower-http` middleware (tracing, CORS, compression)
  - Graceful shutdown
  - Layered architecture: `api → service → repository → domain`

---

## Steps

1. **Read** the relevant skill guides before writing any code:
   - `rust-forge-skill/guides/04-async-tokio-axum.md` — Async and Axum patterns
   - `rust-forge-skill/guides/01-project-architecture.md` — Layer architecture
   - `rust-forge-skill/guides/03-error-handling-anyhow-thiserror.md` — Error handling
   - `rust-forge-skill/guides/02-crate-workspace-patterns.md` — If multi-crate needed

2. **Select template:** Copy `rust-forge-skill/templates/axum-api/` to the target location

3. **Customize** the template:
   - Update `Cargo.toml` with project name, description, author, MSRV
   - Configure `sqlx` features for PostgreSQL
   - Update `README.md` with API documentation
   - Replace all `{{placeholder}}` strings

4. **Implement** the layered API structure:
   - `src/domain/` — `Item` model, domain errors (`thiserror`)
   - `src/repository/` — repository trait + `sqlx` implementation
   - `src/service/` — business logic, transaction handling
   - `src/api/` — Axum handlers, routing, middleware
   - `src/error.rs` — API boundary error handling (`anyhow` + `axum` response)
   - `src/config.rs` — environment variable parsing with fallible conversion

5. **Add** standard middleware:
   - `tower-http` tracing, CORS, compression
   - Health check endpoint
   - Graceful shutdown signal handling

6. **Implement REST endpoints:**

| Method | Path | Handler |
|---|---|---|
| `GET` | `/health` | Health check |
| `GET` | `/api/v1/items` | List items (paginated) |
| `POST` | `/api/v1/items` | Create item |
| `GET` | `/api/v1/items/:id` | Get item by ID |
| `PATCH` | `/api/v1/items/:id` | Update item |
| `DELETE` | `/api/v1/items/:id` | Delete item |

---

## Quality Requirements

These are non-negotiable. The deliverable is only accepted when all pass:

- **No `.unwrap()` or `.expect()` in production code**
- **Layer architecture:** API → Service → Repository → Domain (inward dependencies only)
- **Async traits use `async_trait`**
- **Connection pooling** with `PgPoolOptions` and timeouts
- **Error handling:** `thiserror` in domain, `anyhow` at API boundary
- **MSRV: 1.85.0**
- **All tests pass before delivery**

---

## Validation Commands

Run these commands in sequence. **All must pass.** Report the output of each.

```bash
cargo fmt --all -- --check
echo "=== FORMAT CHECK: PASS ==="

cargo clippy --workspace --all-targets --all-features -- -D warnings
echo "=== CLIPPY CHECK: PASS ==="

cargo test --workspace --all-features
echo "=== TEST CHECK: PASS ==="

cargo build --release
echo "=== BUILD: PASS ==="
```

---

## Deliverables

1. **Project structure** with:
   - `Cargo.toml` — with name, version, description, authors, MSRV
   - `README.md` — with build commands, env vars, API docs
   - `.gitignore`, `rustfmt.toml`, `.clippy.toml`, `.env.example`
   - `src/main.rs`, `src/config.rs`, `src/error.rs`
   - `src/domain/mod.rs`, `src/domain/item.rs`
   - `src/repository/mod.rs`, `src/repository/item.rs`
   - `src/service/mod.rs`, `src/service/item.rs`
   - `src/api/mod.rs`, `src/api/handlers/item.rs`, `src/api/routes.rs`
   - `tests/basic.rs`

2. **Validation output** — copy the terminal output of each validation command

3. **Summary** — 3–5 sentences describing what was created and how to run it

---

## Anti-Patterns That Fail Code Review

- `.unwrap()` in any non-test source file
- `anyhow` in domain layer (use `thiserror`)
- API layer directly calling repository (skipping service layer)
- No timeouts on database connections
- Missing graceful shutdown
- Connection string with credentials in source code
