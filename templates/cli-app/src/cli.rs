use clap::Parser;

/// A production CLI tool built with Rust.
#[derive(Debug, Parser)]
#[command(name = "{{crate_name}}")]
#[command(version = "0.1.0")]
#[command(about = "A production CLI tool", long_about = None)]
pub struct Cli {
    /// Increase verbosity (-vv for very verbose)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Validate the development environment.
    Doctor,
    /// Echo the input text back.
    Echo {
        /// The text to echo.
        #[arg(required = true)]
        text: String,

        /// Convert output to uppercase.
        #[arg(long, default_value = "false")]
        uppercase: bool,
    },
}
