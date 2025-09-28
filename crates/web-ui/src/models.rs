//! Data models for API requests and responses

use serde::{Deserialize, Serialize};

/// File information for comparison
#[derive(Debug, Deserialize, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub content: String,
}

/// Request to compare two files
#[derive(Debug, Deserialize)]
pub struct CompareRequest {
    pub file1: FileInfo,
    pub file2: FileInfo,
    
    /// Optional configuration
    #[serde(default)]
    pub options: CompareOptions,
}

/// Comparison options
#[derive(Debug, Deserialize, Default)]
pub struct CompareOptions {
    /// Minimum similarity threshold (0.0-1.0)
    #[serde(default = "default_threshold")]
    pub threshold: f64,
    
    /// Whether to ignore whitespace changes
    #[serde(default)]
    pub ignore_whitespace: bool,
    
    /// Whether to detect cross-file moves
    #[serde(default)]
    pub detect_moves: bool,
}

fn default_threshold() -> f64 {
    0.7
}

/// Response from file comparison
#[derive(Debug, Serialize)]
pub struct CompareResponse {
    /// Overall similarity score (0.0-1.0)
    pub similarity: f64,
    
    /// List of detected changes
    pub changes: Vec<String>,
    
    /// Functions that were added
    pub functions_added: Vec<String>,
    
    /// Functions that were removed
    pub functions_removed: Vec<String>,
    
    /// Functions that were modified
    pub functions_modified: Vec<String>,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}
