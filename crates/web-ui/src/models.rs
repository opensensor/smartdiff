//! Data models for API requests and responses

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[allow(dead_code)]
    pub options: CompareOptions,
}

/// Comparison options
#[derive(Debug, Deserialize, Default)]
pub struct CompareOptions {
    /// Minimum similarity threshold (0.0-1.0)
    #[serde(default = "default_threshold")]
    #[allow(dead_code)]
    pub threshold: f64,

    /// Whether to ignore whitespace changes
    #[serde(default)]
    #[allow(dead_code)]
    pub ignore_whitespace: bool,

    /// Whether to detect cross-file moves
    #[serde(default)]
    #[allow(dead_code)]
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

    /// Detailed analysis results
    pub analysis: AnalysisResult,

    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Comprehensive analysis result
#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    /// File information
    pub files: FileComparison,

    /// Function-level analysis
    pub functions: FunctionAnalysis,

    /// Change classification
    pub changes: ChangeAnalysis,

    /// Refactoring patterns detected
    pub refactoring_patterns: Vec<RefactoringPattern>,

    /// AST structure comparison
    pub structure: StructureComparison,
}

/// File comparison details
#[derive(Debug, Serialize)]
pub struct FileComparison {
    pub source: FileMetadata,
    pub target: FileMetadata,
    pub language: String,
    pub similarity: SimilarityScore,
}

/// File metadata
#[derive(Debug, Serialize)]
pub struct FileMetadata {
    pub path: String,
    pub lines: usize,
    pub functions: usize,
    pub classes: usize,
    pub complexity: f64,
}

/// Detailed similarity scores
#[derive(Debug, Serialize)]
pub struct SimilarityScore {
    pub overall: f64,
    pub structure: f64,
    pub content: f64,
    pub semantic: f64,
}

/// Function-level analysis
#[derive(Debug, Serialize)]
pub struct FunctionAnalysis {
    pub total_functions: usize,
    pub matched_functions: usize,
    pub function_matches: Vec<FunctionMatch>,
    pub average_similarity: f64,
}

/// Function match information
#[derive(Debug, Serialize)]
pub struct FunctionMatch {
    pub id: String,
    pub source_function: FunctionInfo,
    pub target_function: Option<FunctionInfo>,
    pub similarity: SimilarityScore,
    pub change_type: String,
    pub refactoring_pattern: Option<RefactoringPattern>,
}

/// Function information
#[derive(Debug, Serialize)]
pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
    pub start_line: usize,
    pub end_line: usize,
    pub complexity: usize,
    pub parameters: Vec<String>,
    pub return_type: String,
}

/// Change analysis
#[derive(Debug, Serialize)]
pub struct ChangeAnalysis {
    pub total_changes: usize,
    pub change_types: HashMap<String, usize>,
    pub detailed_changes: Vec<DetailedChange>,
    pub impact_assessment: ImpactAssessment,
}

/// Detailed change information
#[derive(Debug, Serialize)]
pub struct DetailedChange {
    pub id: String,
    pub change_type: String,
    pub description: String,
    pub confidence: f64,
    pub location: ChangeLocation,
    pub impact: String,
}

/// Change location
#[derive(Debug, Serialize)]
pub struct ChangeLocation {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub function: Option<String>,
}

/// Impact assessment
#[derive(Debug, Serialize)]
pub struct ImpactAssessment {
    pub risk_level: String,
    pub breaking_changes: usize,
    pub effort_estimate: String,
    pub affected_components: Vec<String>,
}

/// Refactoring pattern
#[derive(Debug, Serialize)]
pub struct RefactoringPattern {
    pub pattern_type: String,
    pub description: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub impact: String,
}

/// Structure comparison
#[derive(Debug, Serialize)]
pub struct StructureComparison {
    pub source_structure: StructureNode,
    pub target_structure: StructureNode,
    pub matches: Vec<StructureMatch>,
}

/// Structure node
#[derive(Debug, Serialize)]
pub struct StructureNode {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub children: Vec<StructureNode>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Structure match
#[derive(Debug, Serialize)]
pub struct StructureMatch {
    pub source_id: String,
    pub target_id: String,
    pub similarity: f64,
    pub change_type: String,
}

/// Multi-file analysis request
#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub files: Vec<FileInfo>,
    #[serde(default)]
    pub options: AnalyzeOptions,
}

