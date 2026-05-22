# Library Template

A production-ready Rust library template with proper error handling, async support, and feature flags.

## Semver Policy

This crate follows semantic versioning:
- **PATCH** (0.1.x): Bug fixes, no API changes
- **MINOR** (0.x.0): New features, backward compatible
- **MAJOR** (x.0.0): Breaking changes

## Public API

The public API consists of:
- `LibraryError` enum - all library errors
- `Model` struct - the main data type
- `Service` struct - main service for business logic

All public items are documented and include usage examples.

## Error Design

This library uses `thiserror` for error handling:
- `LibraryError` is the main error enum
- Errors are non-exhaustive to allow future extension
- Each error variant has a clear, descriptive message
- No `anyhow` in library code - all errors are explicit

## Feature Flags

- `serde` - Enable serialization support (optional)

## Usage

```toml
[dependencies]
{{crate_name}} = "0.1.0"
```

```rust
use {{crate_name}}::{Service, Model};

let model = Model::new("example".into());
let service = Service::new();
let result = service.process(model);
```

## MSRV

Rust 1.85 or later is required.

## License

MIT OR Apache-2.0