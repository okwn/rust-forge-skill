# {{crate_name}}

WebAssembly module for {{description}}.

## Building

```bash
# Install wasm-pack if needed
cargo install wasm-pack

# Build for web target
wasm-pack build --target web --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg
```

## Usage (JavaScript)

```javascript
import init, { greet, add, create_point } from './pkg/{{crate_name}}.js';

await init();

console.log(greet("WASM"));
console.log("2 + 3 =", add(2, 3));

const p = create_point(3.0, 4.0);
console.log("Point:", p.x, p.y);
```

## Quality Gates

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
wasm-pack build --target web --dev  # Test build
wasm-pack build --target web       # Release build
```

## Browser Testing

```bash
# Requires wasm-pack
wasm-pack test --firefox
wasm-pack test --chrome
```

## MSRV

This crate requires Rust 1.85 or later.

## License

MIT OR Apache-2.0