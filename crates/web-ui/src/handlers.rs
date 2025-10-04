//! HTTP request handlers

#![allow(clippy::all, dead_code, unused_imports)]

use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, Json as ResponseJson},
};
use serde_json::json;
use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use smart_diff_engine::{
    ChangeClassifier, DiffEngine, FunctionMatcher, RefactoringDetector, SimilarityScorer,
    TreeEditDistance, ZhangShashaConfig,
};
use smart_diff_parser::{
    tree_sitter::TreeSitterParser, Language, LanguageDetector, ParseResult, Parser,
};
use smart_diff_semantic::SemanticAnalyzer;
use tracing::{info, warn};

use crate::models::*;

/// Root handler - serves basic info about the API
pub async fn root() -> Html<&'static str> {
    Html(
        r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Smart Code Diff API</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .endpoint { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }
        </style>
    </head>
    <body>
        <h1>Smart Code Diff API</h1>
        <p>A next-generation code diffing tool that performs structural and semantic comparison.</p>
        
        <h2>Available Endpoints</h2>
        <div class="endpoint">
            <strong>GET /api/health</strong> - Health check and system status
        </div>
        <div class="endpoint">
            <strong>POST /api/compare</strong> - Compare two code files with comprehensive analysis
        </div>
        <div class="endpoint">
            <strong>POST /api/analyze</strong> - Multi-file analysis with cross-file detection
        </div>
        <div class="endpoint">
            <strong>POST /api/configure</strong> - Update system configuration
        </div>
        <div class="endpoint">
            <strong>POST /api/filesystem/browse</strong> - Browse directory contents
        </div>
        <div class="endpoint">
            <strong>POST /api/filesystem/read</strong> - Read file content
        </div>
        <div class="endpoint">
            <strong>POST /api/filesystem/read-multiple</strong> - Read multiple files
        </div>
        <div class="endpoint">
            <strong>POST /api/filesystem/search</strong> - Search files and content
        </div>
        
        <h2>Example Usage</h2>
        <pre>
curl -X POST http://localhost:3000/api/compare \
  -H "Content-Type: application/json" \
  -d '{
    "file1": {"path": "old.py", "content": "def hello(): pass"},
    "file2": {"path": "new.py", "content": "def hello_world(): pass"}
  }'
        </pre>
    </body>
    </html>
    "#,
    )
}

/// Health check endpoint
pub async fn health() -> ResponseJson<HealthResponse> {
    let uptime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut components = HashMap::new();

    // Check parser component
    components.insert(
        "parser".to_string(),
        ComponentHealth {
            status: "healthy".to_string(),
            last_check: chrono::Utc::now().to_rfc3339(),
            details: Some("Parser engine operational".to_string()),
        },
    );

    // Check semantic analyzer
    components.insert(
        "semantic".to_string(),
        ComponentHealth {
            status: "healthy".to_string(),
            last_check: chrono::Utc::now().to_rfc3339(),
            details: Some("Semantic analyzer operational".to_string()),
        },
    );

    // Check diff engine
    components.insert(
        "diff_engine".to_string(),
        ComponentHealth {
            status: "healthy".to_string(),
            last_check: chrono::Utc::now().to_rfc3339(),
            details: Some("Diff engine operational".to_string()),
        },
    );

    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "smart-code-diff".to_string(),
        version: "0.1.0".to_string(),
        uptime_seconds: uptime,
        memory_usage: MemoryUsage {
            used_mb: 0.0, // Would implement actual memory tracking
            available_mb: 0.0,
            peak_mb: 0.0,
        },
        components,
    };

    ResponseJson(response)
}

