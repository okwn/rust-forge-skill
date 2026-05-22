# {{workspace_name}}

A multi-crate Rust workspace for {{description}}.

## Structure

```
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/              # Shared domain logic
│   ├── api/               # HTTP server (axum)
│   └── client/            # Client SDK
└── README.md
```

## Building

```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p {{workspace_name}}-api

# Run tests
cargo test --workspace --all-features

# Run clippy
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Quality Gates

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-features
```

## Crates

### core
Shared domain types, error types, and business logic. No external dependencies except serde and thiserror.

### api
HTTP API server built with axum. Depends on core.

### client
Client SDK for consuming the API. Depends on core.

## License

MIT OR Apache-2.0