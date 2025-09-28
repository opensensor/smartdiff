//! Smart Code Diff Engine
//! 
//! Core diff computation engine that implements tree edit distance algorithms,
//! function matching, and change classification.

pub mod matching;
pub mod tree_edit;
pub mod changes;
pub mod refactoring;
pub mod engine;

pub use matching::{FunctionMatcher, MatchResult, SimilarityScore};
pub use tree_edit::{TreeEditDistance, EditOperation, EditCost};
pub use changes::{Change, ChangeType, ChangeClassifier};
pub use refactoring::{RefactoringDetector, RefactoringPattern};
pub use engine::{DiffEngine, DiffResult, DiffError};

/// Re-export commonly used types
pub type Result<T> = std::result::Result<T, DiffError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Basic smoke test to ensure the crate compiles
        assert!(true);
    }
}
