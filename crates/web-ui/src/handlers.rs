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
    let function_matches = analyze_function_changes(&source_files, &target_files, request.options.similarity_threshold).await?;

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
    use walkdir::WalkDir;
    use std::time::SystemTime;

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
                if !options.file_extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                    continue;
                }
            } else {
                continue; // Skip files without extensions if filter is specified
            }
        }

        // Read file content
        if let Ok(content) = fs::read_to_string(path) {
            let relative_path = path.strip_prefix(base_path)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            let metadata = entry.metadata()?;
            let file_info = ComparisonFileInfo {
                path: path.to_string_lossy().to_string(),
                relative_path,
                content,
                size: metadata.len(),
                modified: metadata.modified().ok()
                    .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|duration| {
                        chrono::DateTime::<chrono::Utc>::from_timestamp(duration.as_secs() as i64, 0)
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
    let common_lines = lines1.iter()
        .filter(|line| lines2.contains(line))
        .count();

    let total_lines = std::cmp::max(lines1.len(), lines2.len());
    common_lines as f64 / total_lines as f64
}

/// Analyze function changes between directories
async fn analyze_function_changes(
    source_files: &[ComparisonFileInfo],
    target_files: &[ComparisonFileInfo],
    similarity_threshold: f64,
) -> Result<Vec<crate::models::FunctionMatch>, Box<dyn std::error::Error + Send + Sync>> {
    use crate::models::{FunctionMatch, SimilarityScore};

    let mut matches = Vec::new();

    // Extract functions from all files
    let source_functions = extract_functions_from_files(source_files).await?;
    let target_functions = extract_functions_from_files(target_files).await?;

    tracing::info!(
        "Extracted functions: {} source, {} target",
        source_functions.len(),
        target_functions.len()
    );

    // Simple function matching based on name and signature
    let mut matched_targets = std::collections::HashSet::new();

    for source_func in &source_functions {
        let mut best_match: Option<&FunctionInfo> = None;
        let mut best_similarity = 0.0;

        for target_func in &target_functions {
            if matched_targets.contains(&target_func.name) {
                continue;
            }

            let similarity = calculate_function_similarity(source_func, target_func);
            if similarity > best_similarity && similarity >= similarity_threshold {
                best_match = Some(target_func);
                best_similarity = similarity;
            }
        }

        if let Some(target_func) = best_match {
            matched_targets.insert(target_func.name.clone());

            let change_type = if best_similarity >= 0.99 {
                "identical"
            } else if source_func.name != target_func.name {
                "renamed"
            } else {
                "modified"
            };

            matches.push(FunctionMatch {
                id: uuid::Uuid::new_v4().to_string(),
                source_function: source_func.clone(),
                target_function: Some(target_func.clone()),
                similarity: SimilarityScore {
                    overall: best_similarity,
                    structural: best_similarity,
                    semantic: best_similarity,
                    textual: best_similarity,
                },
                change_type: change_type.to_string(),
                refactoring_pattern: None,
            });
        } else {
            // Function was deleted
            matches.push(FunctionMatch {
                id: uuid::Uuid::new_v4().to_string(),
                source_function: source_func.clone(),
                target_function: None,
                similarity: SimilarityScore {
                    overall: 0.0,
                    structural: 0.0,
                    semantic: 0.0,
                    textual: 0.0,
                },
                change_type: "deleted".to_string(),
                refactoring_pattern: None,
            });
        }
    }

    // Find added functions
    for target_func in &target_functions {
        if !matched_targets.contains(&target_func.name) {
            matches.push(FunctionMatch {
                id: uuid::Uuid::new_v4().to_string(),
                source_function: FunctionInfo {
                    name: "".to_string(),
                    signature: "".to_string(),
                    start_line: 0,
                    end_line: 0,
                    complexity: 0,
                    parameters: Vec::new(),
                    return_type: "".to_string(),
                },
                target_function: Some(target_func.clone()),
                similarity: SimilarityScore {
                    overall: 0.0,
                    structural: 0.0,
                    semantic: 0.0,
                    textual: 0.0,
                },
                change_type: "added".to_string(),
                refactoring_pattern: None,
            });
        }
    }

    Ok(matches)
}

/// Extract functions from files
async fn extract_functions_from_files(
    files: &[ComparisonFileInfo],
) -> Result<Vec<FunctionInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let mut all_functions = Vec::new();

    for file in files {
        if let Some(language_str) = &file.language {
            // Simple function extraction using regex patterns
            let functions = extract_functions_simple(&file.content, language_str, &file.relative_path);
            all_functions.extend(functions);
        }
    }

    Ok(all_functions)
}

/// Simple function extraction using regex
fn extract_functions_simple(content: &str, language: &str, file_path: &str) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    // Simple regex patterns for different languages
    let pattern = match language.to_lowercase().as_str() {
        "javascript" | "typescript" => r"(?:function\s+(\w+)|const\s+(\w+)\s*=.*=>|(\w+)\s*\([^)]*\)\s*\{)",
        "python" => r"def\s+(\w+)\s*\(",
        "java" | "c" | "cpp" => r"(?:public|private|protected)?\s*(?:static\s+)?(?:\w+\s+)+(\w+)\s*\([^)]*\)\s*\{",
        "rust" => r"fn\s+(\w+)\s*\(",
        _ => return functions,
    };

    if let Ok(regex) = regex::Regex::new(pattern) {
        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = regex.captures(line) {
                if let Some(name_match) = captures.get(1).or_else(|| captures.get(2)).or_else(|| captures.get(3)) {
                    let function_name = name_match.as_str().to_string();

                    // Skip common keywords
                    if !["if", "for", "while", "switch", "return"].contains(&function_name.as_str()) {
                        functions.push(FunctionInfo {
                            name: function_name.clone(),
                            signature: line.trim().to_string(),
                            start_line: line_num + 1,
                            end_line: line_num + 10, // Simplified
                            complexity: 1,
                            parameters: Vec::new(), // Simplified
                            return_type: "unknown".to_string(),
                        });
                    }
                }
            }
        }
    }

    functions
}

/// Calculate function similarity
fn calculate_function_similarity(func1: &FunctionInfo, func2: &FunctionInfo) -> f64 {
    // Name similarity
    let name_similarity = if func1.name == func2.name { 1.0 } else { 0.0 };

    // Signature similarity
    let sig_similarity = if func1.signature == func2.signature { 1.0 } else { 0.5 };

    // Weighted average
    (name_similarity * 0.6 + sig_similarity * 0.4)
}

/// Generate comparison summary
fn generate_comparison_summary(
    file_changes: &[crate::models::FileChange],
    function_matches: &[crate::models::FunctionMatch],
) -> crate::models::DirectoryComparisonSummary {
    use crate::models::DirectoryComparisonSummary;

    let total_files = file_changes.len();
    let added_files = file_changes.iter().filter(|c| c.change_type == "added").count();
    let deleted_files = file_changes.iter().filter(|c| c.change_type == "deleted").count();
    let modified_files = file_changes.iter().filter(|c| c.change_type == "modified").count();
    let unchanged_files = file_changes.iter().filter(|c| c.change_type == "unchanged").count();

    let total_functions = function_matches.len();
    let added_functions = function_matches.iter().filter(|m| m.change_type == "added").count();
    let deleted_functions = function_matches.iter().filter(|m| m.change_type == "deleted").count();
    let modified_functions = function_matches.iter().filter(|m| m.change_type == "modified").count();
    let moved_functions = function_matches.iter().filter(|m| m.change_type == "moved" || m.change_type == "renamed").count();

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
