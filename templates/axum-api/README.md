# Axum API Template

Minimal API template using Axum framework without database.

## Route/Module Pattern

```
src/
├── main.rs        # Entry point, tracing init, tokio main, graceful shutdown
├── app.rs        # Compose router, add routes, add middleware
├── config.rs     # AppConfig { host, port } from env with defaults
├── error.rs      # AppError enum with Into<StatusCode>, From impls for anyhow
├── state.rs      # AppState struct with config field
├── http/
│   ├── mod.rs    # http module
│   ├── routes.rs # Router type alias, create_routes function
│   └── health.rs # health endpoint handlers (healthz, readyz, version)
```

## Routes

| Method | Path | Handler | Response |
|--------|------|---------|----------|
| GET | /api/v1/healthz | healthz | "ok" |
| GET | /api/v1/readyz | readyz | "ready" |
| GET | /api/v1/version | version | {"version": "0.1.0"} |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| HOST | 0.0.0.0 | Server bind host |
| PORT | 8080 | Server bind port |

## Features

- Request tracing via tower-http trace layer
- CORS middleware (allows all origins for development)
- Response compression
- Graceful shutdown on Ctrl+C or SIGTERM
- Typed application state (AppState)

## Running

```bash
# Development
cargo run

# Production
cargo build --release
./target/release/axum-api
```

## Testing

```bash
cargo test
```