//! Smart Code Diff Engine
//!
//! Core diff computation engine that implements tree edit distance algorithms,
//! function matching, and change classification.

pub mod changes;
pub mod engine;
pub mod hungarian_matcher;
pub mod matching;
pub mod refactoring;
pub mod similarity_scorer;
pub mod tree_edit;

pub use changes::ChangeClassifier;
pub use engine::{DiffEngine, DiffError, DiffResult};
pub use hungarian_matcher::{
    FunctionAssignment, HungarianMatchResult, HungarianMatcher, HungarianMatcherConfig,
    ManyToManyMapping, MappingType, MatchingStatistics,
};
pub use matching::{FunctionMatcher, SimilarityScore};
pub use refactoring::{RefactoringDetector, RefactoringPattern};
pub use similarity_scorer::{
    ASTSimilarityScore, ComprehensiveSimilarityScore, ContextSimilarityScore,
    DetailedSimilarityBreakdown, MatchType, SemanticSimilarityMetrics, SimilarityFactor,
    SimilarityScorer, SimilarityScoringConfig,
};
pub use smart_diff_parser::MatchResult;
pub use smart_diff_parser::{Change, ChangeType};
pub use tree_edit::{EditCost, EditOperation, TreeEditDistance};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, DiffError>;

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic_functionality() {
        // Basic smoke test to ensure the crate compiles
        // This test passes if the crate compiles successfully
    }
}
