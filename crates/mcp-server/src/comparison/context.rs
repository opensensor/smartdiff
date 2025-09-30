//! Comparison context data structures

use serde::{Deserialize, Serialize};
use smart_diff_engine::DiffResult;
use smart_diff_parser::Function;
use uuid::Uuid;

/// Unique identifier for a comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComparisonId(Uuid);

impl ComparisonId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl Default for ComparisonId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ComparisonId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Parameters for a comparison operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonParams {
    pub source_path: String,
    pub target_path: String,
    #[serde(default)]
    pub recursive: bool,
    #[serde(default)]
    pub file_patterns: Vec<String>,
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
}

/// A single function change with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionChange {
    pub function_name: String,
    pub source_file: Option<String>,
    pub target_file: Option<String>,
    pub change_type: String,
    pub similarity_score: f64,
    pub change_magnitude: f64,
    pub source_signature: Option<String>,
    pub target_signature: Option<String>,
    pub source_content: Option<String>,
    pub target_content: Option<String>,
    pub source_start_line: Option<usize>,
    pub source_end_line: Option<usize>,
    pub target_start_line: Option<usize>,
    pub target_end_line: Option<usize>,
    pub diff_summary: Option<String>,
    /// True if this is a high-similarity move (>= 0.95) with no meaningful changes
    #[serde(default)]
    pub is_unchanged_move: bool,
}

impl FunctionChange {
    /// Calculate change magnitude (0.0 = no change, 1.0 = complete change)
    pub fn calculate_magnitude(&self) -> f64 {
        match self.change_type.as_str() {
            "added" => 1.0,
            "deleted" => 1.0,
            "modified" => 1.0 - self.similarity_score,
            "renamed" => 0.3, // Renamed but similar content
            "moved" => 0.2,   // Moved but same content
            _ => 0.0,
        }
    }
}

/// Complete comparison context
#[derive(Debug, Clone)]
pub struct ComparisonContext {
    pub id: ComparisonId,
    pub params: ComparisonParams,
    pub source_functions: Vec<Function>,
    pub target_functions: Vec<Function>,
    pub diff_result: Option<DiffResult>,
    pub function_changes: Vec<FunctionChange>,
    pub unchanged_moves: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ComparisonContext {
    pub fn new(params: ComparisonParams) -> Self {
        Self {
            id: ComparisonId::new(),
            params,
            source_functions: Vec::new(),
            target_functions: Vec::new(),
            diff_result: None,
            function_changes: Vec::new(),
            unchanged_moves: 0,
            created_at: chrono::Utc::now(),
        }
    }

    /// Get functions sorted by change magnitude (most changed first)
    pub fn get_sorted_changes(&self) -> Vec<FunctionChange> {
        let mut changes = self.function_changes.clone();
        changes.sort_by(|a, b| {
            b.change_magnitude
                .partial_cmp(&a.change_magnitude)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        changes
    }

    /// Get a specific function change by name
    pub fn get_function_change(&self, name: &str) -> Option<&FunctionChange> {
        self.function_changes
            .iter()
            .find(|c| c.function_name == name)
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> ComparisonSummary {
        let added = self
            .function_changes
            .iter()
            .filter(|c| c.change_type == "added")
            .count();
        let deleted = self
            .function_changes
            .iter()
            .filter(|c| c.change_type == "deleted")
            .count();
        let modified = self
            .function_changes
            .iter()
            .filter(|c| c.change_type == "modified")
            .count();
        let renamed = self
            .function_changes
            .iter()
            .filter(|c| c.change_type == "renamed")
            .count();
        let moved = self
            .function_changes
            .iter()
            .filter(|c| c.change_type == "moved")
            .count();

        // Total unique functions is the larger of source or target count
        // (since added functions are only in target, deleted only in source)
        let total_functions = self.source_functions.len().max(self.target_functions.len());

        // Unchanged = total - all changes
        let unchanged = total_functions.saturating_sub(added + deleted + modified + renamed + moved);

        ComparisonSummary {
            total_functions,
            added,
            deleted,
            modified,
            renamed,
            moved,
            unchanged,
            unchanged_moves: self.unchanged_moves,
        }
    }
}

/// Summary statistics for a comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonSummary {
    pub total_functions: usize,
    pub added: usize,
    pub deleted: usize,
    pub modified: usize,
    pub renamed: usize,
    pub moved: usize,
    pub unchanged: usize,
    /// Functions that moved between files without changes (similarity >= 0.95)
    /// These are filtered from the changes list to reduce noise
    #[serde(default)]
    pub unchanged_moves: usize,
}

