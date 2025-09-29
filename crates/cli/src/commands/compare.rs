//! Compare command implementation

use crate::cli::{Cli, Commands, OutputFormat};
use crate::output::{OutputFormatter, ComparisonResult, ComparisonStats};
use anyhow::{Result, Context, bail};
use colored::*;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};
use smart_diff_parser::{tree_sitter::TreeSitterParser, Parser, LanguageDetector, Language};
use smart_diff_semantic::{SemanticAnalyzer, SymbolTable};
use smart_diff_engine::{
    DiffEngine, RefactoringDetector, RefactoringDetectionConfig,
    SimilarityScorer, ChangeClassifier, CrossFileTracker
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Instant, Duration};
use tokio::fs as async_fs;
use tracing::{info, warn, error, debug};

pub async fn run(cli: Cli) -> Result<()> {
    if let Commands::Compare {
        source,
        target,
        format,
        recursive,
        ignore_whitespace,
        ignore_case,
        threshold,
        output,
        ref language,
        detect_refactoring,
        track_moves,
        show_similarity,
        include_ast,
        max_depth,
        show_stats,
        ref include,
        ref exclude,
    } = cli.command
    {
        let start_time = Instant::now();
        let term = Term::stdout();

        if !cli.quiet {
            println!("{}", "Smart Code Diff - Structural Code Comparison".bold().blue());
            println!("{}", "=".repeat(50).dimmed());
        }

        // Validate inputs
        validate_inputs(&source, &target, threshold)?;

        // Initialize progress tracking
        let progress = if !cli.quiet {
            let pb = ProgressBar::new(100);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>3}/{len:3} {msg}")
                    .unwrap()
                    .progress_chars("█▉▊▋▌▍▎▏  ")
            );
            Some(pb)
        } else {
            None
        };

        // Step 1: File discovery and filtering
        if let Some(ref pb) = progress {
            pb.set_message("Discovering files...");
            pb.set_position(10);
        }

        let file_pairs = discover_files(&source, &target, recursive, &include, &exclude).await
            .context("Failed to discover files for comparison")?;

        if file_pairs.is_empty() {
            bail!("No files found to compare. Check your input paths and filters.");
        }

        info!("Found {} file pairs to compare", file_pairs.len());

        // Step 2: Language detection and parser initialization
        if let Some(ref pb) = progress {
            pb.set_message("Detecting languages...");
            pb.set_position(20);
        }

        let language_detector = LanguageDetector;
        let mut parsers: HashMap<Language, TreeSitterParser> = HashMap::new();
        let mut comparison_results = Vec::new();
        let mut total_stats = ComparisonStats::default();

        // Step 3: Process each file pair
        let total_pairs = file_pairs.len();
        for (index, (source_file, target_file)) in file_pairs.iter().enumerate() {
            if let Some(ref pb) = progress {
                pb.set_message(format!("Processing {}/{}: {}",
                    index + 1, total_pairs,
                    source_file.file_name().unwrap_or_default().to_string_lossy()));
                pb.set_position(20 + (60 * index as u64) / total_pairs as u64);
            }

            let file_result = process_file_pair(
                source_file,
                target_file,
                &language,
                &language_detector,
                &mut parsers,
                threshold,
                ignore_whitespace,
                ignore_case,
                detect_refactoring,
                track_moves,
                show_similarity,
                include_ast,
                max_depth,
                &cli,
            ).await;

            match file_result {
                Ok(result) => {
                    total_stats.merge(&result.stats);
                    comparison_results.push(result);
                }
                Err(e) => {
                    warn!("Failed to process file pair {:?} -> {:?}: {}", source_file, target_file, e);
                    if !cli.quiet {
                        eprintln!("{} Failed to process {}: {}",
                            "Warning:".yellow().bold(),
                            source_file.display(),
                            e);
                    }
                }
            }
        }

        // Step 4: Generate output
        if let Some(ref pb) = progress {
            pb.set_message("Generating output...");
            pb.set_position(90);
        }

        let output_content = OutputFormatter::format_comparison_results(
            &comparison_results,
            &format,
            show_stats.then_some(&total_stats),
            cli.no_color,
        )?;

        // Step 5: Write output
        if let Some(ref pb) = progress {
            pb.set_message("Writing output...");
            pb.set_position(95);
        }

        write_output(&output_content, &output, &format).await
            .context("Failed to write output")?;

        if let Some(ref pb) = progress {
            pb.finish_with_message("Comparison complete!");
        }

        // Display summary
        let elapsed = start_time.elapsed();
        if !cli.quiet {
            display_summary(&comparison_results, &total_stats, elapsed, &term)?;
        }

        if show_stats {
            display_detailed_stats(&total_stats, &term)?;
        }

        Ok(())
    } else {
        unreachable!("Compare command should have been matched")
    }
}

