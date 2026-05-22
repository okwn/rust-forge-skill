# 10 — Release, CI, and MSRV Policy

**Purpose:** Guide agents to configure Rust projects for production releases with MSRV enforcement, comprehensive CI/CD pipelines, security gates, and proper release workflows.

---

## MSRV (Minimum Supported Rust Version)

**Default MSRV: 1.85.0** (Rust 2024 edition baseline)

**Why MSRV matters:**
- Libraries with MSRV 1.85 cannot be used by projects on 1.70
- Breaking changes in Rust compiler may require MSRV bumps
- CI enforces MSRV to catch compatibility issues early

### Declaring MSRV

```toml
# Cargo.toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
```

**For workspace crates:**
```toml
[workspace.package]
rust-version = "1.85"

[package]
rust-version.workspace = true
```

### rust-toolchain.toml

```toml
# rust-toolchain.toml (in repo root)
[toolchain]
channel = "1.85"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-gnu"]
```

**This file is auto-read by Cargo.** It pins CI to the correct toolchain.

### Override MSRV for specific crates

```bash
# Override for a single crate
RUST_MSRV=1.82.0 cargo check -p my-crate
```

---

## CI Pipeline

### Standard Rust CI

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  fmt:
    name: Rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy --workspace --all-targets --all-features -- -D warnings

  test:
    name: Test Suite
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo test --workspace --all-features -- --nocapture
      - uses: codecov/codecov-action@v4
        with:
          token: ${{ secrets.CODECOV_TOKEN }}

  doc:
    name: Documentation
    runs-on: ubuntu-latest
    if: ${{ github.event_name == 'push' }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo doc --workspace --all-features --no-deps

  msrv:
    name: MSRV Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.85
      - run: |
          cargo check --workspace --all-features
          cargo build --workspace --all-features
          cargo test --workspace --all-features
```

### Security CI

```yaml
# .github/workflows/security.yml
name: Security

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v2
        with:
          token: ${{ secrets.GITHUB_TOKEN }}

  cargo-deny:
    name: Cargo Deny
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/cargo-deny-action@v1
        with:
          command: check advisories licenses ban

  unsafe-audit:
    name: Unsafe Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: ./scripts/audit_unsafe.sh
```

---

## Cargo.toml Release Configuration

```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"

[dependencies]
tokio = { version = ">=1.40", features = ["full"] }
serde = { version = ">=1.0", features = ["derive"] }

[dev-dependencies]
tokio = { version = "1.40", features = ["full", "test-util"] }

[profile.release]
lto = true           # Link-time optimization
codegen-units = 1     # Better optimization
panic = "abort"       # Smaller binary (no unwinding)
strip = true         # Remove debug symbols
opt-level = 3        # Maximum optimization

[profile.dev]
opt-level = 0         # Fast debug builds

[profile.bench]
opt-level = 3
debug = true          # Line-level profiling
```

---

## cargo fmt Configuration

```toml
# rustfmt.toml
edition = "2024"
normalize_comments = true
wrap_comments = true
comment_width = 100
format_code_in_doc_comments = true
max_width = 100
tab_spaces = 4
```

---

## cargo clippy Configuration

```toml
# .clippy.toml
msrv = "1.85"
cognitive_complexity = 15
too_many_arguments_threshold = 8
type_complexity_threshold = 500

[lint]
unsafe = "deny"  # Require SAFETY comments on unsafe blocks
```

---

## Version Bumping

### Semantic Versioning

```
major.minor.patch
  0.1.0 → 0.2.0 (minor, new features, backward compatible)
  0.2.0 → 1.0.0 (major, breaking changes)
  1.0.0 → 1.0.1 (patch, bug fixes)
```

### Bumping in Workspace

```bash
# Install cargo-workspaces
cargo install cargo-workspaces

# Bump all workspace crates
cargo workspaces version bump --all

# Bump specific crate
cargo workspaces version bump -p my-core --all
```

### Release Process

```bash
# 1. Update CHANGELOG.md
git log --oneline v0.1.0..HEAD

# 2. Update version in Cargo.toml (or use cargo workspaces)

# 3. Tag the release
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# 4. Publish to crates.io
cargo publish --dry-run  # Verify first
cargo publish

# 5. Create GitHub release
gh release create v0.2.0 --title "v0.2.0" -- notes "Release notes"
```

---

## Pre-Release Validation

```bash
# Full validation before release
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --all-features --no-deps

# Security checks
cargo audit
cargo deny check advisories licenses ban

# Build release artifact
cargo build --release

# Dry-run publish
cargo publish --dry-run

# Verify binary
./target/release/my-crate --version
```

---

## MSRV Enforcement in CI

```yaml
msrv:
  name: MSRV Check
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@1.85
      with:
        components: rustfmt, clippy
    - name: Check MSRV
      run: |
        cargo check --workspace --all-features
        cargo build --workspace --all-features
        cargo test --workspace --all-features
```

**Note:** Use `dtolnay/rust-toolchain` with a specific version (e.g., `@1.85`) to pin the MSRV toolchain. Do not use `@stable` for MSRV checks.

---

## Checklist

```
[ ] MSRV declared in Cargo.toml [package] rust-version
[ ] rust-toolchain.toml present in repo root
[ ] CI includes fmt, clippy, test jobs
[ ] CI includes doc build for libraries
[ ] CI includes MSRV check with specific toolchain version
[ ] CI includes cargo audit and cargo deny
[ ] Release profile uses LTO, codegen-units=1, panic=abort
[ ] Version bumped correctly (semver)
[ ] CHANGELOG.md updated
[ ] cargo publish --dry-run succeeds
[ ] Git tag created and pushed
[ ] GitHub release created
[ ] Binary size is reasonable
```