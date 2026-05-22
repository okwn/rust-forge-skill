# Agent Notes

## Template Structure

This template follows Rust library best practices:

### Modules
- `lib.rs` - Library root, re-exports public API
- `error.rs` - Error types using thiserror
- `model.rs` - Data model with optional serde
- `service.rs` - Business logic service

### Edition and MSRV
- Edition: 2024
- MSRV: 1.85

### Feature Flags
- `serde` - Optional serialization support

## Validation

Run these commands to validate the template:

```bash
cargo check --all-features
cargo test --all-features
cargo doc --all-features --no-deps
cargo fmt -- --check
cargo clippy -- -D warnings
```

## Placeholder Substitution

When creating a new crate from this template:
- `{{crate_name}}` → actual crate name
- `{{description}}` → crate description