# 05 — CLI: Clap + Tracing

**Purpose:** Guide agents to build production-grade CLI applications with clap derive, structured logging via tracing, proper error handling, and user-friendly output formatting.

---

## When to Use Clap Derive

**Use clap derive when:**
- Building a CLI with subcommands, flags, and env var support
- The tool will be used by developers or in CI/CD pipelines
- You need `--help` and `--version` to work out of the box
- Shell completion generation is desired

**Do not use clap derive when:**
- The tool is a simple one-liner (use `std::env::args()` instead)
- The tool is a script with no user interaction

---

## Basic CLI Structure

```rust
// Cargo.toml
[dependencies]
clap = { version = "4.5", features = ["derive", "env"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
anyhow = "1.0"
tokio = { version = "1.40", features = ["full"] }
```

```rust
// src/main.rs
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "myapp",
    version = "1.0",
    about = "A production CLI tool",
    author = "Author <author@example.com>"
)]
struct Cli {
    /// Enable verbose logging (-vv for very verbose)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Configuration file path
    #[arg(short, long, env = "MYAPP_CONFIG")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
    },
    /// Process input data
    Process {
        /// Input file (default: stdin)
        #[arg(default_value = "-")]
        input: PathBuf,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show current configuration
    Config {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
```

---

## Clap Env Var Support

**One of clap's most powerful features is automatic env var support.**

```rust
// These generate automatic env var support:
// --config maps to MYAPP_CONFIG
// --port maps to MYAPP_PORT

#[derive(Parser)]
struct Cli {
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    log_level: String,

    #[arg(long, env = "MYAPP_API_KEY")]
    api_key: Option<String>,
}

// Usage:
// DATABASE_URL=postgres://... myapp serve --port 9000
// MYAPP_LOG_LEVEL=debug myapp process input.txt
```

---

## Tracing Initialization

```rust
use tracing::{info, error, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, fmt::format::FmtSpan};
use std::path::PathBuf;

fn init_tracing(verbose: u8, log_format: &str) {
    let level = match verbose {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let env_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "myapp=debug".into()),
    );

    match log_format {
        "json" => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_span_events(FmtSpan::CLOSE)
                        .with_target(true)
                        .with_thread_ids(true)
                        .json()
                )
                .init();
        }
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_span_events(FmtSpan::CLOSE)
                        .with_target(true)
                        .with_thread_ids(true)
                        .with_file(true)
                        .with_line_number(true)
                )
                .init();
        }
    }
}
```

---

## Main Entry Point

```rust
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize logging first
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".into());
    init_tracing(cli.verbose, &log_format);

    info!(version = env!("CARGO_PKG_VERSION"), "starting application");

    // Run the application
    if let Err(e) = run(cli).await {
        error!(error = %e, "application error");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Serve { port, host } => {
            info!(port = port, host = %host, "starting server");
            serve(port, host).await?;
        }
        Commands::Process { input, output, verbose } => {
            info!(input = %input, output = ?output, "processing");
            process(input, output, verbose).await?;
        }
        Commands::Config { json } => {
            show_config(json)?;
        }
    }
    Ok(())
}
```

---

## Exit Codes

**Use exit codes correctly for Unix convention:**

```rust
use std::process::{exit, ExitCode};

fn main() -> ExitCode {
    match run_inner() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {}", e);
            match e.downcast_ref::<ExitCode>() {
                Some(code) => *code,
                None => ExitCode::FAILURE,
            }
        }
    }
}

struct ExitCode(u8);

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self { ExitCode(code) }
}
```

**Standard exit codes:**
- `0` — success
- `1` — general error
- `2` — misuse of command
- `130` — SIGINT (Ctrl+C)

---

## User-Facing Errors

**Error messages to users should be:**
- Clear and actionable
- Not expose internal implementation details
- Suggest how to fix the problem

