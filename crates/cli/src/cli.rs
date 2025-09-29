//! CLI argument parsing and configuration

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "smart-diff")]
#[command(about = "A smart code diffing tool that understands code structure and semantics")]
#[command(version)]
#[command(long_about = "Smart Code Diff is a next-generation code comparison tool that performs \
structural and semantic analysis of source code files. Unlike traditional line-based diff tools, \
it understands code structure, detects refactoring patterns, and provides intelligent matching \
of functions and classes across different versions of your codebase.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output with detailed logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable debug output with extensive logging
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Enable quiet mode (minimal output)
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Compare files or directories with structural analysis
    Compare {
        /// First file or directory to compare
        #[arg(value_name = "SOURCE")]
        source: PathBuf,

        /// Second file or directory to compare
        #[arg(value_name = "TARGET")]
        target: PathBuf,

        /// Output format for comparison results
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,

        /// Compare directories recursively
        #[arg(short, long)]
        recursive: bool,

        /// Ignore whitespace changes in comparison
        #[arg(long)]
        ignore_whitespace: bool,

        /// Ignore case differences in comparison
        #[arg(long)]
        ignore_case: bool,

        /// Minimum similarity threshold for function matching (0.0-1.0)
        #[arg(long, default_value = "0.7")]
        threshold: f64,

        /// Output file path (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Force language detection (override auto-detection)
        #[arg(short, long)]
        language: Option<Language>,

        /// Enable refactoring pattern detection
        #[arg(long)]
        detect_refactoring: bool,

        /// Enable cross-file function tracking
        #[arg(long)]
        track_moves: bool,

        /// Show function-level similarity scores
        #[arg(long)]
        show_similarity: bool,

        /// Include AST structure in output
        #[arg(long)]
        include_ast: bool,

        /// Maximum depth for AST comparison
        #[arg(long, default_value = "10")]
        max_depth: usize,

        /// Show performance statistics
        #[arg(long)]
        show_stats: bool,

        /// File patterns to include (glob patterns)
        #[arg(long, value_delimiter = ',')]
        include: Vec<String>,

        /// File patterns to exclude (glob patterns)
        #[arg(long, value_delimiter = ',')]
        exclude: Vec<String>,
    },

    /// Analyze a single file or directory for code metrics
    Analyze {
        /// File or directory to analyze
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Output format for analysis results
        #[arg(short, long, default_value = "text")]
        format: OutputFormat,

        /// Analyze directories recursively
        #[arg(short, long)]
        recursive: bool,

        /// Force language detection
        #[arg(short, long)]
        language: Option<Language>,

        /// Include complexity metrics
        #[arg(long)]
        complexity: bool,

        /// Include dependency analysis
        #[arg(long)]
        dependencies: bool,

        /// Include function signatures
        #[arg(long)]
        signatures: bool,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Validate configuration and test setup
    Doctor {
        /// Check specific component
        #[arg(long)]
        component: Option<String>,

        /// Fix issues automatically where possible
        #[arg(long)]
        fix: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Human-readable text output with colors
    Text,
    /// JSON format for programmatic consumption
    Json,
    /// HTML format with syntax highlighting
    Html,
    /// XML format for structured data
    Xml,
    /// Compact JSON format (single line)
    JsonCompact,
    /// CSV format for tabular data
    Csv,
    /// Markdown format for documentation
    Markdown,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Language {
    /// Java programming language
    Java,
    /// Python programming language
    Python,
    /// JavaScript programming language
    JavaScript,
    /// C++ programming language
    Cpp,
    /// C programming language
    C,
    /// Auto-detect language from file extension and content
    Auto,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigAction {
    /// Show current configuration
    Show {
        /// Show specific configuration section
        #[arg(long)]
        section: Option<String>,
    },

    /// Set configuration value
    Set {
        /// Configuration key (dot-separated path)
        key: String,
        /// Configuration value
        value: String,
    },

    /// Get configuration value
    Get {
        /// Configuration key (dot-separated path)
        key: String,
    },

    /// Reset configuration to defaults
    Reset {
        /// Reset specific section only
        #[arg(long)]
        section: Option<String>,
    },

    /// List all available configuration keys
    List,

    /// Validate current configuration
    Validate,
}

impl OutputFormat {
    /// Check if format supports colored output
    #[allow(dead_code)]
    pub fn supports_color(&self) -> bool {
        matches!(self, OutputFormat::Text | OutputFormat::Html | OutputFormat::Markdown)
    }

    /// Get file extension for format
    #[allow(dead_code)]
    pub fn file_extension(&self) -> &'static str {
        match self {
            OutputFormat::Text => "txt",
            OutputFormat::Json | OutputFormat::JsonCompact => "json",
            OutputFormat::Html => "html",
            OutputFormat::Xml => "xml",
            OutputFormat::Csv => "csv",
            OutputFormat::Markdown => "md",
        }
    }

    /// Get MIME type for format
    #[allow(dead_code)]
    pub fn mime_type(&self) -> &'static str {
        match self {
            OutputFormat::Text => "text/plain",
            OutputFormat::Json | OutputFormat::JsonCompact => "application/json",
            OutputFormat::Html => "text/html",
            OutputFormat::Xml => "application/xml",
            OutputFormat::Csv => "text/csv",
            OutputFormat::Markdown => "text/markdown",
        }
    }
}

impl Language {
    /// Convert to parser language enum
    pub fn to_parser_language(&self) -> Option<smart_diff_parser::Language> {
        match self {
            Language::Java => Some(smart_diff_parser::Language::Java),
            Language::Python => Some(smart_diff_parser::Language::Python),
            Language::JavaScript => Some(smart_diff_parser::Language::JavaScript),
            Language::Cpp => Some(smart_diff_parser::Language::Cpp),
            Language::C => Some(smart_diff_parser::Language::C),
            Language::Auto => None,
        }
    }

    /// Get file extensions for language
    #[allow(dead_code)]
    pub fn file_extensions(&self) -> Vec<&'static str> {
        match self {
            Language::Java => vec!["java"],
            Language::Python => vec!["py", "pyx", "pyi"],
            Language::JavaScript => vec!["js", "jsx", "mjs", "cjs"],
            Language::Cpp => vec!["cpp", "cxx", "cc", "hpp", "hxx", "h"],
            Language::C => vec!["c", "h"],
            Language::Auto => vec![],
        }
    }
}
