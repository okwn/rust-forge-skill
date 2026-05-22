# 09 — WASM Module Patterns

**Purpose:** Guide agents to build WebAssembly modules targeting browser and edge runtimes. WASM has a restricted standard library, no OS access, and a sandboxed execution model. Design accordingly.

---

## When to Use WASM

**Use WASM when:**
- Building browser-based performance-critical code
- Targeting edge runtimes (Cloudflare Workers, wasmtime, Fastly Compute)
- Sharing code between Rust and JavaScript/TypeScript
- Running untrusted code in a sandboxed environment

**Do not use WASM when:**
- Standard Rust is sufficient (native target is faster to compile and run)
- Heavy I/O dominates (WASM has limited I/O primitives)
- You need a rich ecosystem (most crates work in WASM, but some don't)

---

## Project Structure

```
wasm-module/
├── Cargo.toml
├── src/
│   ├── lib.rs              # WASM library root, wasm_bindgen exports
│   └── utils.rs            # Pure Rust utilities (testable without WASM)
├── pkg/                    # wasm-pack output (gitignored)
├── web/
│   └── index.html          # Test harness
├── tests/
│   └── browser_tests.rs    # wasm-bindgen tests
└── README.md
```

---

## Cargo.toml for WASM

```toml
[package]
name = "my-wasm-module"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

[lib]
crate-type = ["cdylib", "rlib"]  # cdylib for WASM, rlib for native testing

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"

[dev-dependencies]
wasm-bindgen-test = "0.3"
console_error_panic_hook = "0.1"

[profile.release]
opt-level = "s"  # Optimize for size
lto = true
panic = "abort"
strip = true
```

**Key points:**
- `crate-type = ["cdylib", "rlib"]` — cdylib for WASM, rlib for native tests
- `opt-level = "s"` — size optimization (vs "z" for most aggressive)
- `panic = "abort"` — smaller binary (no unwinding machinery)

---

## Basic WASM Library

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in console.
/// Call once at startup.
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Greet a name.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Add two integers.
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Process a byte slice, incrementing each byte by 1.
#[wasm_bindgen]
pub fn process_bytes(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b.wrapping_add(1)).collect()
}
```

---

## Exporting Complex Types

```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 { self.x }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 { self.y }

    /// Calculate distance to another point.
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[wasm_bindgen]
pub fn create_point(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

#[wasm_bindgen]
pub fn points_distance(p1: &Point, p2: &Point) -> f64 {
    p1.distance_to(p2)
}
```

---

## JavaScript Interop

```rust
use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Export with logging
#[wasm_bindgen]
pub fn process_with_log(input: &str) -> String {
    log(&format!("Processing: {}", input));
    input.to_uppercase()
}

// Import JavaScript function
#[wasm_bindgen]
pub fn set_timeout(callback: &js_sys::Function, ms: u32) -> i32 {
    web_sys::window()
        .and_then(|w| w.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref() as &js_sys::Function, ms as i32
        ))
        .unwrap_or(-1)
}
```

---

## Error Serialization

WASM can't throw exceptions across the JS boundary. Use `Result` and map to `JsValue`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_config(json: &str) -> Result<serde_json::Value, JsValue> {
    serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

// Or use Result<T, JsValue> pattern
#[wasm_bindgen]
pub fn risky_operation(input: &str) -> Result<String, JsValue> {
    if input.is_empty() {
        Err(JsValue::from_str("input cannot be empty"))
    } else {
        Ok(input.to_uppercase())
    }
}
```

---

## Building for WASM

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web (generates pkg/)
wasm-pack build --target web --out-dir pkg

# Build for Node.js
wasm-pack build --target nodejs --out-dir pkg

# Build for bundlers (webpack, rollup)
wasm-pack build --target bundler --out-dir pkg

# Build with debug info
wasm-pack build --dev

# Build for WASI (server-side WASM)
wasm-pack build --target wasm32-wasip1 --out-dir pkg
```

---

## Testing WASM

```rust
// tests/browser_tests.rs
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_greet() {
    let result = crate::greet("World");
    assert_eq!(result, "Hello, World!");
}

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(crate::add(2, 3), 5);
}

#[wasm_bindgen_test]
fn test_point_distance() {
    let p1 = crate::Point::new(0.0, 0.0);
    let p2 = crate::Point::new(3.0, 4.0);
    let dist = p1.distance_to(&p2);
    assert!((dist - 5.0).abs() < 1e-10);
}

#[wasm_bindgen_test]
fn test_error_propagation() {
    let result = crate::risky_operation("");
    assert!(result.is_err());
}
```

```bash
# Run tests
wasm-pack test --firefox
wasm-pack test --chrome
wasm-pack test --node
```

---

## Web Testing HTML

```html
<!-- web/index.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>WASM Module Test</title>
</head>
<body>
    <script type="module">
        import init, { greet, add, create_point } from '../pkg/my_wasm_module.js';

        await init();

        console.log(greet("WASM"));           // "Hello, WASM!"
        console.log("2 + 3 =", add(2, 3));    // 5

        const p = create_point(3.0, 4.0);
        console.log("Point:", p.x, p.y);      // 3, 4
    </script>
</body>
</html>
```

---

## WASM-Specific Constraints

### No std::fs

WASM cannot access the file system. Use Web APIs for browser I/O.

### No std::net

Networking is limited to Web APIs (fetch, WebSocket).

### Memory Model

```rust
// WASM has linear memory. Rust sees it as a Vec<u8>.
// Passing large data: use Vec<u8> or &/[u8], not pointers.

#[wasm_bindgen]
pub fn process_large_data(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| transform(b)).collect()
}
```

### Deterministic Builds

WASM must be deterministic. Avoid:
- `std::time::Instant` for timing-dependent logic
- Random number generation without seed
- Non-deterministic compilation (use `--locked` for cargo)

---

## Optimizing WASM Binary Size

```toml
# Cargo.toml
[profile.release]
opt-level = "s"      # Optimize for size, not speed
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization at cost of compile time
panic = "abort"       # No unwinding
strip = true          # Remove debug info
```

```bash
# Use wasm-opt (binaryen) after building
wasm-opt -Oz -o output.wasm input.wasm

# Check size
ls -lh pkg/*.wasm
```

---

## Checklist

```
[ ] Cargo.toml has both cdylib and rlib crate types
[ ] wasm-bindgen used for all JS interop
[ ] Panic hook initialized for better error messages
[ ] All exported functions have #[wasm_bindgen]
[ ] Complex types derive Serialize/Deserialize
[ ] wasm-pack build succeeds
[ ] wasm-pack test passes (browser or node)
[ ] Release build uses -O "s" and LTO for small binaries
[ ] No std::fs or std::net (use Web APIs instead)
[ ] Memory ownership is clear (no use-after-free across boundary)
[ ] README documents usage from JavaScript
[ ] Pure Rust utilities are in separate module (testable without WASM)
```