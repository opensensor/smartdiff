//! Smart Code Diff Engine
//!
//! Core diff computation engine that implements tree edit distance algorithms,
//! function matching, and change classification.

pub mod changes;
pub mod cross_file_tracker;
pub mod engine;
pub mod graph_matcher;
pub mod hungarian_matcher;
pub mod matching;
pub mod refactoring;
pub mod similarity_scorer;
pub mod smart_matcher;
pub mod tree_edit;

pub use changes::{
    ChangeClassifier, ChangeClassificationConfig, DetailedChangeClassification,
    ChangeAnalysis, ChangeCharacteristic, CharacteristicType, ClassificationEvidence,
    EvidenceType, AlternativeClassification, ChangeImpact, ImpactLevel, EffortLevel, RiskLevel
};
pub use graph_matcher::{
    GraphMatcher, GraphMatcherConfig, GraphMatchResult, FunctionMatch,
    FunctionMove as GraphFunctionMove, FunctionRename, DependencyChange,
    DependencyChangeType, MatchType as GraphMatchType
};
pub use engine::{DiffEngine, DiffError, DiffResult};
pub use hungarian_matcher::{
    FunctionAssignment, HungarianMatchResult, HungarianMatcher, HungarianMatcherConfig,
    ManyToManyMapping, MappingType, MatchingStatistics,
};
pub use cross_file_tracker::{
    CrossFileTracker, CrossFileTrackerConfig, CrossFileTrackingResult,
    FunctionMove, FunctionRenameMove, CrossFileSplit, CrossFileMerge,
    MoveType, FileTrackingStats, CrossFileTrackingStats
};
pub use matching::{FunctionMatcher, SimilarityScore};
pub use smart_matcher::{SmartMatcher, SmartMatcherConfig};
pub use refactoring::{
    RefactoringDetector, RefactoringDetectionConfig, RefactoringPattern,
    RefactoringAnalysis, RefactoringCharacteristic, RefactoringCharacteristicType,
    RefactoringEvidence, RefactoringEvidenceType, BeforeAfterComparison, SizeComparison,
    RefactoringImpact, RefactoringImpactLevel, ApiCompatibilityImpact,
    RefactoringQualityMetrics, RefactoringComplexity, RefactoringComplexityLevel, RefactoringEffort
};
pub use similarity_scorer::{
    ASTSimilarityScore, ComprehensiveSimilarityScore, ContextSimilarityScore,
    DetailedSimilarityBreakdown, MatchType, SemanticSimilarityMetrics, SimilarityFactor,
    SimilarityScorer, SimilarityScoringConfig,
};
pub use tree_edit::{
    TreeEditDistance, ZhangShashaConfig, EditCost, EditOperation
};
pub use smart_diff_parser::MatchResult;
pub use smart_diff_parser::{Change, ChangeType};

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
