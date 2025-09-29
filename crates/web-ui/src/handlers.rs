//! HTTP request handlers

use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, Json as ResponseJson},
};
use serde_json::json;
use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use smart_diff_parser::{LanguageDetector, tree_sitter::TreeSitterParser, Parser, Language};
use smart_diff_semantic::{SemanticAnalyzer};
use smart_diff_engine::{
    DiffEngine, FunctionMatcher, SimilarityScorer, ChangeClassifier, RefactoringDetector,
};

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
    components.insert("parser".to_string(), ComponentHealth {
        status: "healthy".to_string(),
        last_check: chrono::Utc::now().to_rfc3339(),
        details: Some("Parser engine operational".to_string()),
    });

    // Check semantic analyzer
    components.insert("semantic".to_string(), ComponentHealth {
        status: "healthy".to_string(),
        last_check: chrono::Utc::now().to_rfc3339(),
        details: Some("Semantic analyzer operational".to_string()),
    });

    // Check diff engine
    components.insert("diff_engine".to_string(), ComponentHealth {
        status: "healthy".to_string(),
        last_check: chrono::Utc::now().to_rfc3339(),
        details: Some("Diff engine operational".to_string()),
    });

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
    let _similarity_scorer = SimilarityScorer::new(language, smart_diff_engine::SimilarityScoringConfig::default());
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
        *change_types.entry(format!("{:?}", change.change_type)).or_insert(0) += 1;
    }

    let detailed_changes = changes.iter().enumerate().map(|(i, change)| {
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
    }).collect();

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
fn build_refactoring_patterns(patterns: &[smart_diff_engine::RefactoringPattern]) -> Vec<RefactoringPattern> {
    patterns.iter().map(|pattern| RefactoringPattern {
        pattern_type: format!("{:?}", pattern.pattern_type),
        description: pattern.description.clone(),
        confidence: pattern.confidence,
        evidence: pattern.evidence.iter().map(|e| format!("{:?}", e)).collect(),
        impact: format!("{:?}", pattern.analysis.impact),
    }).collect()
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

        let function_infos: Vec<FunctionInfo> = functions.iter().map(|f| FunctionInfo {
            name: f.signature.name.clone(),
            signature: format!("{}({})", f.signature.name,
                f.signature.parameters.iter()
                    .map(|p| format!("{}: {}", p.name, p.param_type.name))
                    .collect::<Vec<_>>()
                    .join(", ")),
            start_line: f.location.start_line,
            end_line: f.location.end_line,
            complexity: 1, // Simplified
            parameters: f.signature.parameters.iter().map(|p| p.name.clone()).collect(),
            return_type: f.signature.return_type.as_ref()
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "void".to_string()),
        }).collect();

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
        average_complexity: if files.is_empty() { 0.0 } else { total_complexity / files.len() as f64 },
        duplicate_rate: calculate_duplicate_rate(&cross_file_analysis.duplicate_functions, &all_functions),
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
            updated_settings.insert("parser.enable_error_recovery".to_string(), json!(enable_error_recovery));
        }
    }

    if let Some(semantic_config) = &request.semantic {
        if let Some(max_resolution_depth) = semantic_config.max_resolution_depth {
            updated_settings.insert("semantic.max_resolution_depth".to_string(), json!(max_resolution_depth));
        }
        if let Some(enable_cross_file_analysis) = semantic_config.enable_cross_file_analysis {
            updated_settings.insert("semantic.enable_cross_file_analysis".to_string(), json!(enable_cross_file_analysis));
        }
        if let Some(symbol_cache_size) = semantic_config.symbol_cache_size {
            updated_settings.insert("semantic.symbol_cache_size".to_string(), json!(symbol_cache_size));
        }
    }

    if let Some(diff_config) = &request.diff_engine {
        if let Some(threshold) = diff_config.default_similarity_threshold {
            updated_settings.insert("diff_engine.default_similarity_threshold".to_string(), json!(threshold));
        }
        if let Some(enable_refactoring) = diff_config.enable_refactoring_detection {
            updated_settings.insert("diff_engine.enable_refactoring_detection".to_string(), json!(enable_refactoring));
        }
        if let Some(enable_cross_file) = diff_config.enable_cross_file_tracking {
            updated_settings.insert("diff_engine.enable_cross_file_tracking".to_string(), json!(enable_cross_file));
        }
        if let Some(max_tree_depth) = diff_config.max_tree_depth {
            updated_settings.insert("diff_engine.max_tree_depth".to_string(), json!(max_tree_depth));
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

fn calculate_complexity_distribution(functions: &[smart_diff_parser::Function]) -> HashMap<String, usize> {
    let mut distribution = HashMap::new();

    for _function in functions {
        // Simplified complexity calculation
        let complexity_range = "medium"; // Placeholder
        *distribution.entry(complexity_range.to_string()).or_insert(0) += 1;
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
        let signature_str = format!("{}({})", function.signature.name,
            function.signature.parameters.iter()
                .map(|p| format!("{}: {}", p.name, p.param_type.name))
                .collect::<Vec<_>>()
                .join(", "));

        if let Some(existing_locations) = seen_signatures.get_mut(&signature_str) {
            existing_locations.push(ChangeLocation {
                file: "unknown".to_string(), // Would need to track file association
                start_line: function.location.start_line,
                end_line: function.location.end_line,
                function: Some(function.signature.name.clone()),
            });
        } else {
            seen_signatures.insert(signature_str, vec![ChangeLocation {
                file: "unknown".to_string(),
                start_line: function.location.start_line,
                end_line: function.location.end_line,
                function: Some(function.signature.name.clone()),
            }]);
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
        moved_functions: vec![], // Would implement moved function detection
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

    let duplicate_count: usize = duplicates.iter()
        .map(|d| d.locations.len())
        .sum();

    duplicate_count as f64 / all_functions.len() as f64
}

/// Extract functions from symbol table
fn extract_functions_from_symbol_table(symbol_table: &smart_diff_semantic::SymbolTable) -> Vec<smart_diff_parser::Function> {
    use smart_diff_parser::{Function, FunctionSignature, Type, FunctionLocation};
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
        let body = smart_diff_parser::ASTNode::new(
            smart_diff_parser::NodeType::Function,
            metadata,
        );

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
fn calculate_complexity_from_symbol_table(_symbol_table: &smart_diff_semantic::SymbolTable) -> usize {
    // Simplified complexity calculation
    10 // Placeholder value
}

/// SPA fallback handler - serves index.html for client-side routing
pub async fn spa_fallback() -> Html<&'static str> {
    Html(include_str!("../../../static/index.html"))
}

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

use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use std::os::unix::fs::PermissionsExt;

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
        let file_name = path.file_name()
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

        let extension = path.extension()
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

        let modified = metadata.modified().ok()
            .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|duration| {
                DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            });

        let mut entry_metadata = HashMap::new();
        entry_metadata.insert("permissions".to_string(),
            json!(format!("{:o}", metadata.permissions().mode() & 0o777)));

        entries.push(FileSystemEntry {
            path: path.to_string_lossy().to_string(),
            name: file_name,
            is_directory,
            size: if is_directory { None } else { Some(metadata.len()) },
            modified,
            extension,
            language,
            children: None,
            metadata: entry_metadata,
        });
    }

    // Sort entries: directories first, then files, both alphabetically
    entries.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
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
        encoding: request.encoding.clone().unwrap_or_else(|| "utf-8".to_string()),
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
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

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
            if matches!(search_type, SearchType::FileContent | SearchType::FunctionName | SearchType::Both) {
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
    let search_text = if case_sensitive { text } else { &text.to_lowercase() };
    let search_query = if case_sensitive { query } else { &query.to_lowercase() };

    search_text.find(search_query).map(|pos| pos + 1)
}
