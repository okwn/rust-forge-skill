# {{project_name}}

A production CLI tool built with Rust, using clap derive for argument parsing and tracing for logging.

## Quick Start

```bash
cargo build --release
cargo run -- --help
```

## Commands

### doctor

Validates the development environment, showing Rust and Cargo versions.

```bash
{{crate_name}} doctor
```

### echo

Echoes text back, optionally in uppercase.

```bash
{{crate_name}} echo "Hello, world!"
{{crate_name}} echo "hello" --uppercase
```

## Environment Variables

| Variable | Description | Default |
|---|---|---|
| `LOG_LEVEL` | Log level (trace, debug, info, warn, error) | `info` |
| `TARGET_DIR` | Target directory for build artifacts | `target` |

## Adding a New Command

1. Create a new file in `src/commands/` (e.g., `src/commands/hello.rs`):

```rust
use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    println!("Hello, {}!", name);
    Ok(())
}
```

2. Add the module to `src/commands/mod.rs`:

```rust
pub mod hello;
```

3. Add the command variant to `src/cli.rs` `Commands` enum:

```rust
#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    Doctor,
    Echo {
        text: String,
        uppercase: bool,
    },
    /// New command
    Hello {
        /// Name to greet
        name: String,
    },
}
```

4. Handle the command in `src/main.rs` `run()`:

```rust
Commands::Hello { name } => commands::hello::run(&name),
```

## Quality Gates

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Architecture

```
src/
├── main.rs          # Entry point, CLI parsing, tracing init
├── cli.rs           # Cli struct, Commands enum, argument parsing
├── config.rs        # Config struct, environment variable loading
└── commands/
    ├── mod.rs       # Command modules
    ├── doctor.rs    # Environment validation command
    └── echo.rs      # Echo input command
```

## License

MIT OR Apache-2.0