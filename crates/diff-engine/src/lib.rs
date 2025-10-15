//! Smart Code Diff Engine
//!
//! Core diff computation engine that implements tree edit distance algorithms,
//! function matching, and change classification.

pub mod binary_matcher;
pub mod changes;
pub mod cross_file_tracker;
pub mod engine;
pub mod file_refactoring_detector;
pub mod graph_matcher;
pub mod hungarian_matcher;
pub mod matching;
pub mod refactoring;
pub mod similarity_scorer;
pub mod smart_matcher;
pub mod symbol_migration_tracker;
pub mod tree_edit;

pub use binary_matcher::{
    BinaryFunctionInfo, BinaryFunctionMatch, BinaryFunctionMatcher, BinaryMatchType,
    BinaryMatcherConfig,
};
pub use changes::{
    AlternativeClassification, ChangeAnalysis, ChangeCharacteristic, ChangeClassificationConfig,
    ChangeClassifier, ChangeImpact, CharacteristicType, ClassificationEvidence,
    DetailedChangeClassification, EffortLevel, EvidenceType, ImpactLevel, RiskLevel,
};
pub use cross_file_tracker::{
    CrossFileMerge, CrossFileSplit, CrossFileTracker, CrossFileTrackerConfig,
    CrossFileTrackingResult, CrossFileTrackingStats, FileTrackingStats, FunctionMove,
    FunctionRenameMove, MoveType,
};
pub use engine::{DiffEngine, DiffError, DiffResult};
pub use file_refactoring_detector::{
    ContentFingerprint, FileMerge, FileMove, FileRefactoringDetector,
    FileRefactoringDetectorConfig, FileRefactoringResult, FileRefactoringStats, FileRename,
    FileSplit,
};
pub use graph_matcher::{
    DependencyChange, DependencyChangeType, FunctionMatch, FunctionMove as GraphFunctionMove,
    FunctionRename, GraphMatchResult, GraphMatcher, GraphMatcherConfig,
    MatchType as GraphMatchType,
};
pub use hungarian_matcher::{
    FunctionAssignment, HungarianMatchResult, HungarianMatcher, HungarianMatcherConfig,
    ManyToManyMapping, MappingType, MatchingStatistics,
};
pub use matching::{FunctionMatcher, SimilarityScore};
pub use refactoring::{
    ApiCompatibilityImpact, BeforeAfterComparison, RefactoringAnalysis, RefactoringCharacteristic,
    RefactoringCharacteristicType, RefactoringComplexity, RefactoringComplexityLevel,
    RefactoringDetectionConfig, RefactoringDetector, RefactoringEffort, RefactoringEvidence,
    RefactoringEvidenceType, RefactoringImpact, RefactoringImpactLevel, RefactoringPattern,
    RefactoringQualityMetrics, SizeComparison,
};
pub use similarity_scorer::{
    ASTSimilarityScore, ComprehensiveSimilarityScore, ContextSimilarityScore,
    DetailedSimilarityBreakdown, MatchType, SemanticSimilarityMetrics, SimilarityFactor,
    SimilarityScorer, SimilarityScoringConfig,
};
pub use smart_diff_parser::MatchResult;
pub use smart_diff_parser::{Change, ChangeType};
pub use smart_matcher::{SmartMatcher, SmartMatcherConfig};
pub use symbol_migration_tracker::{
    FileMigration, MigrationStatistics, ReferenceChange, ReferenceChangeType, SymbolMigration,
    SymbolMigrationResult, SymbolMigrationTracker, SymbolMigrationTrackerConfig,
};
pub use tree_edit::{EditCost, EditOperation, TreeEditDistance, ZhangShashaConfig};

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
