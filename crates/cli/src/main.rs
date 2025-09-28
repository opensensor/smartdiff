//! Smart Code Diff CLI
//! 
//! Command-line interface for the smart code diffing tool.

use anyhow::Result;
use clap::Parser;
use tracing_subscriber;

mod cli;
mod commands;
mod output;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Compare { .. } => {
            commands::compare::run(cli).await
        }
        Commands::Config { .. } => {
            commands::config::run(cli).await
        }
    }
}
