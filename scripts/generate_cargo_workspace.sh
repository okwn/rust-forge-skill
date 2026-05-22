#!/usr/bin/env bash
# generate_cargo_workspace.sh
# Generate a clean Rust workspace skeleton.
#
# Usage:
#   ./scripts/generate_cargo_workspace.sh <name>
#
# Creates a new directory <name>/ with a full workspace layout:
#   <name>/
#     Cargo.toml              (workspace root)
#     rust-toolchain.toml     (stable)
#     rustfmt.toml            (2024 edition)
#     .gitignore
#     crates/
#       domain/                (core business logic)
#       infra/                 (database, external services)
#       api/                   (HTTP / API layer)
#     apps/
#       server/                (binary, wires everything together)
#     xtask/                   (build helper tasks)

set -euo pipefail

# ── Argument validation ────────────────────────────────────────────────────

WORKSPACE_NAME="${1:-}"

if [[ -z "$WORKSPACE_NAME" ]]; then
    echo "Usage: $0 <workspace-name>"
    echo ""
    echo "Example:"
    echo "  $0 my-service"
    exit 1
fi

if [[ ! "$WORKSPACE_NAME" =~ ^[a-zA-Z][a-zA-Z0-9_-]*$ ]]; then
    echo "Error: workspace name must start with a letter and contain only letters, digits, underscores, and hyphens"
    exit 1
fi

# ── Helper functions ────────────────────────────────────────────────────────

create_lib_crate() {
    local crate_path="$1"
    local crate_name="$2"

    mkdir -p "$crate_path/src"

    # Use //! for module-level doc comment (inner doc, documents this item)
    cat > "$crate_path/src/lib.rs" << 'EOF'
//! Library root.
EOF
}

# ── Create workspace ───────────────────────────────────────────────────────

echo "=== Generating Rust workspace: ${WORKSPACE_NAME} ==="
echo ""

if [[ -d "$WORKSPACE_NAME" ]]; then
    echo "Error: directory already exists: ${WORKSPACE_NAME}/"
    echo "Remove it first or choose a different name."
    exit 1
fi

mkdir -p "${WORKSPACE_NAME}"

