# Agent Notes — Workspace Service Template

## Template Purpose
Scaffolds a multi-crate Cargo workspace for a service with shared core, client, and API crates. Use when building systems that need both a server binary and a client library.

## Instantiation Notes

### Workspace Structure
```
crates/
├── core/     — Shared domain models, errors, business logic (library)
├── api/      — Axum HTTP server binary
└── client/   — Client library for calling the service
Cargo.toml    — Workspace manifest
```

### Required Customisations
1. **`Cargo.toml`** — Update workspace members, package names, MSRV
2. **`crates/api/`** — Add routes matching your domain (currently placeholder)
3. **`crates/core/`** — Add domain models for your service
4. **`crates/client/`** — Configure the base URL for your service

### Workspace-Specific Validation
```bash
cargo build --workspace
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Workspace Commands
```bash
# Build all crates
cargo build --workspace

# Run the API binary
cargo run -p api

# Run tests for a specific crate
cargo test -p core

# Check all crates
cargo check --workspace
```

### Critical Rules
- Core crate has **no external I/O dependencies** (no sqlx, no reqwest)
- API crate depends on core + any I/O crates it needs
- Client crate depends on core only (no API dependencies)
- MSRV is enforced across all workspace members