/// Analysis options
#[derive(Debug, Deserialize, Default)]
pub struct AnalyzeOptions {
    #[serde(default)]
    #[allow(dead_code)]
    pub include_complexity: bool,
    #[serde(default)]
    #[allow(dead_code)]
    pub include_dependencies: bool,
    #[serde(default)]
    #[allow(dead_code)]
    pub include_signatures: bool,
    #[serde(default = "default_threshold")]
    #[allow(dead_code)]
    pub similarity_threshold: f64,
}

/// Multi-file analysis response
#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub files: Vec<FileAnalysisResult>,
    pub cross_file_analysis: CrossFileAnalysis,
    pub summary: AnalysisSummary,
    pub execution_time_ms: u64,
}

/// Individual file analysis result
#[derive(Debug, Serialize)]
pub struct FileAnalysisResult {
    pub file: FileMetadata,
    pub functions: Vec<FunctionInfo>,
    pub complexity_distribution: HashMap<String, usize>,
    pub dependencies: Vec<String>,
    pub issues: Vec<String>,
}

/// Cross-file analysis
#[derive(Debug, Serialize)]
pub struct CrossFileAnalysis {
    pub duplicate_functions: Vec<DuplicateFunction>,
    pub moved_functions: Vec<MovedFunction>,
    pub dependency_graph: Vec<Dependency>,
}

/// Duplicate function detection
#[derive(Debug, Serialize)]
pub struct DuplicateFunction {
    pub signature: String,
    pub locations: Vec<ChangeLocation>,
    pub similarity: f64,
}

/// Moved function detection
#[derive(Debug, Serialize)]
pub struct MovedFunction {
    pub function_name: String,
    pub from_file: String,
    pub to_file: String,
    pub confidence: f64,
}

/// Dependency information
#[derive(Debug, Serialize)]
pub struct Dependency {
    pub from_file: String,
    pub to_file: String,
    pub dependency_type: String,
    pub symbols: Vec<String>,
}

/// Analysis summary
#[derive(Debug, Serialize)]
pub struct AnalysisSummary {
    pub total_files: usize,
    pub total_functions: usize,
    pub average_complexity: f64,
    pub duplicate_rate: f64,
    pub dependency_count: usize,
}

/// Configuration request
#[derive(Debug, Deserialize)]
pub struct ConfigRequest {
    pub parser: Option<ParserConfig>,
    pub semantic: Option<SemanticConfig>,
    pub diff_engine: Option<DiffEngineConfig>,
}

/// Parser configuration
#[derive(Debug, Deserialize)]
pub struct ParserConfig {
    pub max_file_size: Option<usize>,
    pub parse_timeout: Option<u64>,
    pub enable_error_recovery: Option<bool>,
}

/// Semantic analysis configuration
#[derive(Debug, Deserialize)]
pub struct SemanticConfig {
    pub max_resolution_depth: Option<usize>,
    pub enable_cross_file_analysis: Option<bool>,
    pub symbol_cache_size: Option<usize>,
}

/// Diff engine configuration
#[derive(Debug, Deserialize)]
pub struct DiffEngineConfig {
    pub default_similarity_threshold: Option<f64>,
    pub enable_refactoring_detection: Option<bool>,
    pub enable_cross_file_tracking: Option<bool>,
    pub max_tree_depth: Option<usize>,
}

/// Configuration response
#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub message: String,
    pub updated_settings: HashMap<String, serde_json::Value>,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub memory_usage: MemoryUsage,
    pub components: HashMap<String, ComponentHealth>,
}

/// Memory usage information
#[derive(Debug, Serialize)]
pub struct MemoryUsage {
    pub used_mb: f64,
    pub available_mb: f64,
    pub peak_mb: f64,
}

/// Component health status
#[derive(Debug, Serialize)]
pub struct ComponentHealth {
    pub status: String,
    pub last_check: String,
    pub details: Option<String>,
}

/// Error response
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
    pub error_code: Option<String>,
}
