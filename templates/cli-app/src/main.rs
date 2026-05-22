use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod commands;
mod config;

fn init_tracing(verbose: u8) {
    let _level = match verbose {
        0 => tracing::Level::WARN,
        1 => tracing::Level::INFO,
        2 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    let env_filter = tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
    );

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    init_tracing(cli.verbose);

    info!(version = env!("CARGO_PKG_VERSION"), "starting application");

    let cfg = config::Config::from_env();

    match cli.command {
        cli::Commands::Doctor => commands::doctor::run(&cfg),
        cli::Commands::Echo { text, uppercase } => commands::echo::run(text, uppercase),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}