# Agent Notes: Extending the CLI Template

This document describes how to add new commands and modify this template.

## Template Structure

```
cli-app/
├── Cargo.toml           # Package manifest
├── rust-toolchain.toml  # Rust channel (stable)
├── rustfmt.toml         # Formatting config
├── .gitignore           # Standard Rust gitignore
├── README.md            # User-facing documentation
├── AGENT_NOTES.md       # This file (for agents)
├── src/
│   ├── main.rs          # Entry point, run() function
│   ├── cli.rs           # Cli struct, Commands enum
│   ├── config.rs        # Config struct (env vars)
│   └── commands/
│       ├── mod.rs       # Module declarations
│       ├── doctor.rs    # Example: env validation
│       └── echo.rs      # Example: text echoing
└── tests/
    └── cli_smoke.rs     # Smoke tests
```

## Placeholder Tokens

The following tokens are replaced by `cargo generate-template`:

- `{{crate_name}}` - Package name (kebab-case)
- `{{project_name}}` - Human-readable project name
- `{{author}}` - Author name
- `{{description}}` - Package description

## Adding a New Command

### Step 1: Create the command module

Create `src/commands/<name>.rs`:

```rust
use anyhow::Result;

pub fn run(arg1: String, arg2: bool) -> Result<()> {
    // Implementation
    Ok(())
}
```

### Step 2: Register the module

Edit `src/commands/mod.rs`:

```rust
pub mod hello;  // Add this line
```

### Step 3: Add command variant

Edit `src/cli.rs`, add to `Commands` enum:

```rust
/// Description shown in --help
Hello {
    /// Help text for arg1
    #[arg(long)]
    arg1: String,

    #[arg(long, default_value = "false")]
    arg2: bool,
},
```

### Step 4: Handle the command

Edit `src/main.rs` `run()` function:

```rust
cli::Commands::Hello { arg1, arg2 } => commands::hello::run(arg1, arg2),
```

## Configuration Pattern

Add fields to `Config` struct in `src/config.rs`:

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_value")]
    pub field_name: String,
    // ...existing fields...
}
```

Then parse in `from_env()`:

```rust
pub fn from_env() -> Self {
    Self {
        field_name: std::env::var("FIELD_NAME").unwrap_or_else(|_| default_value()),
        // ...existing fields...
    }
}
```

## Error Handling

- Use `anyhow::Result<()>` for commands that may fail
- Return `Ok(())` on success
- Use `context()` to add context to errors: `some_call().context("failed to do something")?`
- No `thiserror` needed for simple CLI tools

## Testing Pattern

In `tests/cli_smoke.rs`:

```rust
#[test]
fn test_command_help() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "help"])
        .output()
        .expect("failed to run command");
    assert!(output.status.success());
}
```

## Key Dependencies

- `clap` (derive): CLI argument parsing
- `tracing-subscriber`: Structured logging
- `anyhow`: Error handling at app boundary
- `serde`: Configuration from env vars

## Rust Edition Notes

- edition = "2024" requires Rust 1.85+
- Use modern Rust idioms (let chaining, etc.)
- `rustfmt.toml` enforces 2024 style