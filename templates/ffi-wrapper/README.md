# {{crate_name}}

FFI bindings for {{description}}.

## Usage

```rust
use {{crate_name}}::{Vec3, Vec3Wrapper, FfiError};

let vec = Vec3Wrapper::new(3.0, 4.0, 0.0)?;
println!("Length: {}", vec.length());
```

## Building

```bash
# Generate bindings and build
cargo build --release

# Run tests
cargo test
```

## Quality Gates

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
./scripts/audit_unsafe.sh
```

## Safety

This crate wraps unsafe C FFI code. All unsafe blocks are documented with SAFETY comments explaining the invariants that must be maintained.

## MSRV

This crate requires Rust 1.85 or later.

## License

MIT OR Apache-2.0