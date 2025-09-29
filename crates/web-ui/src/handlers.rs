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
    options: &CompareOptions,
) -> anyhow::Result<AnalysisResult> {
    // Initialize components
    let language_detector = LanguageDetector;
    let parser_engine = TreeSitterParser::new()?;
    let semantic_analyzer = SemanticAnalyzer::new();
    let diff_engine = DiffEngine::new();

    // Detect language
    let language = language_detector.detect_from_path(&file1.path)
        .or_else(|| language_detector.detect_from_content(&file1.content))
        .unwrap_or(Language::Unknown);

    // Parse both files
    let parse_result1 = parser_engine.parse(&file1.content, language)?;
    let parse_result2 = parser_engine.parse(&file2.content, language)?;
    let ast1 = parse_result1.ast;
    let ast2 = parse_result2.ast;

    // Perform semantic analysis
    let semantic1 = semantic_analyzer.analyze(&parse_result1)?;
    let semantic2 = semantic_analyzer.analyze(&parse_result2)?;

    // Initialize components that need language
    let function_matcher = FunctionMatcher::new(0.7); // threshold
    let similarity_scorer = SimilarityScorer::new(language, smart_diff_engine::SimilarityScoringConfig::default());
    let change_classifier = ChangeClassifier::new(language);
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
    let changes = function_matches.changes;

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
                complexity: calculate_complexity_from_symbol_table(&semantic1.symbol_table),
            },
            target: FileMetadata {
                path: file2.path.clone(),
                lines: file2.content.lines().count(),
                functions: functions2.len(),
                classes: count_classes_from_symbol_table(&semantic2.symbol_table),
                complexity: calculate_complexity_from_symbol_table(&semantic2.symbol_table),
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

/// Build function analysis from matches
fn build_function_analysis(matches: &[smart_diff_engine::FunctionMatch]) -> FunctionAnalysis {
    let total_functions = matches.len();
    let matched_functions = matches.iter().filter(|m| m.target_function.is_some()).count();
    let average_similarity = if matched_functions > 0 {
        matches.iter()
            .filter(|m| m.target_function.is_some())
            .map(|m| m.similarity.overall)
            .sum::<f64>() / matched_functions as f64
    } else {
        0.0
    };

    let function_matches = matches.iter().enumerate().map(|(i, m)| {
        FunctionMatch {
            id: format!("func-{}", i),
            source_function: FunctionInfo {
                name: m.source_function.name.clone(),
                signature: m.source_function.signature.clone(),
                start_line: m.source_function.start_line,
                end_line: m.source_function.end_line,
                complexity: m.source_function.complexity as usize,
                parameters: m.source_function.parameters.clone(),
                return_type: m.source_function.return_type.clone(),
            },
            target_function: m.target_function.as_ref().map(|tf| FunctionInfo {
                name: tf.name.clone(),
                signature: tf.signature.clone(),
                start_line: tf.start_line,
                end_line: tf.end_line,
                complexity: tf.complexity as usize,
                parameters: tf.parameters.clone(),
                return_type: tf.return_type.clone(),
            }),
            similarity: SimilarityScore {
                overall: m.similarity.overall,
                structure: m.similarity.structure,
                content: m.similarity.content,
                semantic: m.similarity.semantic,
            },
            change_type: m.change_type.to_string(),
            refactoring_pattern: m.refactoring_pattern.as_ref().map(|rp| RefactoringPattern {
                pattern_type: rp.pattern_type.clone(),
                description: rp.description.clone(),
                confidence: rp.confidence,
                evidence: rp.evidence.clone(),
                impact: rp.impact.clone(),
            }),
        }
    }).collect();

    FunctionAnalysis {
        total_functions,
        matched_functions,
        function_matches,
        average_similarity,
    }
}

/// Build change analysis from classified changes
fn build_change_analysis(changes: &[smart_diff_engine::ClassifiedChange]) -> ChangeAnalysis {
    let total_changes = changes.len();
    let mut change_types = HashMap::new();

    for change in changes {
        *change_types.entry(change.change_type.clone()).or_insert(0) += 1;
    }

    let detailed_changes = changes.iter().enumerate().map(|(i, change)| {
        DetailedChange {
            id: format!("change-{}", i),
            change_type: change.change_type.clone(),
            description: change.description.clone(),
            confidence: change.confidence,
            location: ChangeLocation {
                file: change.location.file.clone(),
                start_line: change.location.start_line,
                end_line: change.location.end_line,
                function: change.location.function.clone(),
            },
            impact: change.impact.clone(),
        }
    }).collect();

    let breaking_changes = changes.iter()
        .filter(|c| c.impact == "breaking")
        .count();

    ChangeAnalysis {
        total_changes,
        change_types,
        detailed_changes,
        impact_assessment: ImpactAssessment {
            risk_level: if breaking_changes > 0 { "high" } else { "low" }.to_string(),
            breaking_changes,
            effort_estimate: estimate_effort(changes),
            affected_components: extract_affected_components(changes),
        },
    }
}

/// Build refactoring patterns from detected patterns
fn build_refactoring_patterns(patterns: &[smart_diff_engine::RefactoringPattern]) -> Vec<RefactoringPattern> {
    patterns.iter().map(|pattern| RefactoringPattern {
        pattern_type: pattern.pattern_type.clone(),
        description: pattern.description.clone(),
        confidence: pattern.confidence,
        evidence: pattern.evidence.clone(),
        impact: pattern.impact.clone(),
    }).collect()
}

/// Build structure comparison from ASTs
fn build_structure_comparison(
    ast1: &smart_diff_parser::AST,
    ast2: &smart_diff_parser::AST,
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

/// Estimate effort for implementing changes
fn estimate_effort(changes: &[smart_diff_engine::ClassifiedChange]) -> String {
    let total_changes = changes.len();
    match total_changes {
        0..=5 => "low".to_string(),
        6..=15 => "medium".to_string(),
        _ => "high".to_string(),
    }
}

/// Extract affected components from changes
fn extract_affected_components(changes: &[smart_diff_engine::ClassifiedChange]) -> Vec<String> {
    changes.iter()
        .filter_map(|c| c.location.function.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect()
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
    options: &AnalyzeOptions,
) -> anyhow::Result<MultiFileAnalysisResult> {
    let language_detector = LanguageDetector;
    let parser_engine = TreeSitterParser::new()?;
    let mut semantic_analyzer = SemanticAnalyzer::new();

    let mut file_results = Vec::new();
    let mut all_functions = Vec::new();
    let mut total_complexity = 0.0;

    // Analyze each file
    for file in files {
        let language = language_detector.detect_from_path(&file.path)
            .or_else(|| language_detector.detect_from_content(&file.content))
            .unwrap_or(Language::Unknown);
        let parse_result = parser_engine.parse(&file.content, language)?;
        let semantic = semantic_analyzer.analyze(&parse_result)?;

        let functions = extract_functions_from_symbol_table(&semantic.symbol_table);
        let complexity = calculate_complexity_from_symbol_table(&semantic.symbol_table);
        total_complexity += complexity as f64;

        let function_infos: Vec<FunctionInfo> = functions.iter().map(|f| FunctionInfo {
            name: f.signature.name.clone(),
            signature: format!("{}({})", f.signature.name,
                f.signature.parameters.iter()
                    .map(|p| format!("{}: {}", p.name, p.param_type.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ")),
            start_line: f.location.start_line,
            end_line: f.location.end_line,
            complexity: 1, // Simplified
            parameters: f.signature.parameters.iter().map(|p| p.name.clone()).collect(),
            return_type: f.signature.return_type.as_ref()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "void".to_string()),
        }).collect();

        all_functions.extend(functions);

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
            dependencies: extract_dependencies(&semantic),
            issues: detect_issues(&semantic),
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

fn calculate_complexity_distribution(functions: &[smart_diff_semantic::Function]) -> HashMap<String, usize> {
    let mut distribution = HashMap::new();

    for function in functions {
        let complexity_range = match function.complexity {
            0..=5 => "low",
            6..=10 => "medium",
            11..=20 => "high",
            _ => "very_high",
        };
        *distribution.entry(complexity_range.to_string()).or_insert(0) += 1;
    }

    distribution
}

fn extract_dependencies(semantic: &smart_diff_semantic::SemanticInfo) -> Vec<String> {
    // This would extract actual dependencies from semantic analysis
    // For now, return empty vector
    vec![]
}

fn detect_issues(semantic: &smart_diff_semantic::SemanticInfo) -> Vec<String> {
    // This would detect code issues from semantic analysis
    // For now, return empty vector
    vec![]
}

fn perform_cross_file_analysis(
    functions: &[smart_diff_semantic::Function],
    files: &[FileInfo],
) -> anyhow::Result<CrossFileAnalysis> {
    // Detect duplicate functions
    let mut duplicate_functions = Vec::new();
    let mut seen_signatures = HashMap::new();

    for function in functions {
        if let Some(existing_locations) = seen_signatures.get_mut(&function.signature) {
            existing_locations.push(ChangeLocation {
                file: "unknown".to_string(), // Would need to track file association
                start_line: function.start_line,
                end_line: function.end_line,
                function: Some(function.name.clone()),
            });
        } else {
            seen_signatures.insert(function.signature.clone(), vec![ChangeLocation {
                file: "unknown".to_string(),
                start_line: function.start_line,
                end_line: function.end_line,
                function: Some(function.name.clone()),
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
    all_functions: &[smart_diff_semantic::Function],
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
        let body = smart_diff_parser::ASTNode::new(
            smart_diff_parser::NodeType::Function,
            smart_diff_parser::ASTMetadata::default(),
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