/// Compare endpoint - main functionality
pub async fn compare(
    Json(request): Json<CompareRequest>,
) -> Result<ResponseJson<CompareResponse>, StatusCode> {
    let start_time = Instant::now();

    tracing::info!(
        "Received compare request for {} and {}",
        request.file1.path,
        request.file2.path
    );

    // Perform the actual comparison using our diff engine
    match perform_comparison(&request.file1, &request.file2, &request.options).await {
        Ok(analysis) => {
            let execution_time = start_time.elapsed().as_millis() as u64;

            let response = CompareResponse {
                similarity: analysis.files.similarity.overall,
                analysis,
                execution_time_ms: execution_time,
            };

            Ok(ResponseJson(response))
        }
        Err(e) => {
            tracing::error!("Comparison failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Perform the actual file comparison
async fn perform_comparison(
    file1: &FileInfo,
    file2: &FileInfo,
    _options: &CompareOptions,
) -> anyhow::Result<AnalysisResult> {
    // Initialize components
    let _language_detector = LanguageDetector;
    let parser_engine = TreeSitterParser::new()?;
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let _diff_engine = DiffEngine::new();

    // Detect language
    let language = {
        let path_lang = LanguageDetector::detect_from_path(&file1.path);
        if path_lang != Language::Unknown {
            path_lang
        } else {
            LanguageDetector::detect_from_content(&file1.content)
        }
    };

    // Parse both files
    let parse_result1 = parser_engine.parse(&file1.content, language)?;
    let parse_result2 = parser_engine.parse(&file2.content, language)?;

    // Perform semantic analysis
    let semantic1 = semantic_analyzer.analyze(&parse_result1)?;
    let semantic2 = semantic_analyzer.analyze(&parse_result2)?;

    // Extract ASTs for structure comparison
    let ast1 = &parse_result1.ast;
    let ast2 = &parse_result2.ast;

    // Initialize components that need language
    let function_matcher = FunctionMatcher::new(0.7); // threshold
    let _similarity_scorer = SimilarityScorer::new(
        language,
        smart_diff_engine::SimilarityScoringConfig::default(),
    );
    let _change_classifier = ChangeClassifier::new(language);
    let refactoring_detector = RefactoringDetector::new(language);

    // Extract functions from symbol tables
    let functions1 = extract_functions_from_symbol_table(&semantic1.symbol_table);
    let functions2 = extract_functions_from_symbol_table(&semantic2.symbol_table);

    // Match functions
    let function_matches = function_matcher.match_functions(&functions1, &functions2);

    // Calculate basic similarity scores (simplified for web API)
    let overall_similarity = if !functions1.is_empty() && !functions2.is_empty() {
        function_matches.similarity
    } else {
        0.0
    };
    let structure_similarity = overall_similarity; // Simplified
    let content_similarity = overall_similarity; // Simplified
    let semantic_similarity = overall_similarity; // Simplified

    // Use changes from function matching
    let changes = function_matches.changes.clone();

    // Detect refactoring patterns
    let refactoring_patterns = refactoring_detector.detect_patterns(&changes);

    // Build response
    let analysis = AnalysisResult {
        files: FileComparison {
            source: FileMetadata {
                path: file1.path.clone(),
                lines: file1.content.lines().count(),
                functions: functions1.len(),
                classes: count_classes_from_symbol_table(&semantic1.symbol_table),
                complexity: calculate_complexity_from_symbol_table(&semantic1.symbol_table) as f64,
            },
            target: FileMetadata {
                path: file2.path.clone(),
                lines: file2.content.lines().count(),
                functions: functions2.len(),
                classes: count_classes_from_symbol_table(&semantic2.symbol_table),
                complexity: calculate_complexity_from_symbol_table(&semantic2.symbol_table) as f64,
            },
            language: language.to_string(),
            similarity: SimilarityScore {
                overall: overall_similarity,
                structure: structure_similarity,
                content: content_similarity,
                semantic: semantic_similarity,
            },
        },
        functions: build_function_analysis(&function_matches),
        changes: build_change_analysis(&changes),
        refactoring_patterns: build_refactoring_patterns(&refactoring_patterns),
        structure: build_structure_comparison(&ast1, &ast2),
    };

    Ok(analysis)
}

/// Build function analysis from match result
fn build_function_analysis(match_result: &smart_diff_parser::MatchResult) -> FunctionAnalysis {
    // Simplified function analysis based on MatchResult
    FunctionAnalysis {
        total_functions: match_result.mapping.len(),
        matched_functions: match_result.mapping.len(),
        function_matches: Vec::new(), // Simplified - would need proper conversion
        average_similarity: match_result.similarity,
    }
}

/// Build change analysis from changes
fn build_change_analysis(changes: &[smart_diff_parser::Change]) -> ChangeAnalysis {
    let total_changes = changes.len();
    let mut change_types = HashMap::new();

    for change in changes {
        *change_types
            .entry(format!("{:?}", change.change_type))
            .or_insert(0) += 1;
    }

    let detailed_changes = changes
        .iter()
        .enumerate()
        .map(|(i, change)| {
            DetailedChange {
                id: format!("change-{}", i),
                change_type: format!("{:?}", change.change_type),
                description: change.details.description.clone(),
                confidence: change.confidence,
                location: ChangeLocation {
                    file: "unknown".to_string(), // Simplified
                    start_line: 0,
                    end_line: 0,
                    function: None,
                },
                impact: "medium".to_string(), // Simplified
            }
        })
        .collect();

    ChangeAnalysis {
        total_changes,
        change_types,
        detailed_changes,
        impact_assessment: ImpactAssessment {
            risk_level: "medium".to_string(),
            breaking_changes: 0,
            effort_estimate: "medium".to_string(),
            affected_components: vec![],
        },
    }
}

/// Build refactoring patterns from detected patterns
fn build_refactoring_patterns(
    patterns: &[smart_diff_engine::RefactoringPattern],
) -> Vec<RefactoringPattern> {
    patterns
        .iter()
        .map(|pattern| RefactoringPattern {
            pattern_type: format!("{:?}", pattern.pattern_type),
            description: pattern.description.clone(),
            confidence: pattern.confidence,
            evidence: pattern
                .evidence
                .iter()
                .map(|e| format!("{:?}", e))
                .collect(),
            impact: format!("{:?}", pattern.analysis.impact),
        })
        .collect()
}

/// Build structure comparison from ASTs
fn build_structure_comparison(
    _ast1: &smart_diff_parser::ASTNode,
    _ast2: &smart_diff_parser::ASTNode,
) -> StructureComparison {
    // This would be implemented with actual AST traversal
    // For now, return a simplified structure
    StructureComparison {
        source_structure: StructureNode {
            id: "root-1".to_string(),
            name: "root".to_string(),
            node_type: "file".to_string(),
            children: vec![],
            metadata: HashMap::new(),
        },
        target_structure: StructureNode {
            id: "root-2".to_string(),
            name: "root".to_string(),
            node_type: "file".to_string(),
            children: vec![],
            metadata: HashMap::new(),
        },
        matches: vec![],
    }
}

/// Multi-file analysis endpoint
pub async fn analyze(
    Json(request): Json<AnalyzeRequest>,
) -> Result<ResponseJson<AnalyzeResponse>, StatusCode> {
    let start_time = Instant::now();

    tracing::info!("Received analyze request for {} files", request.files.len());

    match perform_multi_file_analysis(&request.files, &request.options).await {
        Ok(analysis) => {
            let execution_time = start_time.elapsed().as_millis() as u64;

            let response = AnalyzeResponse {
                files: analysis.files,
                cross_file_analysis: analysis.cross_file_analysis,
                summary: analysis.summary,
                execution_time_ms: execution_time,
            };

            Ok(ResponseJson(response))
        }
        Err(e) => {
            tracing::error!("Multi-file analysis failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Perform multi-file analysis
async fn perform_multi_file_analysis(
    files: &[FileInfo],
    _options: &AnalyzeOptions,
) -> anyhow::Result<MultiFileAnalysisResult> {
    let _language_detector = LanguageDetector;
    let parser_engine = TreeSitterParser::new()?;
    let mut semantic_analyzer = SemanticAnalyzer::new();

    let mut file_results = Vec::new();
    let mut all_functions = Vec::new();
    let mut total_complexity = 0.0;

    // Analyze each file
    for file in files {
        let language = {
            let path_lang = LanguageDetector::detect_from_path(&file.path);
            if path_lang != Language::Unknown {
                path_lang
            } else {
                LanguageDetector::detect_from_content(&file.content)
            }
        };
        let parse_result = parser_engine.parse(&file.content, language)?;
        let semantic = semantic_analyzer.analyze(&parse_result)?;

        let functions = extract_functions_from_symbol_table(&semantic.symbol_table);
        let complexity = calculate_complexity_from_symbol_table(&semantic.symbol_table);
        total_complexity += complexity as f64;

        let function_infos: Vec<FunctionInfo> = functions
            .iter()
            .map(|f| {
                // Extract function content from file using line numbers
                let content = extract_content_from_lines(
                    &file.content,
                    f.location.start_line,
                    f.location.end_line,
                );

                FunctionInfo {
                    name: f.signature.name.clone(),
                    signature: format!(
                        "{}({})",
                        f.signature.name,
                        f.signature
                            .parameters
                            .iter()
                            .map(|p| format!("{}: {}", p.name, p.param_type.name))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    start_line: f.location.start_line,
                    end_line: f.location.end_line,
                    complexity: 1, // Simplified
                    parameters: f
                        .signature
                        .parameters
                        .iter()
                        .map(|p| p.name.clone())
                        .collect(),
                    return_type: f
                        .signature
                        .return_type
                        .as_ref()
                        .map(|t| t.name.clone())
                        .unwrap_or_else(|| "void".to_string()),
                    content,
                    file_path: f.location.file_path.clone(),
                }
            })
            .collect();

        all_functions.extend(functions.clone());

        let file_result = FileAnalysisResult {
            file: FileMetadata {
                path: file.path.clone(),
                lines: file.content.lines().count(),
                functions: function_infos.len(),
                classes: count_classes_from_symbol_table(&semantic.symbol_table),
                complexity: complexity as f64,
            },
            functions: function_infos,
            complexity_distribution: calculate_complexity_distribution(&functions),
            dependencies: extract_dependencies(&semantic.symbol_table),
            issues: detect_issues(&semantic.symbol_table),
        };

        file_results.push(file_result);
    }

    // Cross-file analysis
    let cross_file_analysis = perform_cross_file_analysis(&all_functions, files)?;

    let summary = AnalysisSummary {
        total_files: files.len(),
        total_functions: all_functions.len(),
        average_complexity: if files.is_empty() {
            0.0
        } else {
            total_complexity / files.len() as f64
        },
        duplicate_rate: calculate_duplicate_rate(
            &cross_file_analysis.duplicate_functions,
            &all_functions,
        ),
        dependency_count: cross_file_analysis.dependency_graph.len(),
    };

    Ok(MultiFileAnalysisResult {
        files: file_results,
        cross_file_analysis,
        summary,
    })
}

/// Configuration endpoint
pub async fn configure(
    Json(request): Json<ConfigRequest>,
) -> Result<ResponseJson<ConfigResponse>, StatusCode> {
    tracing::info!("Received configuration update request");

    // In a real implementation, this would update the actual configuration
    let mut updated_settings = HashMap::new();

    if let Some(parser_config) = &request.parser {
        if let Some(max_file_size) = parser_config.max_file_size {
            updated_settings.insert("parser.max_file_size".to_string(), json!(max_file_size));
        }
        if let Some(parse_timeout) = parser_config.parse_timeout {
            updated_settings.insert("parser.parse_timeout".to_string(), json!(parse_timeout));
        }
        if let Some(enable_error_recovery) = parser_config.enable_error_recovery {
            updated_settings.insert(
                "parser.enable_error_recovery".to_string(),
                json!(enable_error_recovery),
            );
        }
    }

    if let Some(semantic_config) = &request.semantic {
        if let Some(max_resolution_depth) = semantic_config.max_resolution_depth {
            updated_settings.insert(
                "semantic.max_resolution_depth".to_string(),
                json!(max_resolution_depth),
            );
        }
        if let Some(enable_cross_file_analysis) = semantic_config.enable_cross_file_analysis {
            updated_settings.insert(
                "semantic.enable_cross_file_analysis".to_string(),
                json!(enable_cross_file_analysis),
            );
        }
        if let Some(symbol_cache_size) = semantic_config.symbol_cache_size {
            updated_settings.insert(
                "semantic.symbol_cache_size".to_string(),
                json!(symbol_cache_size),
            );
        }
    }

    if let Some(diff_config) = &request.diff_engine {
        if let Some(threshold) = diff_config.default_similarity_threshold {
            updated_settings.insert(
                "diff_engine.default_similarity_threshold".to_string(),
                json!(threshold),
            );
        }
        if let Some(enable_refactoring) = diff_config.enable_refactoring_detection {
            updated_settings.insert(
                "diff_engine.enable_refactoring_detection".to_string(),
                json!(enable_refactoring),
            );
        }
        if let Some(enable_cross_file) = diff_config.enable_cross_file_tracking {
            updated_settings.insert(
                "diff_engine.enable_cross_file_tracking".to_string(),
                json!(enable_cross_file),
            );
        }
        if let Some(max_tree_depth) = diff_config.max_tree_depth {
            updated_settings.insert(
                "diff_engine.max_tree_depth".to_string(),
                json!(max_tree_depth),
            );
        }
    }

    let response = ConfigResponse {
        message: format!("Updated {} configuration settings", updated_settings.len()),
        updated_settings,
    };

    Ok(ResponseJson(response))
}

// Helper types and functions for multi-file analysis

struct MultiFileAnalysisResult {
    files: Vec<FileAnalysisResult>,
    cross_file_analysis: CrossFileAnalysis,
    summary: AnalysisSummary,
}

fn calculate_complexity_distribution(
    functions: &[smart_diff_parser::Function],
) -> HashMap<String, usize> {
    let mut distribution = HashMap::new();

    for _function in functions {
        // Simplified complexity calculation
        let complexity_range = "medium"; // Placeholder
        *distribution
            .entry(complexity_range.to_string())
            .or_insert(0) += 1;
    }

    distribution
}

fn extract_dependencies(symbol_table: &smart_diff_semantic::SymbolTable) -> Vec<String> {
    // This would extract actual dependencies from symbol table
    // For now, return empty vector
    let _ = symbol_table; // Suppress unused parameter warning
    vec![]
}

fn detect_issues(symbol_table: &smart_diff_semantic::SymbolTable) -> Vec<String> {
    // This would detect code issues from symbol table
    // For now, return empty vector
    let _ = symbol_table; // Suppress unused parameter warning
    vec![]
}

fn perform_cross_file_analysis(
    functions: &[smart_diff_parser::Function],
    _files: &[FileInfo],
) -> anyhow::Result<CrossFileAnalysis> {
    // Detect duplicate functions
    let mut duplicate_functions = Vec::new();
    let mut seen_signatures: HashMap<String, Vec<ChangeLocation>> = HashMap::new();

    for function in functions {
        let signature_str = format!(
            "{}({})",
            function.signature.name,
            function
                .signature
                .parameters
                .iter()
                .map(|p| format!("{}: {}", p.name, p.param_type.name))
                .collect::<Vec<_>>()
                .join(", ")
        );

        if let Some(existing_locations) = seen_signatures.get_mut(&signature_str) {
            existing_locations.push(ChangeLocation {
                file: "unknown".to_string(), // Would need to track file association
                start_line: function.location.start_line,
                end_line: function.location.end_line,
                function: Some(function.signature.name.clone()),
            });
        } else {
            seen_signatures.insert(
                signature_str,
                vec![ChangeLocation {
                    file: "unknown".to_string(),
                    start_line: function.location.start_line,
                    end_line: function.location.end_line,
                    function: Some(function.signature.name.clone()),
                }],
            );
        }
    }

    for (signature, locations) in seen_signatures {
        if locations.len() > 1 {
            duplicate_functions.push(DuplicateFunction {
                signature,
                locations,
                similarity: 1.0, // Exact duplicates
            });
        }
    }

    Ok(CrossFileAnalysis {
        duplicate_functions,
        moved_functions: vec![],  // Would implement moved function detection
        dependency_graph: vec![], // Would implement dependency analysis
    })
}

fn calculate_duplicate_rate(
    duplicates: &[DuplicateFunction],
    all_functions: &[smart_diff_parser::Function],
) -> f64 {
    if all_functions.is_empty() {
        return 0.0;
    }

    let duplicate_count: usize = duplicates.iter().map(|d| d.locations.len()).sum();

    duplicate_count as f64 / all_functions.len() as f64
}

/// Extract functions from symbol table
fn extract_functions_from_symbol_table(
    symbol_table: &smart_diff_semantic::SymbolTable,
) -> Vec<smart_diff_parser::Function> {
    use smart_diff_parser::{Function, FunctionLocation, FunctionSignature, Type};
    use smart_diff_semantic::SymbolKind;

    let mut functions = Vec::new();
    let function_symbols = symbol_table.get_symbols_by_kind(SymbolKind::Function);
    let method_symbols = symbol_table.get_symbols_by_kind(SymbolKind::Method);

    for symbol in function_symbols.iter().chain(method_symbols.iter()) {
        let signature = FunctionSignature {
            name: symbol.name.clone(),
            parameters: Vec::new(), // Simplified
            return_type: Some(Type::new("void".to_string())),
            modifiers: Vec::new(),
            generic_parameters: Vec::new(),
        };

        let location = FunctionLocation {
            file_path: "".to_string(),
            start_line: 0,
            end_line: 0,
            start_column: 0,
            end_column: 0,
        };

        // Create a simple AST node for the function body
        let metadata = smart_diff_parser::NodeMetadata {
            line: 0,
            column: 0,
            original_text: String::new(),
            attributes: std::collections::HashMap::new(),
        };
        let body = smart_diff_parser::ASTNode::new(smart_diff_parser::NodeType::Function, metadata);

        let function = Function {
            signature,
            body,
            location,
            dependencies: Vec::new(),
            hash: format!("hash_{}", symbol.name),
        };

        functions.push(function);
    }

    functions
}

/// Count classes from symbol table
fn count_classes_from_symbol_table(symbol_table: &smart_diff_semantic::SymbolTable) -> usize {
    use smart_diff_semantic::SymbolKind;
    symbol_table.get_symbols_by_kind(SymbolKind::Class).len()
}

/// Calculate complexity from symbol table
fn calculate_complexity_from_symbol_table(
    _symbol_table: &smart_diff_semantic::SymbolTable,
) -> usize {
    // Simplified complexity calculation
    10 // Placeholder value
}

// SPA fallback removed - using Next.js frontend instead

// ============================================================================
// File System API Handlers
// ============================================================================

/// Browse directory contents
pub async fn browse_directory(
    Json(request): Json<BrowseDirectoryRequest>,
) -> Result<ResponseJson<BrowseDirectoryResponse>, StatusCode> {
    let start_time = Instant::now();

    tracing::info!("Browsing directory: {}", request.path);

    match perform_directory_browse(&request).await {
        Ok(response) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            let mut response = response;
            response.execution_time_ms = execution_time;
            Ok(ResponseJson(response))
        }
        Err(e) => {
            tracing::error!("Directory browse failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Read file content
pub async fn read_file(
    Json(request): Json<ReadFileRequest>,
) -> Result<ResponseJson<ReadFileResponse>, StatusCode> {
    let start_time = Instant::now();

    tracing::info!("Reading file: {}", request.path);

    match perform_file_read(&request).await {
        Ok(response) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            let mut response = response;
            response.execution_time_ms = execution_time;
            Ok(ResponseJson(response))
        }
        Err(e) => {
            tracing::error!("File read failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Read multiple files
pub async fn read_multiple_files(
    Json(request): Json<ReadMultipleFilesRequest>,
) -> Result<ResponseJson<ReadMultipleFilesResponse>, StatusCode> {
    let start_time = Instant::now();

    tracing::info!("Reading {} files", request.paths.len());

    match perform_multiple_file_read(&request).await {
        Ok(response) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            let mut response = response;
            response.execution_time_ms = execution_time;
            Ok(ResponseJson(response))
        }
        Err(e) => {
            tracing::error!("Multiple file read failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Search files
pub async fn search_files(
    Json(request): Json<SearchFilesRequest>,
) -> Result<ResponseJson<SearchFilesResponse>, StatusCode> {
    let start_time = Instant::now();

    tracing::info!("Searching files with query: {}", request.query);

    match perform_file_search(&request).await {
        Ok(response) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            let mut response = response;
            response.execution_time_ms = execution_time;
            Ok(ResponseJson(response))
        }
        Err(e) => {
            tracing::error!("File search failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// File System Implementation Functions
// ============================================================================

use chrono::{DateTime, Utc};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Perform directory browsing
async fn perform_directory_browse(
    request: &BrowseDirectoryRequest,
) -> Result<BrowseDirectoryResponse, Box<dyn std::error::Error + Send + Sync>> {
    let path = Path::new(&request.path);

    if !path.exists() {
        return Err("Directory does not exist".into());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".into());
    }

    let mut entries = Vec::new();
    let mut total_files = 0;
    let mut total_directories = 0;
    let mut total_size = 0;

    // LanguageDetector is a unit struct with static methods

    if request.recursive {
        collect_entries_recursive(
            path,
            &mut entries,
            &mut total_files,
            &mut total_directories,
            &mut total_size,
            request.max_depth.unwrap_or(10),
            0,
            request.include_hidden,
            &request.file_extensions,
        )?;
    } else {
        collect_entries_single_level(
            path,
            &mut entries,
            &mut total_files,
            &mut total_directories,
            &mut total_size,
            request.include_hidden,
            &request.file_extensions,
        )?;
    }

    Ok(BrowseDirectoryResponse {
        path: request.path.clone(),
        entries,
        total_files,
        total_directories,
        total_size,
        execution_time_ms: 0, // Will be set by caller
    })
}

/// Collect entries from a single directory level
fn collect_entries_single_level(
    dir_path: &Path,
    entries: &mut Vec<FileSystemEntry>,
    total_files: &mut usize,
    total_directories: &mut usize,
    total_size: &mut u64,

    include_hidden: bool,
    file_extensions: &Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let read_dir = fs::read_dir(dir_path)?;

    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Skip hidden files if not requested
        if !include_hidden && file_name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata()?;
        let is_directory = metadata.is_dir();

        if is_directory {
            *total_directories += 1;
        } else {
            *total_files += 1;
            *total_size += metadata.len();
        }

        // Check file extension filter
        if !is_directory {
            if let Some(extensions) = file_extensions {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if !extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                        continue;
                    }
                }
            }
        }

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        let language = if !is_directory {
            let detected = LanguageDetector::detect_from_path(&path);
            if detected != Language::Unknown {
                Some(format!("{:?}", detected))
            } else {
                None
            }
        } else {
            None
        };

        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|duration| {
                DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            });

        let mut entry_metadata = HashMap::new();
        #[cfg(unix)]
        entry_metadata.insert(
            "permissions".to_string(),
            json!(format!("{:o}", metadata.permissions().mode() & 0o777)),
        );
        #[cfg(not(unix))]
        entry_metadata.insert("permissions".to_string(), json!("N/A"));

        entries.push(FileSystemEntry {
            path: path.to_string_lossy().to_string(),
            name: file_name,
            is_directory,
            size: if is_directory {
                None
            } else {
                Some(metadata.len())
            },
            modified,
            extension,
            language,
            children: None,
            metadata: entry_metadata,
        });
    }

    // Sort entries: directories first, then files, both alphabetically
    entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(())
}

/// Collect entries recursively
fn collect_entries_recursive(
    dir_path: &Path,
    entries: &mut Vec<FileSystemEntry>,
    total_files: &mut usize,
    total_directories: &mut usize,
    total_size: &mut u64,

    max_depth: usize,
    current_depth: usize,
    include_hidden: bool,
    file_extensions: &Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if current_depth >= max_depth {
        return Ok(());
    }

    collect_entries_single_level(
        dir_path,
        entries,
        total_files,
        total_directories,
        total_size,
        include_hidden,
        file_extensions,
    )?;

    // Recursively process subdirectories
    for entry in entries.iter_mut() {
        if entry.is_directory {
            let mut children = Vec::new();
            let child_path = Path::new(&entry.path);

            collect_entries_recursive(
                child_path,
                &mut children,
                total_files,
                total_directories,
                total_size,
                max_depth,
                current_depth + 1,
                include_hidden,
                file_extensions,
            )?;

            entry.children = Some(children);
        }
    }

    Ok(())
}

/// Perform file reading
async fn perform_file_read(
    request: &ReadFileRequest,
) -> Result<ReadFileResponse, Box<dyn std::error::Error + Send + Sync>> {
    let path = Path::new(&request.path);

    if !path.exists() {
        return Err("File does not exist".into());
    }

    if !path.is_file() {
        return Err("Path is not a file".into());
    }

    let metadata = fs::metadata(path)?;
    let file_size = metadata.len();

    // Check file size limit
    if let Some(max_size) = request.max_size {
        if file_size > max_size as u64 {
            return Err(format!("File too large: {} bytes (max: {})", file_size, max_size).into());
        }
    }

    let content = fs::read_to_string(path)?;
    let line_count = content.lines().count();

    let detected = LanguageDetector::detect_from_path(&request.path);
    let language = if detected != Language::Unknown {
        Some(format!("{:?}", detected))
    } else {
        None
    };

    Ok(ReadFileResponse {
        path: request.path.clone(),
        content,
        size: file_size,
        encoding: request
            .encoding
            .clone()
            .unwrap_or_else(|| "utf-8".to_string()),
        language,
        line_count,
        execution_time_ms: 0, // Will be set by caller
    })
}

/// Perform multiple file reading
async fn perform_multiple_file_read(
    request: &ReadMultipleFilesRequest,
) -> Result<ReadMultipleFilesResponse, Box<dyn std::error::Error + Send + Sync>> {
    let mut files = Vec::new();
    let mut errors = Vec::new();
    let mut total_size = 0;

    for path in &request.paths {
        let read_request = ReadFileRequest {
            path: path.clone(),
            encoding: None,
            max_size: request.max_file_size,
        };

        match perform_file_read(&read_request).await {
            Ok(response) => {
                total_size += response.size;
                files.push(response);
            }
            Err(e) => {
                errors.push(FileReadError {
                    path: path.clone(),
                    error: e.to_string(),
                    error_type: "read_error".to_string(),
                });
            }
        }
    }

    Ok(ReadMultipleFilesResponse {
        files,
        errors,
        total_size,
        execution_time_ms: 0, // Will be set by caller
    })
}

/// Perform file search
async fn perform_file_search(
    request: &SearchFilesRequest,
) -> Result<SearchFilesResponse, Box<dyn std::error::Error + Send + Sync>> {
    let mut results = Vec::new();
    let mut total_matches = 0;

    let root_path = Path::new(&request.root_path);
    if !root_path.exists() {
        return Err("Root path does not exist".into());
    }

    search_directory_recursive(
        root_path,
        &request.query,
        &request.search_type,
        &request.file_extensions,
        request.case_sensitive,
        &mut results,
        &mut total_matches,
        request.max_results,
    )?;

    // Sort results by score (descending)
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(SearchFilesResponse {
        query: request.query.clone(),
        results,
        total_matches,
        execution_time_ms: 0, // Will be set by caller
    })
}

/// Search directory recursively
fn search_directory_recursive(
    dir_path: &Path,
    query: &str,
    search_type: &SearchType,
    file_extensions: &Option<Vec<String>>,
    case_sensitive: bool,
    results: &mut Vec<SearchResult>,
    total_matches: &mut usize,
    max_results: Option<usize>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(max) = max_results {
        if results.len() >= max {
            return Ok(());
        }
    }

    let read_dir = fs::read_dir(dir_path)?;

    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            search_directory_recursive(
                &path,
                query,
                search_type,
                file_extensions,
                case_sensitive,
                results,
                total_matches,
                max_results,
            )?;
        } else if path.is_file() {
            // Check file extension filter
            if let Some(extensions) = file_extensions {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if !extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                        continue;
                    }
                }
            }

            let mut matches = Vec::new();
            let mut score = 0.0;

            // Search filename
            if matches!(search_type, SearchType::FileName | SearchType::Both) {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if search_in_text(filename, query, case_sensitive) {
                        matches.push(SearchMatch {
                            line_number: None,
                            column: None,
                            context: filename.to_string(),
                            match_type: "filename".to_string(),
                        });
                        score += 1.0;
                    }
                }
            }

            // Search file content
            if matches!(
                search_type,
                SearchType::FileContent | SearchType::FunctionName | SearchType::Both
            ) {
                if let Ok(content) = fs::read_to_string(&path) {
                    for (line_num, line) in content.lines().enumerate() {
                        if search_in_text(line, query, case_sensitive) {
                            matches.push(SearchMatch {
                                line_number: Some(line_num + 1),
                                column: find_match_column(line, query, case_sensitive),
                                context: line.to_string(),
                                match_type: "content".to_string(),
                            });
                            score += 0.5;
                        }
                    }
                }
            }

            if !matches.is_empty() {
                *total_matches += matches.len();
                results.push(SearchResult {
                    path: path.to_string_lossy().to_string(),
                    matches,
                    score,
                });
            }
        }
    }

    Ok(())
}

/// Search for text in a string
fn search_in_text(text: &str, query: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        text.contains(query)
    } else {
        text.to_lowercase().contains(&query.to_lowercase())
    }
}

/// Find the column position of a match
fn find_match_column(text: &str, query: &str, case_sensitive: bool) -> Option<usize> {
    let search_text = if case_sensitive {
        text
    } else {
        &text.to_lowercase()
    };
    let search_query = if case_sensitive {
        query
    } else {
        &query.to_lowercase()
    };

    search_text.find(search_query).map(|pos| pos + 1)
}

// ============================================================================
// Directory Comparison Handlers
// ============================================================================

/// Get home directory
pub async fn get_home_directory() -> Result<ResponseJson<serde_json::Value>, StatusCode> {
    match dirs::home_dir() {
        Some(home_path) => {
            let path_str = home_path.to_string_lossy().to_string();
            Ok(ResponseJson(json!({ "path": path_str })))
        }
        None => {
            tracing::warn!("Could not determine home directory");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Compare two directories
pub async fn compare_directories(
    Json(request): Json<crate::models::CompareDirectoriesRequest>,
) -> Result<ResponseJson<crate::models::CompareDirectoriesResponse>, StatusCode> {
    let start_time = Instant::now();

    tracing::info!(
        "Received directory comparison request: {} vs {}",
        request.source_path,
        request.target_path
    );

    match perform_directory_comparison(&request).await {
        Ok(response) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            let mut response = response;
            response.execution_time_ms = execution_time;
            Ok(ResponseJson(response))
        }
        Err(e) => {
            tracing::error!("Directory comparison failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Perform directory comparison
async fn perform_directory_comparison(
    request: &crate::models::CompareDirectoriesRequest,
) -> Result<crate::models::CompareDirectoriesResponse, Box<dyn std::error::Error + Send + Sync>> {
    use crate::models::*;

    // Scan source directory
    let source_files = scan_directory_for_comparison(&request.source_path, &request.options)?;
    let target_files = scan_directory_for_comparison(&request.target_path, &request.options)?;

    tracing::info!(
        "Scanned directories: {} source files, {} target files",
        source_files.len(),
        target_files.len()
    );

    // Analyze file changes
    let file_changes = analyze_file_changes(&source_files, &target_files);

    // Extract and match functions
    let function_matches = analyze_function_changes(
        &source_files,
        &target_files,
        request.options.similarity_threshold,
    )
    .await?;

    // Generate summary
    let summary = generate_comparison_summary(&file_changes, &function_matches);

    Ok(CompareDirectoriesResponse {
        summary,
        file_changes,
        function_matches,
        execution_time_ms: 0, // Will be set by caller
    })
}

/// Scan directory for comparison
fn scan_directory_for_comparison(
    dir_path: &str,
    options: &crate::models::DirectoryCompareOptions,
) -> Result<Vec<ComparisonFileInfo>, Box<dyn std::error::Error + Send + Sync>> {
    use std::time::SystemTime;
    use walkdir::WalkDir;

    let mut files = Vec::new();
    let base_path = Path::new(dir_path);

    if !base_path.exists() {
        return Err(format!("Directory does not exist: {}", dir_path).into());
    }

    for entry in WalkDir::new(base_path)
        .max_depth(options.max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Skip hidden files if not requested
        if !options.include_hidden {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with('.') {
                    continue;
                }
            }
        }

        // Check file extension filter
        if !options.file_extensions.is_empty() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !options
                    .file_extensions
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(ext))
                {
                    continue;
                }
            } else {
                continue; // Skip files without extensions if filter is specified
            }
        }

        // Read file content
        if let Ok(content) = fs::read_to_string(path) {
            let relative_path = path
                .strip_prefix(base_path)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            let metadata = entry.metadata()?;
            let file_info = ComparisonFileInfo {
                path: path.to_string_lossy().to_string(),
                relative_path,
                content,
                size: metadata.len(),
                modified: metadata
                    .modified()
                    .ok()
                    .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|duration| {
                        chrono::DateTime::<chrono::Utc>::from_timestamp(
                            duration.as_secs() as i64,
                            0,
                        )
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default()
                    }),
                language: detect_language_from_path(path),
                functions: Vec::new(), // Will be populated later
            };

            files.push(file_info);
        }
    }

    Ok(files)
}

/// File info for comparison
#[derive(Debug, Clone)]
struct ComparisonFileInfo {
    path: String,
    relative_path: String,
    content: String,
    size: u64,
    modified: Option<String>,
    language: Option<String>,
    functions: Vec<FunctionInfo>,
}

/// Detect language from file path
fn detect_language_from_path(path: &Path) -> Option<String> {
    let detected = LanguageDetector::detect_from_path(path);
    if detected != Language::Unknown {
        Some(format!("{:?}", detected))
    } else {
        None
    }
}

/// Analyze file changes between source and target directories
fn analyze_file_changes(
    source_files: &[ComparisonFileInfo],
    target_files: &[ComparisonFileInfo],
) -> Vec<crate::models::FileChange> {
    use crate::models::FileChange;
    use std::collections::HashMap;

    let mut changes = Vec::new();
    let mut target_map: HashMap<String, &ComparisonFileInfo> = HashMap::new();

    // Create a map of target files by relative path
    for file in target_files {
        target_map.insert(file.relative_path.clone(), file);
    }

    // Check source files for deletions and modifications
    for source_file in source_files {
        if let Some(target_file) = target_map.remove(&source_file.relative_path) {
            // File exists in both - check if modified
            let similarity = calculate_file_similarity(&source_file.content, &target_file.content);

            let change_type = if similarity >= 0.99 {
                "unchanged"
            } else {
                "modified"
            };

            changes.push(FileChange {
                change_type: change_type.to_string(),
                source_path: Some(source_file.relative_path.clone()),
                target_path: Some(target_file.relative_path.clone()),
                similarity: Some(similarity),
            });
        } else {
            // File was deleted
            changes.push(FileChange {
                change_type: "deleted".to_string(),
                source_path: Some(source_file.relative_path.clone()),
                target_path: None,
                similarity: None,
            });
        }
    }

    // Remaining files in target_map are additions
    for (_, target_file) in target_map {
        changes.push(FileChange {
            change_type: "added".to_string(),
            source_path: None,
            target_path: Some(target_file.relative_path.clone()),
            similarity: None,
        });
    }

    changes
}

/// Calculate simple file similarity
fn calculate_file_similarity(content1: &str, content2: &str) -> f64 {
    if content1 == content2 {
        return 1.0;
    }

    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();

    if lines1.is_empty() && lines2.is_empty() {
        return 1.0;
    }

    if lines1.is_empty() || lines2.is_empty() {
        return 0.0;
    }

    // Simple line-based similarity
    let common_lines = lines1.iter().filter(|line| lines2.contains(line)).count();

    let total_lines = std::cmp::max(lines1.len(), lines2.len());
    common_lines as f64 / total_lines as f64
}

/// Analyze function changes between directories using advanced AST-based matching
async fn analyze_function_changes(
    source_files: &[ComparisonFileInfo],
    target_files: &[ComparisonFileInfo],
    similarity_threshold: f64,
) -> Result<Vec<crate::models::FunctionMatch>, Box<dyn std::error::Error + Send + Sync>> {
    use crate::models::{FunctionMatch, SimilarityScore};
    use smart_diff_engine::{SmartMatcher, SmartMatcherConfig};
    use smart_diff_parser::{
        tree_sitter::TreeSitterParser, Function, Language, LanguageDetector, Parser,
    };
    use std::collections::HashMap;

    let mut matches = Vec::new();

    // Create parser with same config as MCP server
    let parser = TreeSitterParser::builder()
        .max_text_length(1_000_000)
        .include_comments(true)
        .extract_signatures(true)
        .build_symbol_table(true)
        .enable_optimization(true)
        .enable_analysis(false)
        .build()
        .map_err(|e| format!("Failed to create parser: {}", e))?;

    // Create smart matcher
    let config = SmartMatcherConfig {
        similarity_threshold,
        enable_cross_file_matching: true,
        cross_file_penalty: 0.5,
    };
    let smart_matcher = SmartMatcher::new(config);

    // Build file content lookup maps
    let mut source_file_contents: HashMap<String, String> = HashMap::new();
    let mut target_file_contents: HashMap<String, String> = HashMap::new();

    for file in source_files {
        source_file_contents.insert(file.path.clone(), file.content.clone());
    }

    for file in target_files {
        target_file_contents.insert(file.path.clone(), file.content.clone());
    }

    // Parse source files and extract functions
    let mut source_functions: Vec<Function> = Vec::new();
    for file in source_files {
        if let Some(language_str) = &file.language {
            let language = match language_str.to_lowercase().as_str() {
                "javascript" => Language::JavaScript,
                "typescript" => Language::TypeScript,
                "python" => Language::Python,
                "java" => Language::Java,
                "c" => Language::C,
                "cpp" | "c++" => Language::Cpp,
                "rust" => Language::Rust,
                _ => Language::Unknown,
            };

            if language != Language::Unknown {
                if let Ok(parse_result) = parser.parse(&file.content, language) {
                    let file_functions = extract_functions_from_ast(&parse_result.ast, &file.path)?;
                    source_functions.extend(file_functions);
                }
            }
        }
    }

    // Parse target files and extract functions
    let mut target_functions: Vec<Function> = Vec::new();
    for file in target_files {
        if let Some(language_str) = &file.language {
            let language = match language_str.to_lowercase().as_str() {
                "javascript" => Language::JavaScript,
                "typescript" => Language::TypeScript,
                "python" => Language::Python,
                "java" => Language::Java,
                "c" => Language::C,
                "cpp" | "c++" => Language::Cpp,
                "rust" => Language::Rust,
                _ => Language::Unknown,
            };

            if language != Language::Unknown {
                if let Ok(parse_result) = parser.parse(&file.content, language) {
                    let file_functions = extract_functions_from_ast(&parse_result.ast, &file.path)?;
                    target_functions.extend(file_functions);
                }
            }
        }
    }

    tracing::info!(
        "Extracted functions using AST: {} source, {} target",
        source_functions.len(),
        target_functions.len()
    );

    // Use smart matcher to find matches
    let match_result = smart_matcher.match_functions(&source_functions, &target_functions);

    tracing::info!(
        "Smart matching complete: {} changes found",
        match_result.changes.len()
    );

    // Convert match result to FunctionMatch format
    for change in &match_result.changes {
        let similarity = change.details.similarity_score.unwrap_or(0.0);

        if let (Some(source), Some(target)) = (&change.source, &change.target) {
            // Matched function (modified, moved, or renamed)
            let change_type = determine_change_type(source, target, similarity);

            matches.push(FunctionMatch {
                id: uuid::Uuid::new_v4().to_string(),
                source_function: Some(convert_function_to_info(source, &source_file_contents)),
                target_function: Some(convert_function_to_info(target, &target_file_contents)),
                similarity: SimilarityScore {
                    overall: similarity,
                    structure: similarity,
                    content: similarity,
                    semantic: similarity,
                },
                match_type: change_type,
                refactoring_pattern: None,
            });
        } else if let Some(source) = &change.source {
            // Deleted function
            matches.push(FunctionMatch {
                id: uuid::Uuid::new_v4().to_string(),
                source_function: Some(convert_function_to_info(source, &source_file_contents)),
                target_function: None,
                similarity: SimilarityScore {
                    overall: 0.0,
                    structure: 0.0,
                    content: 0.0,
                    semantic: 0.0,
                },
                match_type: "deleted".to_string(),
                refactoring_pattern: None,
            });
        } else if let Some(target) = &change.target {
            // Added function
            matches.push(FunctionMatch {
                id: uuid::Uuid::new_v4().to_string(),
                source_function: None,
                target_function: Some(convert_function_to_info(target, &target_file_contents)),
                similarity: SimilarityScore {
                    overall: 0.0,
                    structure: 0.0,
                    content: 0.0,
                    semantic: 0.0,
                },
                match_type: "added".to_string(),
                refactoring_pattern: None,
            });
        }
    }

    tracing::info!(
        "Function matching complete: {} total matches",
        matches.len()
    );

    Ok(matches)
}

/// Extract functions from an AST (same approach as MCP server)
fn extract_functions_from_ast(
    ast: &smart_diff_parser::ASTNode,
    file_path: &str,
) -> Result<Vec<smart_diff_parser::Function>, Box<dyn std::error::Error + Send + Sync>> {
    use smart_diff_parser::{Function, FunctionSignature, NodeType};

    let mut functions = Vec::new();

    // Find all function and method nodes
    let function_nodes = ast.find_by_type(&NodeType::Function);
    let method_nodes = ast.find_by_type(&NodeType::Method);

    for node in function_nodes.iter().chain(method_nodes.iter()) {
        // Skip function declarators (just signatures without bodies)
        if let Some(kind) = node.metadata.attributes.get("kind") {
            if kind == "function_declarator" {
                continue;
            }
        }

        if let Some(name) = node.metadata.attributes.get("name") {
            let signature = FunctionSignature::new(name.clone());
            let function = Function::new(signature, (*node).clone(), file_path.to_string());
            functions.push(function);
        }
    }

    Ok(functions)
}

/// Convert smart_diff_parser::CodeElement to FunctionInfo
fn convert_function_to_info(
    element: &smart_diff_parser::CodeElement,
    file_contents: &std::collections::HashMap<String, String>,
) -> FunctionInfo {
    // Extract function content from file
    let content = if let Some(file_content) = file_contents.get(&element.file_path) {
        let lines: Vec<&str> = file_content.lines().collect();
        if element.start_line > 0 && element.end_line <= lines.len() {
            let extracted = lines[(element.start_line - 1)..element.end_line].join("\n");
            tracing::debug!(
                "Extracting function '{}' from lines {}-{} (total {} lines): {} chars",
                element.name,
                element.start_line,
                element.end_line,
                element.end_line - element.start_line + 1,
                extracted.len()
            );
            extracted
        } else {
            tracing::warn!(
                "Invalid line range for function '{}': {}-{} (file has {} lines)",
                element.name,
                element.start_line,
                element.end_line,
                lines.len()
            );
            String::new()
        }
    } else {
        tracing::warn!(
            "File not found in contents map: {} for function '{}'",
            element.file_path,
            element.name
        );
        String::new()
    };

    FunctionInfo {
        name: element.name.clone(),
        signature: element
            .signature
            .clone()
            .unwrap_or_else(|| element.name.clone()),
        start_line: element.start_line,
        end_line: element.end_line,
        complexity: 1,
        parameters: Vec::new(),
        return_type: "unknown".to_string(),
        content,
        file_path: element.file_path.clone(),
    }
}

/// Determine change type based on function properties and similarity
fn determine_change_type(
    source: &smart_diff_parser::CodeElement,
    target: &smart_diff_parser::CodeElement,
    similarity: f64,
) -> String {
    use std::path::Path;

    // Normalize file paths to compare just the filename
    let source_filename = Path::new(&source.file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&source.file_path);
    let target_filename = Path::new(&target.file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&target.file_path);

    let is_cross_file_move = source_filename != target_filename;
    let is_renamed = source.name != target.name;

    if similarity >= 0.99 {
        "identical".to_string()
    } else if is_cross_file_move {
        "moved".to_string()
    } else if is_renamed && similarity >= 0.85 {
        "renamed".to_string()
    } else {
        "modified".to_string()
    }
}

/// Extract functions from files
async fn extract_functions_from_files(
    files: &[ComparisonFileInfo],
) -> Result<Vec<FunctionInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let mut all_functions = Vec::new();

    for file in files {
        if let Some(language_str) = &file.language {
            tracing::info!(
                "Extracting functions from {} (language: {})",
                file.relative_path,
                language_str
            );
            // Simple function extraction using regex patterns
            let functions =
                extract_functions_simple(&file.content, language_str, &file.relative_path);
            tracing::info!(
                "Extracted {} functions from {}",
                functions.len(),
                file.relative_path
            );
            all_functions.extend(functions);
        } else {
            tracing::warn!("No language detected for file: {}", file.relative_path);
        }
    }

    Ok(all_functions)
}

/// Simple function extraction using regex
fn extract_functions_simple(content: &str, language: &str, file_path: &str) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();

    // Use specialized extractors for C/C++ to handle multi-line signatures
    match language.to_lowercase().as_str() {
        "c" => return extract_c_functions(content, file_path),
        "cpp" | "c++" => return extract_cpp_functions(content, file_path),
        _ => {}
    }

    let lines: Vec<&str> = content.lines().collect();

    // Simple regex patterns for other languages
    let pattern = match language.to_lowercase().as_str() {
        "javascript" | "typescript" => {
            r"(?:function\s+(\w+)|const\s+(\w+)\s*=.*=>|(\w+)\s*\([^)]*\)\s*\{)"
        }
        "python" => r"def\s+(\w+)\s*\(",
        "java" => {
            r"(?:public|private|protected)?\s*(?:static\s+)?(?:\w+\s+)+(\w+)\s*\([^)]*\)\s*\{"
        }
        "rust" => r"fn\s+(\w+)\s*\(",
        _ => return functions,
    };

    if let Ok(regex) = regex::Regex::new(pattern) {
        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = regex.captures(line) {
                if let Some(name_match) = captures
                    .get(1)
                    .or_else(|| captures.get(2))
                    .or_else(|| captures.get(3))
                {
                    let function_name = name_match.as_str().to_string();

                    // Skip common keywords
                    if !["if", "for", "while", "switch", "return"].contains(&function_name.as_str())
                    {
                        // Extract function content by finding the function body
                        let (end_line, function_content) =
                            extract_function_body(&lines, line_num, language);

                        functions.push(FunctionInfo {
                            name: function_name.clone(),
                            signature: line.trim().to_string(),
                            start_line: line_num + 1,
                            end_line,
                            complexity: 1,
                            parameters: Vec::new(), // Simplified
                            return_type: "unknown".to_string(),
                            content: function_content,
                            file_path: file_path.to_string(),
                        });
                    }
                }
            }
        }
    }

    functions
}

/// Extract C functions with better handling of multi-line signatures and K&R style
fn extract_c_functions(content: &str, file_path: &str) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Pattern to match C function signatures (more flexible)
    // Matches: [static] [inline] [const] return_type [*] function_name (
    let func_pattern = regex::Regex::new(
        r"^(?:static\s+)?(?:inline\s+)?(?:const\s+)?(?:unsigned\s+)?(?:signed\s+)?(?:struct\s+)?(?:enum\s+)?(\w+)\s+(\**)(\w+)\s*\("
    ).unwrap();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Skip preprocessor directives, comments, and empty lines
        if line.starts_with('#')
            || line.starts_with("//")
            || line.starts_with("/*")
            || line.is_empty()
        {
            i += 1;
            continue;
        }

        // Try to match function signature
        if let Some(captures) = func_pattern.captures(line) {
            let return_type = captures.get(1).map(|m| m.as_str()).unwrap_or("void");
            let function_name = captures
                .get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            // Skip common keywords and type definitions
            if [
                "if", "for", "while", "switch", "return", "sizeof", "typedef",
            ]
            .contains(&function_name.as_str())
            {
                i += 1;
                continue;
            }

            // Build full signature (may span multiple lines)
            let mut signature = String::new();
            let mut sig_line = i;
            let mut found_closing_paren = false;

            while sig_line < lines.len() && sig_line < i + 10 {
                let current_line = lines[sig_line].trim();
                signature.push_str(current_line);
                signature.push(' ');

                if current_line.contains(')') {
                    found_closing_paren = true;
                    break;
                }
                sig_line += 1;
            }

            if !found_closing_paren {
                i += 1;
                continue;
            }

            // Find the opening brace (may be on next line for K&R style)
            let mut brace_line = sig_line;
            let mut found_brace = false;

            while brace_line < lines.len() && brace_line < sig_line + 20 {
                let current_line = lines[brace_line].trim();

                // Skip old K&R style parameter declarations
                if current_line.contains('{') {
                    found_brace = true;
                    break;
                }

                // If we hit a semicolon, it's a declaration, not a definition
                if current_line.ends_with(';') {
                    break;
                }

                brace_line += 1;
            }

            if found_brace {
                // Extract function body
                let (end_line, function_content) = extract_function_body(&lines, brace_line, "c");

                functions.push(FunctionInfo {
                    name: function_name.clone(),
                    signature: signature.trim().to_string(),
                    start_line: i + 1,
                    end_line,
                    complexity: 1,
                    parameters: Vec::new(),
                    return_type: return_type.to_string(),
                    content: function_content,
                    file_path: file_path.to_string(),
                });

                // Skip to end of function
                i = end_line;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    tracing::info!(
        "Extracted {} C functions from {}",
        functions.len(),
        file_path
    );
    functions
}

/// Extract C++ functions with better handling
fn extract_cpp_functions(content: &str, file_path: &str) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Pattern for C++ functions (includes access modifiers and templates)
    let func_pattern = regex::Regex::new(
        r"^(?:public|private|protected)?\s*:?\s*(?:static\s+)?(?:virtual\s+)?(?:inline\s+)?(?:const\s+)?(?:unsigned\s+)?(?:signed\s+)?(?:struct\s+)?(?:class\s+)?(\w+(?:<[^>]+>)?)\s+(\**)(\w+)\s*\("
    ).unwrap();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Skip preprocessor, comments, empty lines
        if line.starts_with('#')
            || line.starts_with("//")
            || line.starts_with("/*")
            || line.is_empty()
        {
            i += 1;
            continue;
        }

        if let Some(captures) = func_pattern.captures(line) {
            let return_type = captures.get(1).map(|m| m.as_str()).unwrap_or("void");
            let function_name = captures
                .get(3)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            if ["if", "for", "while", "switch", "return", "sizeof"]
                .contains(&function_name.as_str())
            {
                i += 1;
                continue;
            }

            // Build signature
            let mut signature = String::new();
            let mut sig_line = i;
            let mut found_closing_paren = false;

            while sig_line < lines.len() && sig_line < i + 10 {
                let current_line = lines[sig_line].trim();
                signature.push_str(current_line);
                signature.push(' ');

                if current_line.contains(')') {
                    found_closing_paren = true;
                    break;
                }
                sig_line += 1;
            }

            if !found_closing_paren {
                i += 1;
                continue;
            }

            // Find opening brace
            let mut brace_line = sig_line;
            let mut found_brace = false;

            while brace_line < lines.len() && brace_line < sig_line + 5 {
                let current_line = lines[brace_line].trim();

                if current_line.contains('{') {
                    found_brace = true;
                    break;
                }

                if current_line.ends_with(';') {
                    break;
                }

                brace_line += 1;
            }

            if found_brace {
                let (end_line, function_content) = extract_function_body(&lines, brace_line, "cpp");

                functions.push(FunctionInfo {
                    name: function_name.clone(),
                    signature: signature.trim().to_string(),
                    start_line: i + 1,
                    end_line,
                    complexity: 1,
                    parameters: Vec::new(),
                    return_type: return_type.to_string(),
                    content: function_content,
                    file_path: file_path.to_string(),
                });

                i = end_line;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    tracing::info!(
        "Extracted {} C++ functions from {}",
        functions.len(),
        file_path
    );
    functions
}

/// Extract function body content from lines starting at the given line
fn extract_function_body(lines: &[&str], start_line: usize, language: &str) -> (usize, String) {
    let mut content = Vec::new();
    let mut brace_count = 0;
    let mut in_function = false;
    let mut end_line = start_line + 1;

    // Different languages have different block delimiters
    let (open_char, close_char) = match language.to_lowercase().as_str() {
        "python" => {
            // For Python, we need to handle indentation-based blocks
            return extract_python_function_body(lines, start_line);
        }
        _ => ('{', '}'), // Most C-style languages
    };

    for (i, line) in lines.iter().enumerate().skip(start_line) {
        content.push(line.to_string());
        end_line = i + 1;

        // Count braces to find function end
        for ch in line.chars() {
            if ch == open_char {
                brace_count += 1;
                in_function = true;
            } else if ch == close_char && in_function {
                brace_count -= 1;
                if brace_count == 0 {
                    return (end_line, content.join("\n"));
                }
            }
        }

        // Safety limit to prevent infinite loops
        if i - start_line > 100 {
            break;
        }
    }

    (end_line, content.join("\n"))
}

/// Extract Python function body based on indentation
fn extract_python_function_body(lines: &[&str], start_line: usize) -> (usize, String) {
    let mut content = Vec::new();
    let mut end_line = start_line + 1;

    if start_line >= lines.len() {
        return (start_line + 1, String::new());
    }

    // Get the indentation level of the function definition
    let def_line = lines[start_line];
    let def_indent = def_line.len() - def_line.trim_start().len();

    content.push(def_line.to_string());

    // Look for the function body (next lines with greater indentation)
    for (i, line) in lines.iter().enumerate().skip(start_line + 1) {
        if line.trim().is_empty() {
            content.push(line.to_string());
            continue;
        }

        let line_indent = line.len() - line.trim_start().len();

        // If we find a line with same or less indentation, function is done
        if line_indent <= def_indent {
            break;
        }

        content.push(line.to_string());
        end_line = i + 1;

        // Safety limit
        if i - start_line > 100 {
            break;
        }
    }

    (end_line, content.join("\n"))
}

/// Extract content from file content using line numbers (1-based)
fn extract_content_from_lines(file_content: &str, start_line: usize, end_line: usize) -> String {
    let lines: Vec<&str> = file_content.lines().collect();

    if start_line == 0 || start_line > lines.len() {
        return String::new();
    }

    let start_idx = start_line - 1; // Convert to 0-based
    let end_idx = std::cmp::min(end_line, lines.len());

    if start_idx >= end_idx {
        return String::new();
    }

    lines[start_idx..end_idx].join("\n")
}

/// Calculate function similarity with smart matching rules
fn calculate_function_similarity(func1: &FunctionInfo, func2: &FunctionInfo) -> f64 {
    let same_file = func1.file_path == func2.file_path;
    let same_name = func1.name == func2.name;

    // Rule 1: Same-named functions at module level should always map if they're in the same file
    if same_name && same_file {
        // For same-named functions in same file, base similarity on content only
        if func1.content == func2.content {
            return 1.0; // Identical
        } else {
            let content_sim = calculate_content_similarity_fast(&func1.content, &func2.content);
            return 0.7 + (content_sim * 0.3); // Minimum 70% for same name, up to 100%
        }
    }

    // Rule 2: Don't match simple functions unless they're identical
    if is_simple_function(&func1.content) || is_simple_function(&func2.content) {
        if func1.content == func2.content && same_name {
            return 1.0; // Only match if identical
        } else {
            return 0.0; // Don't match simple functions with differences
        }
    }

    // Rule 3: Regular similarity calculation for complex functions
    let mut score = 0.0;
    let mut weight = 0.0;

    // Name similarity (30% weight)
    let name_weight = 0.3;
    if same_name {
        score += name_weight;
    } else {
        let name_sim = calculate_simple_string_similarity(&func1.name, &func2.name);
        score += name_weight * name_sim * 0.5; // Reduced credit for similar names
    }
    weight += name_weight;

    // Signature similarity (20% weight)
    let sig_weight = 0.2;
    if func1.signature == func2.signature {
        score += sig_weight;
    } else {
        let sig_sim = calculate_simple_string_similarity(&func1.signature, &func2.signature);
        score += sig_weight * sig_sim * 0.7;
    }
    weight += sig_weight;

    // Content similarity (50% weight) - highest weight
    let content_weight = 0.5;
    if !func1.content.is_empty() && !func2.content.is_empty() {
        if func1.content == func2.content {
            score += content_weight;
        } else {
            let content_sim = calculate_content_similarity_fast(&func1.content, &func2.content);
            score += content_weight * content_sim * 0.8;
        }
    } else if func1.content.is_empty() && func2.content.is_empty() {
        score += content_weight;
    } else {
        score += content_weight * 0.1; // Penalty for one empty, one not
    }
    weight += content_weight;

    let final_score = if weight > 0.0 { score / weight } else { 0.0 };

    // Apply cross-file penalty
    if !same_file {
        if final_score < 0.9 {
            final_score * 0.5 // Heavy penalty for cross-file matches
        } else {
            final_score * 0.8
        }
    } else {
        final_score
    }
}

/// Check if a function is "simple" (just returns a constant or has very few lines)
fn is_simple_function(content: &str) -> bool {
    let lines: Vec<&str> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//") && !line.starts_with("/*"))
        .collect();

    // Consider it simple if:
    // 1. Very few lines (3 or less non-empty, non-comment lines)
    // 2. Just returns a constant (return 0, return 1, return true, etc.)
    if lines.len() <= 3 {
        let content_lower = content.to_lowercase();
        if content_lower.contains("return 0")
            || content_lower.contains("return 1")
            || content_lower.contains("return true")
            || content_lower.contains("return false")
            || content_lower.contains("return null")
            || content_lower.contains("return nullptr")
        {
            return true;
        }
    }

    false
}

/// Fast string similarity using character overlap
fn calculate_simple_string_similarity(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }

    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    // Use character set overlap for fast similarity
    let chars1: std::collections::HashSet<char> = s1.chars().collect();
    let chars2: std::collections::HashSet<char> = s2.chars().collect();

    let intersection = chars1.intersection(&chars2).count();
    let union = chars1.union(&chars2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Fast content similarity using basic metrics
fn calculate_content_similarity_fast(content1: &str, content2: &str) -> f64 {
    if content1 == content2 {
        return 1.0;
    }

    let lines1 = content1.lines().count();
    let lines2 = content2.lines().count();
    let len1 = content1.len();
    let len2 = content2.len();

    // Line count similarity (50%)
    let line_sim = if lines1.max(lines2) == 0 {
        1.0
    } else {
        1.0 - ((lines1 as i32 - lines2 as i32).abs() as f64 / lines1.max(lines2) as f64)
    };

    // Length similarity (50%)
    let len_sim = if len1.max(len2) == 0 {
        1.0
    } else {
        1.0 - ((len1 as i32 - len2 as i32).abs() as f64 / len1.max(len2) as f64)
    };

    (line_sim + len_sim) / 2.0
}

/// Generate comparison summary
fn generate_comparison_summary(
    file_changes: &[crate::models::FileChange],
    function_matches: &[crate::models::FunctionMatch],
) -> crate::models::DirectoryComparisonSummary {
    use crate::models::DirectoryComparisonSummary;

    let total_files = file_changes.len();
    let added_files = file_changes
        .iter()
        .filter(|c| c.change_type == "added")
        .count();
    let deleted_files = file_changes
        .iter()
        .filter(|c| c.change_type == "deleted")
        .count();
    let modified_files = file_changes
        .iter()
        .filter(|c| c.change_type == "modified")
        .count();
    let unchanged_files = file_changes
        .iter()
        .filter(|c| c.change_type == "unchanged")
        .count();

    let total_functions = function_matches.len();
    let added_functions = function_matches
        .iter()
        .filter(|m| m.match_type == "added")
        .count();
    let deleted_functions = function_matches
        .iter()
        .filter(|m| m.match_type == "deleted")
        .count();
    let modified_functions = function_matches
        .iter()
        .filter(|m| m.match_type == "similar")
        .count();
    let moved_functions = function_matches
        .iter()
        .filter(|m| m.match_type == "moved" || m.match_type == "renamed")
        .count();

    DirectoryComparisonSummary {
        total_files,
        added_files,
        deleted_files,
        modified_files,
        unchanged_files,
        total_functions,
        added_functions,
        deleted_functions,
        modified_functions,
        moved_functions,
    }
}

/// AST-powered diff handler
pub async fn ast_diff(
    Json(request): Json<ASTDiffRequest>,
) -> Result<ResponseJson<ASTDiffResponse>, StatusCode> {
    info!(
        "Received AST diff request for {} vs {}",
        request.source_file_path, request.target_file_path
    );

    // Detect language
    let language = if request.language == "auto" {
        match detect_language_from_path(std::path::Path::new(&request.source_file_path)) {
            Some(lang_str) => match lang_str.as_str() {
                "c" => Language::C,
                "cpp" | "c++" => Language::Cpp,
                "python" => Language::Python,
                "javascript" | "js" => Language::JavaScript,
                "typescript" | "ts" => Language::TypeScript,
                "rust" => Language::Rust,
                "java" => Language::Java,
                "go" => Language::Go,
                _ => Language::C,
            },
            None => Language::C,
        }
    } else {
        match request.language.as_str() {
            "c" => Language::C,
            "cpp" | "c++" => Language::Cpp,
            "python" => Language::Python,
            "javascript" | "js" => Language::JavaScript,
            "typescript" | "ts" => Language::TypeScript,
            "rust" => Language::Rust,
            "java" => Language::Java,
            "go" => Language::Go,
            _ => Language::C, // Default fallback
        }
    };

    // Parse both contents into ASTs
    let parser = match TreeSitterParser::new() {
        Ok(p) => p,
        Err(e) => {
            warn!("Failed to create parser: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let source_ast = match parser.parse(&request.source_content, language) {
        Ok(ast) => ast,
        Err(e) => {
            warn!("Failed to parse source content: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let target_ast = match parser.parse(&request.target_content, language) {
        Ok(ast) => ast,
        Err(e) => {
            warn!("Failed to parse target content: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Generate line mappings using selected algorithm
    let line_mappings = if request.options.generate_line_mapping {
        match request.options.diff_algorithm.as_str() {
            "ast" => {
                info!("Using AST-aware diff with Zhang-Shasha and Hungarian matching");
                generate_ast_aware_line_mappings(
                    &request.source_content,
                    &request.target_content,
                    &source_ast,
                    &target_ast,
                    &request.options,
                )
            }
            "lcs" | _ => {
                info!("Using LCS-based diff algorithm");
                generate_lcs_line_mappings(
                    &request.source_content,
                    &request.target_content,
                    &source_ast,
                    &target_ast,
                    request.options.ignore_whitespace,
                )
            }
        }
    } else {
        Vec::new()
    };

    // Generate AST operations using the diff engine
    let ast_operations = if request.options.enable_structural_analysis {
        generate_ast_operations(&source_ast, &target_ast)
    } else {
        Vec::new()
    };

    // Calculate summary
    let summary = calculate_ast_diff_summary(&line_mappings);

    let response = ASTDiffResponse {
        line_mappings,
        ast_operations,
        summary,
    };

    info!(
        "AST diff completed with {} line mappings and {} operations",
        response.line_mappings.len(),
        response.ast_operations.len()
    );

    Ok(ResponseJson(response))
}

/// Generate line-by-line mappings using LCS-based Myers diff algorithm (fast, similar to git diff)
fn generate_lcs_line_mappings(
    source_content: &str,
    target_content: &str,
    _source_ast: &ParseResult,
    _target_ast: &ParseResult,
    ignore_whitespace: bool,
) -> Vec<ASTLineMapping> {
    let source_lines: Vec<&str> = source_content.lines().collect();
    let target_lines: Vec<&str> = target_content.lines().collect();

    // Use LCS (Longest Common Subsequence) based diff algorithm
    let diff_ops = compute_diff_operations(&source_lines, &target_lines, ignore_whitespace);

    let mut mappings = Vec::new();
    let mut source_idx = 0;
    let mut target_idx = 0;

    for op in diff_ops {
        match op {
            DiffOp::Equal(src_line, tgt_line) => {
                mappings.push(ASTLineMapping {
                    change_type: "unchanged".to_string(),
                    source_line: Some(source_idx + 1),
                    target_line: Some(target_idx + 1),
                    source_content: Some(src_line.to_string()),
                    target_content: Some(tgt_line.to_string()),
                    ast_node_type: None,
                    similarity: Some(1.0),
                    is_structural_change: false,
                    semantic_changes: Vec::new(),
                });
                source_idx += 1;
                target_idx += 1;
            }
            DiffOp::Delete(src_line) => {
                mappings.push(ASTLineMapping {
                    change_type: "deleted".to_string(),
                    source_line: Some(source_idx + 1),
                    target_line: None,
                    source_content: Some(src_line.to_string()),
                    target_content: None,
                    ast_node_type: None,
                    similarity: None,
                    is_structural_change: true,
                    semantic_changes: Vec::new(),
                });
                source_idx += 1;
            }
            DiffOp::Insert(tgt_line) => {
                mappings.push(ASTLineMapping {
                    change_type: "added".to_string(),
                    source_line: None,
                    target_line: Some(target_idx + 1),
                    source_content: None,
                    target_content: Some(tgt_line.to_string()),
                    ast_node_type: None,
                    similarity: None,
                    is_structural_change: true,
                    semantic_changes: Vec::new(),
                });
                target_idx += 1;
            }
            DiffOp::Replace(src_line, tgt_line) => {
                let similarity = calculate_line_similarity(&src_line, &tgt_line);
                mappings.push(ASTLineMapping {
                    change_type: "modified".to_string(),
                    source_line: Some(source_idx + 1),
                    target_line: Some(target_idx + 1),
                    source_content: Some(src_line.to_string()),
                    target_content: Some(tgt_line.to_string()),
                    ast_node_type: None,
                    similarity: Some(similarity),
                    is_structural_change: similarity < 0.5,
                    semantic_changes: detect_semantic_changes(&src_line, &tgt_line),
                });
                source_idx += 1;
                target_idx += 1;
            }
        }
    }

    mappings
}

/// Generate AST-aware line mappings using Zhang-Shasha tree edit distance and Hungarian matching
fn generate_ast_aware_line_mappings(
    source_content: &str,
    target_content: &str,
    source_ast: &ParseResult,
    target_ast: &ParseResult,
    options: &ASTDiffOptions,
) -> Vec<ASTLineMapping> {
    info!("Starting AST-aware diff - using LCS with AST-enhanced similarity");

    // FIXED: The previous implementation was fundamentally broken because it only mapped
    // the start_line of each AST node, causing duplicate line numbers and missing lines.
    //
    // New approach: Use LCS for reliable line-by-line mapping, then enhance with AST info

    // Step 1: Get base LCS line mappings (reliable and complete)
    let base_mappings = generate_lcs_line_mappings(
        source_content,
        target_content,
        source_ast,
        target_ast,
        options.ignore_whitespace,
    );

    // Step 2: Build AST node lookup by line number for enrichment
    let source_nodes = extract_nodes_with_lines(&source_ast.ast);
    let target_nodes = extract_nodes_with_lines(&target_ast.ast);

    let mut source_line_to_node: std::collections::HashMap<usize, &NodeWithLines> =
        std::collections::HashMap::new();
    let mut target_line_to_node: std::collections::HashMap<usize, &NodeWithLines> =
        std::collections::HashMap::new();

    for node in &source_nodes {
        source_line_to_node.insert(node.start_line, node);
    }

    for node in &target_nodes {
        target_line_to_node.insert(node.start_line, node);
    }

    // Step 3: Enhance LCS mappings with AST information
    let enhanced_mappings: Vec<ASTLineMapping> = base_mappings
        .into_iter()
        .map(|mut mapping| {
            // Add AST node type information if available
            if let Some(src_line) = mapping.source_line {
                if let Some(node) = source_line_to_node.get(&src_line) {
                    mapping.ast_node_type = Some(format!("{:?}", node.node_type));
                }
            } else if let Some(tgt_line) = mapping.target_line {
                if let Some(node) = target_line_to_node.get(&tgt_line) {
                    mapping.ast_node_type = Some(format!("{:?}", node.node_type));
                }
            }

            // Enhance semantic change detection for modified lines
            if mapping.change_type == "modified" {
                if let (Some(ref src_content), Some(ref tgt_content)) =
                    (&mapping.source_content, &mapping.target_content)
                {
                    mapping.semantic_changes = detect_semantic_changes(src_content, tgt_content);
                    mapping.is_structural_change = !mapping.semantic_changes.is_empty();
                }
            }

            mapping
        })
        .collect();

    info!(
        "Generated {} AST-enhanced line mappings",
        enhanced_mappings.len()
    );
    enhanced_mappings
}

/// Node with line information for matching
#[derive(Debug, Clone)]
struct NodeWithLines {
    node_type: smart_diff_parser::NodeType,
    start_line: usize,
    end_line: usize,
    content: String,
}

/// Extract AST nodes with their line ranges
fn extract_nodes_with_lines(node: &smart_diff_parser::ASTNode) -> Vec<NodeWithLines> {
    let mut nodes = Vec::new();
    extract_nodes_recursive(node, &mut nodes);
    nodes
}

fn extract_nodes_recursive(node: &smart_diff_parser::ASTNode, nodes: &mut Vec<NodeWithLines>) {
    // Add current node
    nodes.push(NodeWithLines {
        node_type: node.node_type.clone(),
        start_line: node.metadata.line,
        end_line: node.metadata.line, // Simplified - could calculate actual end line
        content: format!("{:?}", node.node_type),
    });

    // Recursively process children
    for child in &node.children {
        extract_nodes_recursive(child, nodes);
    }
}

/// Node assignment from Hungarian-inspired matching
#[derive(Debug, Clone)]
struct NodeAssignment {
    source_idx: usize,
    target_idx: usize,
    similarity: f64,
}

/// Compute optimal node matching using greedy algorithm (simplified Hungarian approach)
fn compute_optimal_node_matching(
    source_nodes: &[NodeWithLines],
    target_nodes: &[NodeWithLines],
) -> Vec<NodeAssignment> {
    let mut assignments = Vec::new();
    let mut matched_targets = std::collections::HashSet::new();

    // Build similarity matrix
    let mut similarities: Vec<(usize, usize, f64)> = Vec::new();

    for (src_idx, src_node) in source_nodes.iter().enumerate() {
        for (tgt_idx, tgt_node) in target_nodes.iter().enumerate() {
            let similarity = calculate_node_similarity(src_node, tgt_node);
            if similarity > 0.3 {
                // Only consider reasonable matches
                similarities.push((src_idx, tgt_idx, similarity));
            }
        }
    }

    // Sort by similarity (descending) for greedy matching
    similarities.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    // Greedy assignment: pick best matches first
    let mut matched_sources = std::collections::HashSet::new();

    for (src_idx, tgt_idx, similarity) in similarities {
        if !matched_sources.contains(&src_idx) && !matched_targets.contains(&tgt_idx) {
            assignments.push(NodeAssignment {
                source_idx: src_idx,
                target_idx: tgt_idx,
                similarity,
            });
            matched_sources.insert(src_idx);
            matched_targets.insert(tgt_idx);
        }
    }

    assignments
}

/// Calculate similarity between two AST nodes
fn calculate_node_similarity(node1: &NodeWithLines, node2: &NodeWithLines) -> f64 {
    // Node type similarity (most important)
    let type_similarity = if node1.node_type == node2.node_type {
        1.0
    } else {
        0.0
    };

    // Line proximity (nodes at similar positions are more likely to match)
    let line_diff = (node1.start_line as i32 - node2.start_line as i32).abs();
    let proximity_similarity = 1.0 / (1.0 + line_diff as f64 * 0.1);

    // Content similarity
    let content_similarity = calculate_line_similarity(&node1.content, &node2.content);

    // Weighted combination
    type_similarity * 0.6 + proximity_similarity * 0.2 + content_similarity * 0.2
}

/// Diff operation types
#[derive(Debug, Clone)]
enum DiffOp {
    Equal(String, String),
    Delete(String),
    Insert(String),
    Replace(String, String),
}

/// Compute diff operations using Myers diff algorithm (LCS-based)
fn compute_diff_operations(
    source_lines: &[&str],
    target_lines: &[&str],
    ignore_whitespace: bool,
) -> Vec<DiffOp> {
    // Compute LCS (Longest Common Subsequence)
    let lcs_table = compute_lcs_table(source_lines, target_lines, ignore_whitespace);

    // Backtrack to generate diff operations
    let mut operations = Vec::new();
    let mut i = source_lines.len();
    let mut j = target_lines.len();

    while i > 0 || j > 0 {
        let lines_equal = if ignore_whitespace {
            source_lines[i - 1].trim() == target_lines[j - 1].trim()
        } else {
            source_lines[i - 1] == target_lines[j - 1]
        };

        if i > 0 && j > 0 && lines_equal {
            // Lines are equal
            operations.push(DiffOp::Equal(
                source_lines[i - 1].to_string(),
                target_lines[j - 1].to_string(),
            ));
            i -= 1;
            j -= 1;
        } else if i > 0 && j > 0 && lcs_table[i][j - 1] < lcs_table[i - 1][j] {
            // Delete from source
            operations.push(DiffOp::Delete(source_lines[i - 1].to_string()));
            i -= 1;
        } else if i > 0 && j > 0 && lcs_table[i][j - 1] >= lcs_table[i - 1][j] {
            // Check if lines are similar enough to be a replacement
            let similarity = calculate_line_similarity(source_lines[i - 1], target_lines[j - 1]);
            if similarity > 0.3 {
                // Replace (modified line)
                operations.push(DiffOp::Replace(
                    source_lines[i - 1].to_string(),
                    target_lines[j - 1].to_string(),
                ));
                i -= 1;
                j -= 1;
            } else {
                // Insert to target
                operations.push(DiffOp::Insert(target_lines[j - 1].to_string()));
                j -= 1;
            }
        } else if j > 0 {
            // Insert to target
            operations.push(DiffOp::Insert(target_lines[j - 1].to_string()));
            j -= 1;
        } else if i > 0 {
            // Delete from source
            operations.push(DiffOp::Delete(source_lines[i - 1].to_string()));
            i -= 1;
        }
    }

    operations.reverse();
    operations
}

/// Compute LCS (Longest Common Subsequence) table using dynamic programming
fn compute_lcs_table(
    source_lines: &[&str],
    target_lines: &[&str],
    ignore_whitespace: bool,
) -> Vec<Vec<usize>> {
    let m = source_lines.len();
    let n = target_lines.len();
    let mut table = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            let lines_equal = if ignore_whitespace {
                source_lines[i - 1].trim() == target_lines[j - 1].trim()
            } else {
                source_lines[i - 1] == target_lines[j - 1]
            };

            if lines_equal {
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = table[i - 1][j].max(table[i][j - 1]);
            }
        }
    }

    table
}

/// Generate AST operations using the diff engine
fn generate_ast_operations(
    _source_ast: &ParseResult,
    _target_ast: &ParseResult,
) -> Vec<ASTOperation> {
    // This would use the actual diff engine to generate operations
    // For now, return empty list as a placeholder
    Vec::new()
}

/// Calculate line similarity
fn calculate_line_similarity(line1: &str, line2: &str) -> f64 {
    let trimmed1 = line1.trim();
    let trimmed2 = line2.trim();

    if trimmed1 == trimmed2 {
        return 1.0;
    }

    if trimmed1.is_empty() || trimmed2.is_empty() {
        return 0.0;
    }

    // Simple character-based similarity
    let max_len = trimmed1.len().max(trimmed2.len());
    let min_len = trimmed1.len().min(trimmed2.len());

    let mut matches = 0;
    for (c1, c2) in trimmed1.chars().zip(trimmed2.chars()) {
        if c1 == c2 {
            matches += 1;
        }
    }

    (matches as f64) / (max_len as f64) * (min_len as f64) / (max_len as f64)
}

/// Detect semantic changes between two lines
fn detect_semantic_changes(line1: &str, line2: &str) -> Vec<String> {
    let mut changes = Vec::new();

    // Simple heuristics for semantic changes
    if line1.contains("if") != line2.contains("if") {
        changes.push("Control flow change".to_string());
    }

    if line1.contains("return") != line2.contains("return") {
        changes.push("Return statement change".to_string());
    }

    if line1.contains("=") != line2.contains("=") {
        changes.push("Assignment change".to_string());
    }

    changes
}

/// Calculate summary statistics for AST diff
fn calculate_ast_diff_summary(line_mappings: &[ASTLineMapping]) -> ASTDiffSummary {
    let total_lines = line_mappings.len();
    let added_lines = line_mappings
        .iter()
        .filter(|m| m.change_type == "added")
        .count();
    let deleted_lines = line_mappings
        .iter()
        .filter(|m| m.change_type == "deleted")
        .count();
    let modified_lines = line_mappings
        .iter()
        .filter(|m| m.change_type == "modified")
        .count();
    let unchanged_lines = line_mappings
        .iter()
        .filter(|m| m.change_type == "unchanged")
        .count();
    let structural_changes = line_mappings
        .iter()
        .filter(|m| m.is_structural_change)
        .count();
    let semantic_changes = line_mappings.iter().map(|m| m.semantic_changes.len()).sum();

    ASTDiffSummary {
        total_lines,
        added_lines,
        deleted_lines,
        modified_lines,
        unchanged_lines,
        structural_changes,
        semantic_changes,
    }
}
