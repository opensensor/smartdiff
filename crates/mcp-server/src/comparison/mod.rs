//! Comparison context and state management

pub mod binary_comparison;
pub mod context;
pub mod manager;

pub use binary_comparison::{
    BinaryComparisonContext, BinaryComparisonId, BinaryComparisonManager, BinaryComparisonParams,
    BinaryComparisonSummary,
};
pub use context::{ComparisonId, ComparisonParams};
pub use manager::ComparisonManager;
