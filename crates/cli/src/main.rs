//! Smart Code Diff CLI
//!
//! Command-line interface for the smart code diffing tool that provides
//! structural and semantic code comparison with advanced analysis capabilities.

use anyhow::Result;
use clap::Parser;
use colored::*;
use tracing_subscriber::{self, EnvFilter};

mod cli;
mod commands;
mod output;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing with appropriate level
    let log_level = if cli.debug {
        "debug"
    } else if cli.verbose {
        "info"
    } else if cli.quiet {
        "error"
    } else {
        "warn"
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(format!("smart_diff={}", log_level)))
        .with_target(false)
        .with_level(false)
        .init();

    // Disable colors if requested
    if cli.no_color {
        colored::control::set_override(false);
    }

    // Handle global configuration
    if let Some(config_path) = &cli.config {
        // TODO: Load configuration from file
        tracing::info!("Using configuration file: {}", config_path.display());
    }

    // Route to appropriate command handler
    let result = match cli.command {
        Commands::Compare { .. } => commands::compare::run(cli.clone()).await,
        Commands::Analyze { .. } => commands::analyze::run(cli.clone()).await,
        Commands::Config { .. } => commands::config::run(cli.clone()).await,
        Commands::Doctor { .. } => commands::doctor::run(cli.clone()).await,
    };

    // Handle errors with appropriate formatting
    if let Err(ref error) = result {
        if cli.debug {
            // Show full error chain in debug mode
            eprintln!("{} {:?}", "Error:".red().bold(), error);
        } else {
            // Show user-friendly error message
            eprintln!("{} {}", "Error:".red().bold(), error);

            // Show suggestion for more details
            if !cli.verbose {
                eprintln!(
                    "{} Run with {} for more details",
                    "Hint:".yellow().bold(),
                    "--verbose".cyan()
                );
            }
        }

        std::process::exit(1);
    }

    result
}