# Root Cargo.toml
cat > "${WORKSPACE_NAME}/Cargo.toml" << 'TOML_EOF'
[workspace]
resolver = "2"
members = [
    "crates/domain",
    "crates/infra",
    "crates/api",
    "apps/server",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
authors = ["{{author}}"]

[workspace.dependencies]
tokio = { version = "1.40", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"
TOML_EOF

# rust-toolchain.toml
cat > "${WORKSPACE_NAME}/rust-toolchain.toml" << 'TOML_EOF'
[toolchain]
channel = "stable"
TOML_EOF

# rustfmt.toml
cat > "${WORKSPACE_NAME}/rustfmt.toml" << 'TOML_EOF'
edition = "2024"
normalize_comments = true
wrap_comments = true
comment_width = 100
format_code_in_doc_comments = true
max_width = 100
tab_spaces = 4
TOML_EOF

# .gitignore
cat > "${WORKSPACE_NAME}/.gitignore" << 'EOF'
target/
**/*.rs.bak
.env
.env.local
.DS_Store
*.swp
*.swo
*~
.vscode/
.idea/
*.log
*.tmp
/crates/*/target/
/apps/*/target/
EOF

# .clippy.toml
cat > "${WORKSPACE_NAME}/.clippy.toml" << 'TOML_EOF'
msrv = "1.85"
cognitive_complexity = 15
TOML_EOF

# ── Crates ─────────────────────────────────────────────────────────────────

create_lib_crate "${WORKSPACE_NAME}/crates/domain" "domain"
cat > "${WORKSPACE_NAME}/crates/domain/Cargo.toml" << 'TOML_EOF'
[package]
name = "domain"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
TOML_EOF

create_lib_crate "${WORKSPACE_NAME}/crates/infra" "infra"
cat > "${WORKSPACE_NAME}/crates/infra/Cargo.toml" << 'TOML_EOF'
[package]
name = "infra"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
TOML_EOF

create_lib_crate "${WORKSPACE_NAME}/crates/api" "api"
cat > "${WORKSPACE_NAME}/crates/api/Cargo.toml" << 'TOML_EOF'
[package]
name = "api"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
domain = { path = "../domain" }
infra = { path = "../infra" }
tokio = { workspace = true }
anyhow = { workspace = true }
TOML_EOF

# ── Apps ───────────────────────────────────────────────────────────────────

mkdir -p "${WORKSPACE_NAME}/apps/server/src"

cat > "${WORKSPACE_NAME}/apps/server/Cargo.toml" << 'TOML_EOF'
[package]
name = "server"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[[bin]]
name = "server"
path = "src/main.rs"

[dependencies]
domain = { path = "../../crates/domain" }
infra = { path = "../../crates/infra" }
api = { path = "../../crates/api" }
tokio = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
TOML_EOF

cat > "${WORKSPACE_NAME}/apps/server/src/main.rs" << 'EOF'
//! Server application entry point.

fn main() {
    println!("Server binary — wire up api, infra, and domain crates here");
}
EOF

# ── Xtask ─────────────────────────────────────────────────────────────────

mkdir -p "${WORKSPACE_NAME}/xtask/src"

cat > "${WORKSPACE_NAME}/xtask/Cargo.toml" << 'TOML_EOF'
[package]
name = "xtask"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[[bin]]
name = "xtask"
path = "src/main.rs"

[dependencies]
toml = "0.8"
walkdir = "2.5"
TOML_EOF

cat > "${WORKSPACE_NAME}/xtask/src/main.rs" << 'EOF'
//! Build helper tasks.
//
//! Run with: cargo xtask <task>
//
//! Available tasks:
//!   check-migrations  - Validate database migrations
//!   generate-schema  - Generate types from schema
//!   lint-fix          - Run cargo fmt and clippy --fix

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        None => {
            eprintln!("Usage: xtask <task>");
            eprintln!("Tasks: check-migrations, generate-schema, lint-fix");
        }
        Some("lint-fix") => {
            println!("Running format and clippy fix...");
            std::process::Command::new("cargo")
                .args(["fmt", "--all"])
                .status()
                .expect("cargo fmt failed");
            std::process::Command::new("cargo")
                .args(["clippy", "--workspace", "--all-targets", "--", "-W", "clippy::all"])
                .status()
                .expect("cargo clippy failed");
        }
        Some(task) => {
            eprintln!("Unknown task: {task}");
            std::process::exit(1);
        }
    }
}
EOF

# ── Readme ─────────────────────────────────────────────────────────────────

cat > "${WORKSPACE_NAME}/README.md" << 'EOF'
# Workspace

A multi-crate Rust workspace following hexagonal/clean architecture patterns.

## Structure

```
<name>/
├── Cargo.toml              ← workspace root
├── rust-toolchain.toml     ← pinned Rust toolchain
├── rustfmt.toml           ← formatting rules (2024 edition)
├── .gitignore
├── .clippy.toml           ← lint rules
│
├── crates/
│   ├── domain/             ← core business logic, no external deps
│   ├── infra/              ← database, cache, external services
│   └── api/                ← HTTP handlers, middleware
│
├── apps/
│   └── server/             ← binary that wires everything together
│
└── xtask/                  ← build helper tasks (cargo xtask <task>)
```

## Crate responsibilities

| Crate | Responsibility | Key rules |
|-------|----------------|-----------|
| `domain` | Business entities, pure logic, error types | No infra deps. No `anyhow`. |
| `infra` | DB access, external APIs, I/O | Depends on `domain`. |
| `api` | HTTP routing, middleware, JSON | Depends on `domain` + `infra`. |
| `server` | Application entry, wiring | Depends on all crates. |

## Adding a new crate

1. Create `crates/<name>/src/lib.rs`
2. Add to `[workspace] members` in `Cargo.toml`
3. Add `[dependencies]` section to `Cargo.toml` in workspace root

## Building

```bash
cargo build --workspace
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Xtask

```bash
cargo xtask lint-fix        # Format + auto-fix clippy
```

## License

MIT OR Apache-2.0
EOF

# ── Verify compilation ──────────────────────────────────────────────────────

echo ""
echo "=== Verifying workspace compiles ==="
echo ""

cd "$WORKSPACE_NAME"
if cargo check --workspace 2>&1; then
    echo ""
    echo "✓ Workspace compiles cleanly"
else
    echo ""
    echo "⚠ Workspace has compilation errors — review above"
fi

# ── Done ───────────────────────────────────────────────────────────────────

echo ""
echo "=== Workspace generated successfully ==="
echo ""
echo "Location:  ./${WORKSPACE_NAME}/"
echo "Crates:    domain, infra, api"
echo "Apps:      server"
echo "Xtask:     xtask"
echo ""
echo "Next steps:"
echo ""
echo "  cd ${WORKSPACE_NAME}"
echo "  cargo build --workspace"
echo "  cargo test --workspace"
echo ""
echo "  # Add a new crate:"
echo "  mkdir crates/my-crate && cargo new --bare crates/my-crate"
echo "  # then add 'crates/my-crate' to [workspace] members in Cargo.toml"
echo ""