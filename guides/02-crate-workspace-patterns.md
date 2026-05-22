# 02 — Crate and Workspace Patterns

**Purpose:** Guide agents to design and manage Cargo workspaces with proper dependency declarations, inter-crate boundaries, and publishing workflows.

---

## When to Use a Workspace

**Use a workspace** when:
- Multiple crates share types or logic (e.g., `core` + `api` + `client`)
- Crates are versioned and published together
- You want unified `cargo test --workspace` and `cargo clippy --workspace`
- Each crate has a distinct responsibility but shared dependencies

**Do not use a workspace** when:
- You have a single crate — simpler is better
- Crates are independently versioned and published to crates.io separately
- The added complexity of workspace membership is not justified

---

## Workspace Root Layout

```
my-workspace/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/               # Shared domain logic (rlib)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── api/                # HTTP server (binary)
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── client/             # Client SDK (rlib)
│       ├── Cargo.toml
│       └── src/lib.rs
└── README.md
```

---

## Workspace Root Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/api",
    "crates/client",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
rust-version = "1.85"

[workspace.dependencies]
# Core deps — versioned once, referenced everywhere
tokio = { version = "1.40", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"

# Dev deps
tokio = { version = "1.40", features = ["full", "test-util"] }

# Optional feature deps
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "tls-rustls"] }
axum = "0.8"
tower-http = { version = "0.6", features = ["trace", "cors", "compression"] }
```

**Key fields:**
- `resolver = "2"` — Enables feature resolvers in workspace
- `workspace.package` — Shared metadata inherited by all crates
- `workspace.dependencies` — Central version definition

---

## Per-Crate Cargo.toml

```toml
# crates/core/Cargo.toml
[package]
name = "my-core"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
uuid = { version = "1.10", features = ["v4", "serde"] }

[dev-dependencies]
tokio.workspace = true

[lib]
crate-type = ["rlib"]
path = "src/lib.rs"
```

```toml
# crates/api/Cargo.toml
[package]
name = "my-api"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true

[dependencies]
my-core = { path = "../core" }
tokio.workspace = true
axum.workspace = true
serde.workspace = true
anyhow.workspace = true
tracing.workspace = true
tower-http.workspace = true
uuid.workspace = true

[dev-dependencies]
my-core = { path = "../core", features = ["test-util"] }

[[bin]]
name = "my-api"
path = "src/main.rs"
```

```toml
# crates/client/Cargo.toml
[package]
name = "my-client"
version.workspace = true
edition.workspace = true
license.workspace = true
rust-version.workspace = true

[dependencies]
my-core = { path = "../core" }
tokio.workspace = true
reqwest = { version = "0.12", features = ["json"] }
serde.workspace = true
thiserror.workspace = true

[dev-dependencies]
tokio.workspace = true
wiremock = "0.7"
```

---

## The `workspace = true` Pattern

Instead of hardcoding versions in each crate:

```toml
# BAD: version drift risk
[dependencies]
tokio = "1.40"
thiserror = "2.0"

# GOOD: single source of truth in workspace root
[workspace.dependencies]
tokio = { version = "1.40", features = ["full"] }
thiserror = "2.0"

# Crate: reference via workspace = true
[dependencies]
tokio.workspace = true
thiserror.workspace = true
```

If a crate needs a different version of a dep (e.g., `client` needs `reqwest 0.11` while `api` needs `0.12`), it can override explicitly:

```toml
# crates/legacy/Cargo.toml
[dependencies]
reqwest = "0.11"  # Override workspace default
```

---

## Feature Flags

Each crate defines its own feature flags. Workspace features are additive.

```toml
# crates/core/Cargo.toml
[features]
default = []
test-util = ["tokio/sync"]
serde = ["dep:serde"]  # Conditionally enable serde

# Usage:
# cargo build -p my-core
# cargo build -p my-core --features serde
```

```toml
# crates/api/Cargo.toml
[features]
default = ["api-server"]
api-server = []
cli = ["clap"]

