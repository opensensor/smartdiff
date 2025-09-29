//! Analyze command implementation

use crate::cli::{Cli, Commands, OutputFormat};
use crate::output::{OutputFormatter, AnalysisResult};
use anyhow::{Result, Context, bail};
use colored::*;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};
use smart_diff_parser::{Parser, LanguageDetector, Language};
use smart_diff_semantic::{SemanticAnalyzer, SymbolTable, ComplexityAnalyzer, DependencyAnalyzer};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio::fs as async_fs;
use tracing::{info, warn, debug};

pub async fn run(cli: Cli) -> Result<()> {
    if let Commands::Analyze {
        path,
        format,
        recursive,
        language,
        complexity,
        dependencies,
        signatures,
        output,
    } = cli.command
    {
        let start_time = Instant::now();
        let term = Term::stdout();
        
        if !cli.quiet {
            println!("{}", "Smart Code Analysis".bold().blue());
            println!("{}", "=".repeat(30).dim());
        }

        // Validate input
        if !path.exists() {
            bail!("Path does not exist: {}", path.display());
        }

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

        // Step 1: File discovery
        if let Some(ref pb) = progress {
            pb.set_message("Discovering files...");
            pb.set_position(10);
        }

        let files = discover_analysis_files(&path, recursive).await
            .context("Failed to discover files for analysis")?;

        if files.is_empty() {
            bail!("No source files found to analyze");
        }

        info!("Found {} files to analyze", files.len());

        // Step 2: Language detection and parser initialization
        if let Some(ref pb) = progress {
            pb.set_message("Detecting languages...");
            pb.set_position(20);
        }

        let language_detector = LanguageDetector::new();
        let mut parsers: HashMap<Language, Parser> = HashMap::new();
        let mut analysis_results = Vec::new();

        // Step 3: Process each file
        let total_files = files.len();
        for (index, file_path) in files.iter().enumerate() {
            if let Some(ref pb) = progress {
                pb.set_message(format!("Analyzing {}/{}: {}", 
                    index + 1, total_files, 
                    file_path.file_name().unwrap_or_default().to_string_lossy()));
                pb.set_position(20 + (60 * index as u64) / total_files as u64);
            }

            let file_result = analyze_file(
                file_path,
                &language,
                &language_detector,
                &mut parsers,
                complexity,
                dependencies,
                signatures,
                &cli,
            ).await;

            match file_result {
                Ok(result) => {
                    analysis_results.push(result);
                }
                Err(e) => {
                    warn!("Failed to analyze file {:?}: {}", file_path, e);
                    if !cli.quiet {
                        eprintln!("{} Failed to analyze {}: {}", 
                            "Warning:".yellow().bold(), 
                            file_path.display(), 
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

        let output_content = OutputFormatter::format_analysis_results(
            &analysis_results,
            &format,
            cli.no_color,
        )?;

        // Step 5: Write output
        if let Some(ref pb) = progress {
            pb.set_message("Writing output...");
            pb.set_position(95);
        }

        write_analysis_output(&output_content, &output, &format).await
            .context("Failed to write output")?;

        if let Some(ref pb) = progress {
            pb.finish_with_message("Analysis complete!");
        }

        // Display summary
        let elapsed = start_time.elapsed();
        if !cli.quiet {
            display_analysis_summary(&analysis_results, elapsed, &term)?;
        }

        Ok(())
    } else {
        unreachable!("Analyze command should have been matched")
    }
}

/// Discover files for analysis
async fn discover_analysis_files(path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if path.is_file() {
        files.push(path.to_path_buf());
    } else if path.is_dir() {
        collect_source_files(path, recursive, &mut files).await?;
    }

    Ok(files)
}

/// Collect source files from directory
async fn collect_source_files(dir: &Path, recursive: bool, files: &mut Vec<PathBuf>) -> Result<()> {
    let mut entries = async_fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();

        // Skip hidden files and directories
        if file_name.starts_with('.') {
            continue;
        }

        if path.is_file() {
            // Check if it's a source file
            if is_source_file(&path) {
                files.push(path);
            }
        } else if path.is_dir() && recursive {
            collect_source_files(&path, recursive, files).await?;
        }
    }

    Ok(())
}

/// Check if file is a source code file
fn is_source_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), 
            "java" | "py" | "pyx" | "pyi" | 
            "js" | "jsx" | "mjs" | "cjs" | 
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" | "h" | 
            "c"
        )
    } else {
        false
    }
}

