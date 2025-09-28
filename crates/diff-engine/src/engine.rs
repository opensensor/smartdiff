//! Main diff engine

use crate::matching::FunctionMatcher;
use crate::tree_edit::{TreeEditDistance, EditCost};
use crate::changes::ChangeClassifier;
use crate::refactoring::RefactoringDetector;
use smart_diff_parser::{Function, MatchResult};
use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Main diff engine that orchestrates the comparison process
pub struct DiffEngine {
    function_matcher: FunctionMatcher,
    tree_edit_distance: TreeEditDistance,
    change_classifier: ChangeClassifier,
    refactoring_detector: RefactoringDetector,
}

/// Result of diff computation
#[derive(Debug, Serialize, Deserialize)]
pub struct DiffResult {
    pub match_result: MatchResult,
    pub refactoring_patterns: Vec<crate::refactoring::RefactoringPattern>,
    pub execution_time_ms: u64,
    pub statistics: DiffStatistics,
}

/// Statistics about the diff computation
#[derive(Debug, Serialize, Deserialize)]
pub struct DiffStatistics {
    pub functions_compared: usize,
    pub functions_matched: usize,
    pub functions_added: usize,
    pub functions_removed: usize,
    pub functions_modified: usize,
    pub average_similarity: f64,
}

/// Diff engine errors
#[derive(Error, Debug)]
pub enum DiffError {
    #[error("Comparison failed: {0}")]
    ComparisonFailed(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
}

impl DiffEngine {
    pub fn new() -> Self {
        Self {
            function_matcher: FunctionMatcher::new(0.7),
            tree_edit_distance: TreeEditDistance::new(EditCost::default()),
            change_classifier: ChangeClassifier,
            refactoring_detector: RefactoringDetector::new(),
        }
    }
    
    /// Compare two sets of functions
    pub fn compare_functions(&self, source_functions: &[Function], target_functions: &[Function]) -> Result<DiffResult, DiffError> {
        let start_time = std::time::Instant::now();
        
        // Match functions
        let match_result = self.function_matcher.match_functions(source_functions, target_functions);
        
        // Detect refactoring patterns
        let refactoring_patterns = self.refactoring_detector.detect_patterns(&match_result.changes);
        
        // Calculate statistics
        let statistics = self.calculate_statistics(source_functions, target_functions, &match_result);
        
        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(DiffResult {
            match_result,
            refactoring_patterns,
            execution_time_ms,
            statistics,
        })
    }
    
    fn calculate_statistics(&self, source: &[Function], target: &[Function], match_result: &MatchResult) -> DiffStatistics {
        let functions_compared = source.len() + target.len();
        let functions_matched = match_result.mapping.len();
        let functions_added = match_result.unmatched_target.len();
        let functions_removed = match_result.unmatched_source.len();
        let functions_modified = match_result.changes.iter()
            .filter(|c| matches!(c.change_type, smart_diff_parser::ChangeType::Modify))
            .count();
        
        DiffStatistics {
            functions_compared,
            functions_matched,
            functions_added,
            functions_removed,
            functions_modified,
            average_similarity: match_result.similarity,
        }
    }
}