[dependencies]
clap = { version = "4.5", features = ["derive", "env"], optional = true }
```

---

## Avoiding Cyclic Dependencies

**Rust forbids cyclic dependencies between crates.** If you have A → B → A, one crate must be split or the shared code moved to a third crate.

```
# Forbidden: cyclic
crates/a/Cargo.toml: A depends on B
crates/b/Cargo.toml: B depends on A

# Solution: extract shared code to C
crates/c/Cargo.toml: C has no internal deps
crates/a/Cargo.toml: A depends on C
crates/b/Cargo.toml: B depends on C
```

**Dependency direction rules:**
- `core` — no internal deps, exports domain types and traits
- `api` — depends on `core`
- `client` — depends on `core`
- Never: `core` depends on `api` or `client`

---

## xtask Pattern

For complex build workflows, use the `xtask` pattern (cargo's own approach):

```
my-workspace/
├── Cargo.toml
├── xtask/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs     # Build-time tasks (codegen, formatting, etc.)
├── crates/
│   └── ...
```

```toml
# xtask/Cargo.toml
[package]
name = "xtask"
version = "0.1.0"
edition = "2024"

[dependencies]
cargo_metadata = "0.18"
clap = { version = "4.5", features = ["derive"] }
```

```rust
// xtask/src/main.rs
use clap::Parser;

#[derive(Parser)]
enum Xtask {
    GenerateBindings,
    RunTests,
}

fn main() {
    let cmd = Xtask::parse();
    match cmd {
        Xtask::GenerateBindings => { /* run bindgen */ }
        Xtask::RunTests => { /* run tests across workspace */ }
    }
}
```

**Usage:** `cargo xtask generate-bindings`

---

## Workspace-Wide Commands

```bash
# Build all crates
cargo build --workspace

# Build specific crate and its deps
cargo build -p my-core

# Test all crates
cargo test --workspace --all-features

# Test specific crate
cargo test -p my-core --all-features

# Clippy across workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Check all crates
cargo check --workspace --all-features

# Doc test across workspace
cargo test --workspace --doc

# Build release across workspace
cargo build --workspace --release

# Update all deps
cargo update --workspace

# List outdated deps
cargo outdated --workspace
```

---

## Versioning and Publishing

**When publishing workspace crates:**

```bash
# 1. Bump all versions together
cargo workspaces version bump --all

# 2. Publish in dependency order (bottomological)
# core has no internal deps → publish first
cargo publish -p my-core
cargo publish -p my-client
cargo publish -p my-api

# 3. Tag the release
git tag v0.2.0
git push origin v0.2.0
```

**Version consistency:** All workspace crates share the same version by convention. If `core` is `0.2.0`, `api` and `client` are also `0.2.0`.

**Semver rules for workspace crates:**
- Changing `core`'s public API is a breaking change → bump major version for all
- `api` changing internal implementation is not a breaking change
- Document which crate owns which semver contract

---

## MSRV in Workspaces

Each crate can declare its own MSRV, but it **cannot be lower** than the workspace floor:

```toml
[workspace.package]
rust-version = "1.85"

# crates/legacy/Cargo.toml — cannot declare 1.70
[package]
rust-version = "1.85"  # Must be >= workspace floor
```

**CI enforces MSRV** by running the workspace with the declared MSRV toolchain:

```yaml
msrv:
  runs-on: ubuntu-latest
  steps:
    - uses: dtolnay/rust-toolchain@1.85
    - run: cargo check --workspace --all-features
```

---

## Checklist

```
[ ] Workspace root has resolver = "2"
[ ] All crate versions reference workspace = true
[ ] Dependencies declared in workspace.dependencies
[ ] No circular dependencies between crates
[ ] Domain layer (core) has no internal deps
[ ] Each crate has explicit crate-type
[ ] Feature flags are additive (default = [])
[ ] CI checks MSRV with declared toolchain
[ ] Crates published in dependency order
[ ] Semver implications documented when changing core
```