```rust
use anyhow::{Context, Result};

fn load_config(path: &Path) -> Result<Config> {
    std::fs::read_to_string(path)
        .with_context(|| format!(
            "failed to read config file '{}'. \
            Use --config to specify a different path or set MYAPP_CONFIG env var.",
            path.display()
        ))?
        .parse()
        .context("config file is invalid YAML")?;

    Ok(config)
}
```

---

## Machine-Readable Output

**For CI/CD integration, support JSON output:**

```rust
use serde_json::json;

enum OutputFormat {
    Pretty,
    Json,
}

fn show_config(json_format: bool) -> anyhow::Result<()> {
    if json_format {
        // Machine-readable
        let config = get_config();
        println!("{}", serde_json::to_string_pretty(&config)?);
    } else {
        // Human-readable
        let config = get_config();
        println!("Database: {}", config.database_url);
        println!("Port: {}", config.port);
    }
    Ok(())
}
```

---

## Subcommand Organization

For complex CLIs, organize each subcommand as a module:

```
src/
├── main.rs              # CLI parsing, tracing init, run()
├── commands/
│   ├── mod.rs
│   ├── serve.rs        # serve subcommand
│   ├── process.rs      # process subcommand
│   └── config.rs       # config subcommand
├── config.rs            # Config loading
└── error.rs             # Error types
```

```rust
// src/commands/mod.rs
pub mod serve;
pub mod process;
pub mod config;

use anyhow::Result;

pub async fn run(cli: Commands) -> Result<()> {
    match cli {
        Commands::Serve { port, host } => serve::run(port, host).await,
        Commands::Process { input, output, verbose } => process::run(input, output, verbose).await,
        Commands::Config { json } => config::show(json),
    }
}
```

---

## Testing CLI

```rust
#[cfg(test)]
mod tests {
    use clap::Command;

    #[test]
    fn test_help_output() {
        let cmd = Cli::command();
        let result = cmd.clone()
            .try_get_matches_from(["myapp", "--help"])
            .unwrap_err();
        let output = result.to_string();
        assert!(output.contains("Usage:"));
        assert!(output.contains("serve"));
        assert!(output.contains("process"));
    }

    #[test]
    fn test_default_port() {
        let cli = Cli::try_parse_from(["myapp", "serve"]).unwrap();
        match cli.command {
            Commands::Serve { port, .. } => assert_eq!(port, 8080),
            _ => panic!("expected serve command"),
        }
    }

    #[test]
    fn test_env_var_override() {
        std::env::set_var("MYAPP_PORT", "9000");
        let cli = Cli::try_parse_from(["myapp", "serve"]).unwrap();
        std::env::remove_var("MYAPP_PORT");
        match cli.command {
            Commands::Serve { port, .. } => assert_eq!(port, 9000),
            _ => panic!("expected serve command"),
        }
    }
}
```

---

## Progress Indicators

```rust
use std::time::Duration;

// For long-running operations
fn run_with_progress(items: &[Item]) -> anyhow::Result<()> {
    let pb = indicatif::ProgressBar::new(items.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40} {pos}/{len} {msg}")?
            .unwrap()
    );

    for item in items {
        pb.set_message(&item.name);
        process_item(item)?;
        pb.inc(1);
    }

    pb.finish_with_message("done");
    Ok(())
}
```

---

## Checklist

```
[ ] CLI uses clap derive with #[command] and #[arg] attributes
[ ] Env vars documented via #[arg(env = "...")]
[ ] Tracing initialized before any operation
[ ] --help and --version work correctly
[ ] Error messages go to stderr, not stdout
[ ] Successful output goes to stdout
[ ] Structured fields in tracing (not format! in logs)
[ ] No println! / eprintln! in production code
[ ] Subcommands organized in separate modules
[ ] Verbose flag controls log level (-v, -vv)
[ ] Exit codes follow Unix conventions
[ ] JSON output option for CI integration
[ ] Tests cover argument parsing, env var override, help output
```