/// Validate input parameters
fn validate_inputs(source: &Path, target: &Path, threshold: f64) -> Result<()> {
    if !source.exists() {
        bail!("Source path does not exist: {}", source.display());
    }

    if !target.exists() {
        bail!("Target path does not exist: {}", target.display());
    }

    if !(0.0..=1.0).contains(&threshold) {
        bail!("Threshold must be between 0.0 and 1.0, got: {}", threshold);
    }

    // Check if both are files or both are directories
    let source_is_dir = source.is_dir();
    let target_is_dir = target.is_dir();

    if source_is_dir != target_is_dir {
        bail!("Both paths must be either files or directories. Source is {}, target is {}",
            if source_is_dir { "directory" } else { "file" },
            if target_is_dir { "directory" } else { "file" }
        );
    }

    Ok(())
}

/// Discover files to compare based on input paths and filters
async fn discover_files(
    source: &Path,
    target: &Path,
    recursive: bool,
    include: &[String],
    exclude: &[String],
) -> Result<Vec<(PathBuf, PathBuf)>> {
    let mut file_pairs = Vec::new();

    if source.is_file() && target.is_file() {
        // Single file comparison
        file_pairs.push((source.to_path_buf(), target.to_path_buf()));
    } else if source.is_dir() && target.is_dir() {
        // Directory comparison
        file_pairs = discover_directory_files(source, target, recursive, include, exclude).await?;
    }

    Ok(file_pairs)
}

/// Discover files in directories for comparison
async fn discover_directory_files(
    source_dir: &Path,
    target_dir: &Path,
    recursive: bool,
    include: &[String],
    exclude: &[String],
) -> Result<Vec<(PathBuf, PathBuf)>> {
    let mut file_pairs = Vec::new();
    let mut source_files = HashMap::new();
    let mut target_files = HashMap::new();

    // Collect source files
    collect_files(source_dir, recursive, include, exclude, &mut source_files).await?;

    // Collect target files
    collect_files(target_dir, recursive, include, exclude, &mut target_files).await?;

    // Match files by relative path
    for (rel_path, source_file) in &source_files {
        if let Some(target_file) = target_files.get(rel_path) {
            file_pairs.push((source_file.clone(), target_file.clone()));
        } else {
            debug!("File only exists in source: {}", rel_path.display());
        }
    }

    // Report files only in target
    for (rel_path, _) in target_files {
        if !source_files.contains_key(&rel_path) {
            debug!("File only exists in target: {}", rel_path.display());
        }
    }

    Ok(file_pairs)
}

/// Collect files from a directory
async fn collect_files(
    dir: &Path,
    recursive: bool,
    include: &[String],
    exclude: &[String],
    files: &mut HashMap<PathBuf, PathBuf>,
) -> Result<()> {
    let mut entries = async_fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();

        // Skip hidden files and directories
        if file_name.starts_with('.') {
            continue;
        }

        if path.is_file() {
            // Apply include/exclude filters
            if should_include_file(&path, include, exclude) {
                let rel_path = path.strip_prefix(dir)
                    .context("Failed to create relative path")?
                    .to_path_buf();
                files.insert(rel_path, path);
            }
        } else if path.is_dir() && recursive {
            Box::pin(collect_files(&path, recursive, include, exclude, files)).await?;
        }
    }

    Ok(())
}

/// Check if file should be included based on filters
fn should_include_file(path: &Path, include: &[String], exclude: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    // Check exclude patterns first
    for pattern in exclude {
        if glob_match(pattern, &path_str) || glob_match(pattern, &file_name) {
            return false;
        }
    }

    // If no include patterns, include by default
    if include.is_empty() {
        return true;
    }

    // Check include patterns
    for pattern in include {
        if glob_match(pattern, &path_str) || glob_match(pattern, &file_name) {
            return true;
        }
    }

    false
}

/// Simple glob pattern matching
fn glob_match(pattern: &str, text: &str) -> bool {
    // Simple implementation - could be enhanced with proper glob library
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            text.starts_with(parts[0]) && text.ends_with(parts[1])
        } else {
            // More complex patterns - fallback to simple contains
            text.contains(&pattern.replace('*', ""))
        }
    } else {
        text == pattern || text.ends_with(pattern)
    }
}