/// Analyze a single file
async fn analyze_file(
    file_path: &Path,
    language_override: &Option<crate::cli::Language>,
    language_detector: &LanguageDetector,
    parsers: &mut HashMap<Language, Parser>,
    include_complexity: bool,
    include_dependencies: bool,
    include_signatures: bool,
    cli: &Cli,
) -> Result<AnalysisResult> {
    let file_start = Instant::now();

    // Read file content
    let content = async_fs::read_to_string(file_path).await
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

    // Detect language
    let detected_language = if let Some(lang_override) = language_override {
        lang_override.to_parser_language()
            .context("Invalid language override")?
    } else {
        language_detector.detect_from_path(file_path)
            .or_else(|| language_detector.detect_from_content(&content))
            .context("Could not detect programming language")?
    };

    debug!("Detected language: {:?} for file: {}", detected_language, file_path.display());

    // Get or create parser for this language
    let parser = parsers.entry(detected_language)
        .or_insert_with(|| Parser::new(detected_language));

    // Parse file
    let ast = parser.parse(&content, Some(file_path.to_string_lossy().to_string()))
        .with_context(|| format!("Failed to parse file: {}", file_path.display()))?;

    // Perform semantic analysis
    let mut semantic_analyzer = SemanticAnalyzer::new(detected_language);
    let symbols = semantic_analyzer.analyze(&ast)
        .with_context(|| format!("Failed to analyze file: {}", file_path.display()))?;

    // Complexity analysis
    let complexity_metrics = if include_complexity {
        let complexity_analyzer = ComplexityAnalyzer::new(detected_language);
        Some(complexity_analyzer.analyze(&ast, &symbols)?)
    } else {
        None
    };

    // Dependency analysis
    let dependency_info = if include_dependencies {
        let dependency_analyzer = DependencyAnalyzer::new(detected_language);
        Some(dependency_analyzer.analyze(&ast, &symbols)?)
    } else {
        None
    };

    // Function signatures
    let function_signatures = if include_signatures {
        Some(extract_function_signatures(&symbols))
    } else {
        None
    };

    // Build analysis result
    let result = AnalysisResult {
        file_path: file_path.to_path_buf(),
        language: detected_language,
        line_count: content.lines().count(),
        symbols,
        complexity_metrics,
        dependency_info,
        function_signatures,
        processing_time: file_start.elapsed(),
    };

    if cli.verbose {
        info!("Analyzed {} in {:?}", file_path.display(), file_start.elapsed());
    }

    Ok(result)
}

/// Extract function signatures from symbol table
fn extract_function_signatures(symbols: &SymbolTable) -> HashMap<String, String> {
    let mut signatures = HashMap::new();
    
    for (name, function) in &symbols.functions {
        // This would be implemented based on the actual function structure
        let signature = format!("{}(...)", name); // Placeholder
        signatures.insert(name.clone(), signature);
    }
    
    signatures
}

/// Write analysis output
async fn write_analysis_output(
    content: &str,
    output_path: &Option<PathBuf>,
    format: &OutputFormat,
) -> Result<()> {
    match output_path {
        Some(path) => {
            if let Some(parent) = path.parent() {
                async_fs::create_dir_all(parent).await
                    .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
            }

            async_fs::write(path, content).await
                .with_context(|| format!("Failed to write output to: {}", path.display()))?;
            
            info!("Analysis output written to: {}", path.display());
        }
        None => {
            print!("{}", content);
        }
    }

    Ok(())
}

/// Display analysis summary
fn display_analysis_summary(
    results: &[AnalysisResult],
    elapsed: std::time::Duration,
    term: &Term,
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&format!("{}", "Analysis Summary".bold().green()))?;
    term.write_line(&format!("{}", "-".repeat(20).dim()))?;
    
    let total_files = results.len();
    let total_lines: usize = results.iter().map(|r| r.line_count).sum();
    let total_functions: usize = results.iter().map(|r| r.symbols.functions.len()).sum();
    
    term.write_line(&format!("Files analyzed: {}", total_files.to_string().bold()))?;
    term.write_line(&format!("Total lines: {}", total_lines.to_string().bold()))?;
    term.write_line(&format!("Total functions: {}", total_functions.to_string().bold()))?;
    term.write_line(&format!("Processing time: {}", format_duration(elapsed).bold()))?;
    
    if total_files > 0 {
        term.write_line(&format!("Average lines per file: {}", (total_lines / total_files).to_string().bold()))?;
        term.write_line(&format!("Files per second: {:.1}", total_files as f64 / elapsed.as_secs_f64()))?;
    }

    Ok(())
}

/// Format duration for display
fn format_duration(duration: std::time::Duration) -> String {
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
