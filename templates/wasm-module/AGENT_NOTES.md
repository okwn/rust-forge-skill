# Agent Notes — WASM Module Template

## Template Purpose
Scaffolds a Rust WebAssembly module using `wasm-bindgen` and `web-sys`. Use for browser or edge runtime (Cloudflare Workers, etc.).

## Instantiation Notes

### Required Customisations
1. **`Cargo.toml`** — Update `crate-name` and verify `wasm-bindgen` features match your API
2. **`src/lib.rs`** — Replace placeholder functions with your actual WASM API
3. **`web/index.html`** — Update to match your WASM module's exported functions

### WASM Build Commands
```bash
wasm-pack build --target web --out-dir pkg
wasm-pack build --target no-modules --out-dir pkg  # alternative
```

### Module Structure
```
src/
├── lib.rs    — WASM-exposed functions (#[wasm_bindgen])
└── utils.rs  — Internal helpers (not WASM-exposed)
web/
└── index.html — Browser test harness
```

### Validation
```bash
wasm-pack build --target web --out-dir pkg --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-features
```

### Browser Testing
- Requires a local HTTP server (e.g. `python3 -m http.server 8080`)
- Open `web/index.html` in a browser
- WASM must be served over HTTP (not `file://`) for `WebAssembly.instantiate` to work

### Critical Rules
- No `unsafe` blocks without SAFETY comments
- No `static mut` — WASM has no threading
- All exported functions must be `pub extern "C"` or `#[wasm_bindgen]`
- Memory allocation in WASM is explicit — no `Box::new()` in hot paths
