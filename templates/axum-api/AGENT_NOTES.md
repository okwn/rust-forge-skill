# AGENT_NOTES.md

## Template Purpose

This template provides a minimal working Axum API without database integration. It serves as a starting point for API projects that do not require persistent storage.

## Key Files

- `src/main.rs` - Entry point with tracing and graceful shutdown
- `src/app.rs` - Router composition with middleware layers
- `src/state.rs` - Typed AppState holding configuration
- `src/http/` - HTTP route modules following separation of concerns

## Design Decisions

1. **No database** - This template intentionally excludes sqlx and repository patterns
2. **Typed state** - AppState is a concrete struct, not Arc<()>
3. **Minimal routes** - Only health check endpoints to demonstrate the pattern
4. **Edition 2024** - Uses latest Rust edition for modern syntax

## Extension Points

- Add new route modules under `src/http/` (e.g., `users.rs`, `items.rs`)
- Add service layer in `src/service.rs`
- Add domain models in `src/domain.rs`
- Add repository trait and implementations when adding database

## Dependencies

- axum 0.8 - Web framework
- tokio 1.40 (full) - Async runtime
- tower-http 0.6 (trace, cors, compression) - Middleware utilities
- tracing 0.1 - Structured logging
- tracing-subscriber 0.3 (env-filter, json) - Log formatting
- serde 1.0 - Serialization
- anyhow 1.0 - Error handling