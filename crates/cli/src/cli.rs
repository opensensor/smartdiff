//! CLI argument parsing and configuration

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "smart-diff")]
#[command(about = "A smart code diffing tool that understands code structure")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compare files or directories
    Compare {
        /// First file or directory to compare
        #[arg(value_name = "FILE1")]
        file1: PathBuf,

        /// Second file or directory to compare
        #[arg(value_name = "FILE2")]
        file2: PathBuf,

        /// Output format
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,

        /// Compare directories recursively
        #[arg(short, long)]
        recursive: bool,

        /// Ignore whitespace changes
        #[arg(long)]
        ignore_whitespace: bool,

        /// Minimum similarity threshold (0.0-1.0)
        #[arg(long, default_value = "0.7")]
        threshold: f64,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    Html,
    Xml,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,

    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },

    /// Reset configuration to defaults
    Reset,
}
