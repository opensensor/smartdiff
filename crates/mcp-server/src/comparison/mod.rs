//! Comparison context and state management

pub mod context;
pub mod manager;

pub use context::{ComparisonContext, ComparisonId, ComparisonParams, FunctionChange};
pub use manager::ComparisonManager;