/// Process a single file pair for comparison
async fn process_file_pair(
    source_file: &Path,
    target_file: &Path,
    language_override: &Option<crate::cli::Language>,
    language_detector: &LanguageDetector,
    parsers: &mut HashMap<Language, TreeSitterParser>,
    threshold: f64,
    ignore_whitespace: bool,
    ignore_case: bool,
    detect_refactoring: bool,
    track_moves: bool,
    show_similarity: bool,
    include_ast: bool,
    max_depth: usize,
    cli: &Cli,
) -> Result<ComparisonResult> {
    let file_start = Instant::now();

    // Read file contents
    let source_content = async_fs::read_to_string(source_file).await
        .with_context(|| format!("Failed to read source file: {}", source_file.display()))?;

    let target_content = async_fs::read_to_string(target_file).await
        .with_context(|| format!("Failed to read target file: {}", target_file.display()))?;

    // Detect language
    let detected_language = if let Some(lang_override) = language_override {
        lang_override.to_parser_language()
            .context("Invalid language override")?
    } else {
        let detected = LanguageDetector::detect_from_path(source_file);
        if detected != Language::Unknown {
            detected
        } else {
            LanguageDetector::detect_from_content(&source_content)
        }
    };

    debug!("Detected language: {:?} for file: {}", detected_language, source_file.display());

    // Get or create parser for this language
    let parser = parsers.entry(detected_language)
        .or_insert_with(|| TreeSitterParser::new().expect("Failed to create parser"));

    // Parse source and target files
    let source_ast = parser.parse(&source_content, detected_language)
        .with_context(|| format!("Failed to parse source file: {}", source_file.display()))?;

    let target_ast = parser.parse(&target_content, detected_language)
        .with_context(|| format!("Failed to parse target file: {}", target_file.display()))?;

    // Perform semantic analysis
    let mut semantic_analyzer = SemanticAnalyzer::new();

    let source_symbols = semantic_analyzer.analyze(&source_ast)
        .with_context(|| format!("Failed to analyze source file: {}", source_file.display()))?;

    let target_symbols = semantic_analyzer.analyze(&target_ast)
        .with_context(|| format!("Failed to analyze target file: {}", target_file.display()))?;

    // Initialize diff engine components
    let mut diff_engine = DiffEngine::new();

    // Configure similarity scorer
    let mut similarity_scorer = SimilarityScorer::new(detected_language, smart_diff_engine::SimilarityScoringConfig::default());
    if ignore_whitespace {
        // Configure to ignore whitespace - would need to add this to SimilarityScorer
        debug!("Ignoring whitespace in similarity calculation");
    }

    // Configure change classifier
    let change_classifier = ChangeClassifier::new(detected_language);

    // Configure refactoring detector if enabled
    let refactoring_detector = if detect_refactoring {
        let mut config = RefactoringDetectionConfig::default();
        config.min_confidence_threshold = threshold;
        Some(RefactoringDetector::with_config(detected_language, config))
    } else {
        None
    };

    // Configure cross-file tracker if enabled
    let cross_file_tracker = if track_moves {
        Some(CrossFileTracker::new(detected_language, smart_diff_engine::CrossFileTrackerConfig::default()))
    } else {
        None
    };

    // Perform comparison
    let comparison_start = Instant::now();

    // Extract functions from AST for comparison
    let source_functions = extract_functions_from_ast(&source_ast.ast);
    let target_functions = extract_functions_from_ast(&target_ast.ast);

    let diff_result = diff_engine.compare_functions(
        &source_functions,
        &target_functions,
    ).context("Failed to perform structural comparison")?;

    let comparison_time = comparison_start.elapsed();

    // Classify changes
    let mut classified_changes = Vec::new();
    for change in &diff_result.match_result.changes {
        let classification = change_classifier.classify_change(
            change.source.as_ref(),
            change.target.as_ref(),
        );
        // Convert to DetailedChangeClassification - simplified for now
        classified_changes.push(smart_diff_engine::DetailedChangeClassification {
            change_type: classification,
            confidence: change.confidence,
            analysis: smart_diff_engine::ChangeAnalysis {
                description: "Change detected".to_string(),
                alternatives: Vec::new(),
                complexity_score: 0.5,
                characteristics: Vec::new(),
                evidence: Vec::new(),
            },
            secondary_types: Vec::new(),
            similarity_metrics: None,
            impact: smart_diff_engine::ChangeImpact {
                impact_level: smart_diff_engine::ImpactLevel::Low,
                affected_components: Vec::new(),
                implementation_effort: smart_diff_engine::EffortLevel::Low,
                risk_level: smart_diff_engine::RiskLevel::Low,
                is_breaking_change: false,
            },
        });
    }

    // Detect refactoring patterns if enabled
    let refactoring_patterns = if let Some(ref detector) = refactoring_detector {
        detector.detect_patterns(&diff_result.match_result.changes)
    } else {
        Vec::new()
    };

    // Calculate similarity scores if requested
    let similarity_scores = if show_similarity {
        Some(calculate_function_similarities(
            &source_symbols.symbol_table,
            &target_symbols.symbol_table,
            &similarity_scorer,
        )?)
    } else {
        None
    };

    // Track cross-file moves if enabled
    let cross_file_moves = if let Some(_tracker) = cross_file_tracker {
        // Cross-file tracking would require multiple files - simplified for now
        Vec::new()
    } else {
        Vec::new()
    };

    // Build comparison result
    let stats = ComparisonStats {
        files_compared: 1,
        functions_compared: 0, // Would need to count functions from symbol table
        changes_detected: diff_result.match_result.changes.len(),
        refactoring_patterns: refactoring_patterns.len(),
        cross_file_moves: cross_file_moves.len(),
        parsing_time: file_start.elapsed() - comparison_time,
        comparison_time,
        total_time: file_start.elapsed(),
        source_lines: source_content.lines().count(),
        target_lines: target_content.lines().count(),
        similarity_score: diff_result.match_result.similarity,
    };

    let result = ComparisonResult {
        source_file: source_file.to_path_buf(),
        target_file: target_file.to_path_buf(),
        language: detected_language,
        diff_result,
        classified_changes,
        refactoring_patterns,
        similarity_scores,
        cross_file_moves,
        stats,
        source_ast: if include_ast { Some(source_ast.ast) } else { None },
        target_ast: if include_ast { Some(target_ast.ast) } else { None },
    };

    if cli.verbose {
        info!("Processed {} -> {} in {:?}",
            source_file.display(),
            target_file.display(),
            file_start.elapsed());
    }

    Ok(result)
}

/// Calculate function-level similarity scores
fn calculate_function_similarities(
    source_symbols: &SymbolTable,
    target_symbols: &SymbolTable,
    similarity_scorer: &SimilarityScorer,
) -> Result<HashMap<String, f64>> {
    let mut similarities: HashMap<String, f64> = HashMap::new();

    // Would need to iterate over functions from symbol table - simplified for now
    let similarities = HashMap::new();
    /*
    for (source_name, source_func) in &source_symbols.functions {
        let mut best_similarity = 0.0;
        let mut best_match = String::new();

        for (target_name, target_func) in &target_symbols.functions {
            // This would need to be implemented in SimilarityScorer
            // let similarity = similarity_scorer.calculate_function_similarity(source_func, target_func)?;

            // Placeholder implementation
            let name_similarity = if source_name == target_name { 1.0 } else { 0.0 };

            if name_similarity > best_similarity {
                best_similarity = name_similarity;
                best_match = target_name.clone();
            }
        }

        if best_similarity > 0.0 {
            similarities.insert(format!("{} -> {}", source_name, best_match), best_similarity);
        }
    }
    */

    Ok(similarities)
}

/// Write output to file or stdout
async fn write_output(
    content: &str,
    output_path: &Option<PathBuf>,
    format: &OutputFormat,
) -> Result<()> {
    match output_path {
        Some(path) => {
            // Ensure directory exists
            if let Some(parent) = path.parent() {
                async_fs::create_dir_all(parent).await
                    .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
            }

            async_fs::write(path, content).await
                .with_context(|| format!("Failed to write output to: {}", path.display()))?;

            info!("Output written to: {}", path.display());
        }
        None => {
            // Write to stdout
            print!("{}", content);
        }
    }

    Ok(())
}

/// Display comparison summary
fn display_summary(
    results: &[ComparisonResult],
    stats: &ComparisonStats,
    elapsed: Duration,
    term: &Term,
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&format!("{}", "Summary".bold().green()))?;
    term.write_line(&format!("{}", "-".repeat(20).dimmed()))?;

    term.write_line(&format!("Files compared: {}", stats.files_compared.to_string().bold()))?;
    term.write_line(&format!("Functions analyzed: {}", stats.functions_compared.to_string().bold()))?;
    term.write_line(&format!("Changes detected: {}", stats.changes_detected.to_string().bold()))?;

    if stats.refactoring_patterns > 0 {
        term.write_line(&format!("Refactoring patterns: {}", stats.refactoring_patterns.to_string().bold().yellow()))?;
    }

    if stats.cross_file_moves > 0 {
        term.write_line(&format!("Cross-file moves: {}", stats.cross_file_moves.to_string().bold().blue()))?;
    }

    term.write_line(&format!("Total time: {}", format_duration(elapsed).bold()))?;

    // Show per-file breakdown if multiple files
    if results.len() > 1 {
        term.write_line("")?;
        term.write_line(&format!("{}", "Per-file Results".bold()))?;
        term.write_line(&format!("{}", "-".repeat(20).dimmed()))?;

        for result in results {
            let file_name = result.source_file.file_name()
                .unwrap_or_default()
                .to_string_lossy();

            let status = if result.diff_result.match_result.changes.is_empty() {
                "No changes".green()
            } else if result.diff_result.match_result.changes.len() < 5 {
                "Minor changes".yellow()
            } else {
                "Major changes".red()
            };

            term.write_line(&format!("  {} - {} ({} changes, {:.1}% similar)",
                file_name,
                status,
                result.diff_result.match_result.changes.len(),
                result.stats.similarity_score * 100.0
            ))?;
        }
    }

    Ok(())
}

/// Display detailed statistics
fn display_detailed_stats(stats: &ComparisonStats, term: &Term) -> Result<()> {
    term.write_line("")?;
    term.write_line(&format!("{}", "Detailed Statistics".bold().cyan()))?;
    term.write_line(&format!("{}", "=".repeat(30).dimmed()))?;

    term.write_line(&format!("Parsing time: {}", format_duration(stats.parsing_time)))?;
    term.write_line(&format!("Comparison time: {}", format_duration(stats.comparison_time)))?;
    term.write_line(&format!("Total processing time: {}", format_duration(stats.total_time)))?;

    term.write_line("")?;
    term.write_line(&format!("Source lines: {}", stats.source_lines))?;
    term.write_line(&format!("Target lines: {}", stats.target_lines))?;
    term.write_line(&format!("Overall similarity: {:.2}%", stats.similarity_score * 100.0))?;

    if stats.functions_compared > 0 {
        term.write_line("")?;
        term.write_line(&format!("Functions per second: {:.1}",
            stats.functions_compared as f64 / stats.comparison_time.as_secs_f64()))?;
        term.write_line(&format!("Lines per second: {:.1}",
            (stats.source_lines + stats.target_lines) as f64 / stats.total_time.as_secs_f64()))?;
    }

    Ok(())
}

/// Format duration for display
fn format_duration(duration: Duration) -> String {
    let total_ms = duration.as_millis();

    if total_ms < 1000 {
        format!("{}ms", total_ms)
    } else if total_ms < 60_000 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        let minutes = total_ms / 60_000;
        let seconds = (total_ms % 60_000) as f64 / 1000.0;
        format!("{}m {:.1}s", minutes, seconds)
    }
}

/// Extract functions from AST for comparison
fn extract_functions_from_ast(ast: &smart_diff_parser::ASTNode) -> Vec<smart_diff_parser::Function> {
    use smart_diff_parser::{Function, FunctionSignature, Parameter, Type, NodeType};

    let mut functions = Vec::new();

    // Find all function nodes in the AST
    let function_nodes = ast.find_by_type(&NodeType::Function);

    for (i, node) in function_nodes.iter().enumerate() {
        let name = node.metadata.attributes.get("name")
            .cloned()
            .unwrap_or_else(|| format!("function_{}", i));

        let signature = FunctionSignature {
            name: name.clone(),
            parameters: Vec::new(), // Simplified for now
            return_type: Some(smart_diff_parser::Type::new("void".to_string())),
            modifiers: Vec::new(),
            generic_parameters: Vec::new(),
        };

        let function = Function {
            signature,
            body: (*node).clone(),
            location: smart_diff_parser::FunctionLocation {
                file_path: "".to_string(),
                start_line: node.metadata.line,
                end_line: node.metadata.line,
                start_column: node.metadata.column,
                end_column: node.metadata.column,
            },
            dependencies: Vec::new(),
            hash: "0".to_string(), // Simplified for now
        };

        functions.push(function);
    }

    functions
}
