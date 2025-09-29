//! Output formatting utilities

use crate::cli::OutputFormat;
use anyhow::{Result, Context};
use colored::*;
use serde::{Deserialize, Serialize};
use smart_diff_parser::{Language, ASTNode};
use smart_diff_engine::{
    DiffResult, RefactoringPattern, DetailedChangeClassification, FunctionMove
};
use smart_diff_semantic::{SymbolTable, FunctionComplexityMetrics, DependencyGraph};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Complete comparison result for a file pair
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub source_file: PathBuf,
    pub target_file: PathBuf,
    pub language: Language,
    pub diff_result: DiffResult,
    pub classified_changes: Vec<DetailedChangeClassification>,
    pub refactoring_patterns: Vec<RefactoringPattern>,
    pub similarity_scores: Option<HashMap<String, f64>>,
    pub cross_file_moves: Vec<FunctionMove>,
    pub stats: ComparisonStats,
    pub source_ast: Option<ASTNode>,
    pub target_ast: Option<ASTNode>,
}

/// Analysis result for a single file
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub file_path: PathBuf,
    pub language: Language,
    pub line_count: usize,
    pub symbols: SymbolTable,
    pub complexity_metrics: Option<FunctionComplexityMetrics>,
    pub dependency_info: Option<DependencyGraph>,
    pub function_signatures: Option<HashMap<String, String>>,
    pub processing_time: Duration,
}

/// Statistics for comparison operations
#[derive(Debug, Clone, Default, Serialize)]
pub struct ComparisonStats {
    pub files_compared: usize,
    pub functions_compared: usize,
    pub changes_detected: usize,
    pub refactoring_patterns: usize,
    pub cross_file_moves: usize,
    pub parsing_time: Duration,
    pub comparison_time: Duration,
    pub total_time: Duration,
    pub source_lines: usize,
    pub target_lines: usize,
    pub similarity_score: f64,
}

impl ComparisonStats {
    /// Merge statistics from another comparison
    pub fn merge(&mut self, other: &ComparisonStats) {
        self.files_compared += other.files_compared;
        self.functions_compared += other.functions_compared;
        self.changes_detected += other.changes_detected;
        self.refactoring_patterns += other.refactoring_patterns;
        self.cross_file_moves += other.cross_file_moves;
        self.parsing_time += other.parsing_time;
        self.comparison_time += other.comparison_time;
        self.total_time += other.total_time;
        self.source_lines += other.source_lines;
        self.target_lines += other.target_lines;

        // Average similarity score
        if other.files_compared > 0 {
            self.similarity_score = (self.similarity_score * (self.files_compared - other.files_compared) as f64
                + other.similarity_score * other.files_compared as f64) / self.files_compared as f64;
        }
    }
}

/// Output formatter for comparison results
pub struct OutputFormatter;

impl OutputFormatter {
    /// Format comparison results in the specified format
    pub fn format_comparison_results(
        results: &[ComparisonResult],
        format: &OutputFormat,
        stats: Option<&ComparisonStats>,
        no_color: bool,
    ) -> Result<String> {
        match format {
            OutputFormat::Text => Self::format_text(results, stats, no_color),
            OutputFormat::Json => Self::format_json(results, stats),
            OutputFormat::JsonCompact => Self::format_json_compact(results, stats),
            OutputFormat::Html => Self::format_html(results, stats),
            OutputFormat::Xml => Self::format_xml(results, stats),
            OutputFormat::Csv => Self::format_csv(results, stats),
            OutputFormat::Markdown => Self::format_markdown(results, stats),
        }
    }

    /// Format analysis results in the specified format
    pub fn format_analysis_results(
        results: &[AnalysisResult],
        format: &OutputFormat,
        no_color: bool,
    ) -> Result<String> {
        match format {
            OutputFormat::Text => Self::format_analysis_text(results, no_color),
            OutputFormat::Json => Self::format_analysis_json(results),
            OutputFormat::JsonCompact => Self::format_analysis_json_compact(results),
            OutputFormat::Html => Self::format_analysis_html(results),
            OutputFormat::Xml => Self::format_analysis_xml(results),
            OutputFormat::Csv => Self::format_analysis_csv(results),
            OutputFormat::Markdown => Self::format_analysis_markdown(results),
        }
    }

    /// Format analysis results as text
    fn format_analysis_text(results: &[AnalysisResult], no_color: bool) -> Result<String> {
        let mut output = String::new();

        let header = "Smart Code Analysis Results";
        if no_color {
            output.push_str(&format!("{}\n{}\n\n", header, "=".repeat(header.len())));
        } else {
            output.push_str(&format!("{}\n{}\n\n",
                header.bold().blue(),
                "=".repeat(header.len()).dimmed()));
        }

        for (index, result) in results.iter().enumerate() {
            if results.len() > 1 {
                let file_header = format!("File {}: {}", index + 1, result.file_path.display());
                if no_color {
                    output.push_str(&format!("{}\n{}\n", file_header, "-".repeat(file_header.len())));
                } else {
                    output.push_str(&format!("{}\n{}\n",
                        file_header.bold().green(),
                        "-".repeat(file_header.len()).dimmed()));
                }
            }

            output.push_str(&format!("Language: {:?}\n", result.language));
            output.push_str(&format!("Lines of code: {}\n", result.line_count));
            output.push_str(&format!("Functions: {}\n", result.symbols.functions.len()));
            output.push_str(&format!("Variables: {}\n", result.symbols.variables.len()));
            output.push_str(&format!("Processing time: {}\n", Self::format_duration(result.processing_time)));

            if let Some(ref complexity) = result.complexity_metrics {
                output.push_str(&format!("Cyclomatic complexity: {}\n", complexity.cyclomatic_complexity));
                output.push_str(&format!("Cognitive complexity: {}\n", complexity.cognitive_complexity));
            }

            if let Some(ref deps) = result.dependency_info {
                output.push_str(&format!("Dependencies: {}\n", deps.edge_count()));
            }

            output.push_str("\n");
        }

        Ok(output)
    }

    /// Format analysis results as JSON
    fn format_analysis_json(results: &[AnalysisResult]) -> Result<String> {
        let output = serde_json::json!({
            "analysis_results": results,
            "format_version": "1.0",
            "generated_at": chrono::Utc::now().to_rfc3339()
        });

        serde_json::to_string_pretty(&output)
            .context("Failed to serialize analysis results to JSON")
    }

    /// Format analysis results as compact JSON
    fn format_analysis_json_compact(results: &[AnalysisResult]) -> Result<String> {
        serde_json::to_string(results)
            .context("Failed to serialize analysis results to compact JSON")
    }

    /// Format analysis results as HTML
    fn format_analysis_html(results: &[AnalysisResult]) -> Result<String> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <title>Code Analysis Results</title>\n");
        html.push_str("    <style>body { font-family: Arial, sans-serif; margin: 20px; }</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("    <h1>Code Analysis Results</h1>\n");

        for (index, result) in results.iter().enumerate() {
            html.push_str(&format!("    <div class=\"file-analysis\" id=\"file-{}\">\n", index));
            html.push_str(&format!("        <h2>{}</h2>\n", html_escape(&result.file_path.to_string_lossy())));
            html.push_str(&format!("        <p><strong>Language:</strong> {:?}</p>\n", result.language));
            html.push_str(&format!("        <p><strong>Lines:</strong> {}</p>\n", result.line_count));
            html.push_str(&format!("        <p><strong>Functions:</strong> {}</p>\n", result.symbols.functions.len()));
            html.push_str("    </div>\n");
        }

        html.push_str("</body>\n</html>");
        Ok(html)
    }

    /// Format analysis results as XML
    fn format_analysis_xml(results: &[AnalysisResult]) -> Result<String> {
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<analysis-results>\n");

        for (index, result) in results.iter().enumerate() {
            xml.push_str(&format!("  <file id=\"{}\">\n", index));
            xml.push_str(&format!("    <path>{}</path>\n", xml_escape(&result.file_path.to_string_lossy())));
            xml.push_str(&format!("    <language>{:?}</language>\n", result.language));
            xml.push_str(&format!("    <lines>{}</lines>\n", result.line_count));
            xml.push_str(&format!("    <functions>{}</functions>\n", result.symbols.functions.len()));
            xml.push_str("  </file>\n");
        }

        xml.push_str("</analysis-results>\n");
        Ok(xml)
    }

    /// Format analysis results as CSV
    fn format_analysis_csv(results: &[AnalysisResult]) -> Result<String> {
        let mut csv = String::new();

        csv.push_str("file_path,language,lines,functions,variables,processing_time_ms\n");

        for result in results {
            csv.push_str(&format!("{},{:?},{},{},{},{}\n",
                csv_escape(&result.file_path.to_string_lossy()),
                result.language,
                result.line_count,
                result.symbols.functions.len(),
                result.symbols.variables.len(),
                result.processing_time.as_millis()
            ));
        }

        Ok(csv)
    }

    /// Format analysis results as Markdown
    fn format_analysis_markdown(results: &[AnalysisResult]) -> Result<String> {
        let mut md = String::new();

        md.push_str("# Code Analysis Results\n\n");

        for (index, result) in results.iter().enumerate() {
            md.push_str(&format!("## File {}: {}\n\n", index + 1, result.file_path.display()));
            md.push_str(&format!("- **Language**: {:?}\n", result.language));
            md.push_str(&format!("- **Lines of Code**: {}\n", result.line_count));
            md.push_str(&format!("- **Functions**: {}\n", result.symbols.functions.len()));
            md.push_str(&format!("- **Variables**: {}\n", result.symbols.variables.len()));
            md.push_str(&format!("- **Processing Time**: {}\n\n", Self::format_duration(result.processing_time)));
        }

        Ok(md)
    }

    /// Format as human-readable text
    fn format_text(
        results: &[ComparisonResult],
        stats: Option<&ComparisonStats>,
        no_color: bool,
    ) -> Result<String> {
        let mut output = String::new();

        // Header
        let header = "Smart Code Diff Results";
        if no_color {
            output.push_str(&format!("{}\n{}\n\n", header, "=".repeat(header.len())));
        } else {
            output.push_str(&format!("{}\n{}\n\n",
                header.bold().blue(),
                "=".repeat(header.len()).dim()));
        }

        // Process each file comparison
        for (index, result) in results.iter().enumerate() {
            if results.len() > 1 {
                let file_header = format!("File Comparison {}: {} -> {}",
                    index + 1,
                    result.source_file.display(),
                    result.target_file.display()
                );

                if no_color {
                    output.push_str(&format!("{}\n{}\n", file_header, "-".repeat(file_header.len())));
                } else {
                    output.push_str(&format!("{}\n{}\n",
                        file_header.bold().green(),
                        "-".repeat(file_header.len()).dim()));
                }
            }

            // Basic information
            output.push_str(&format!("Language: {:?}\n", result.language));
            output.push_str(&format!("Similarity: {:.1}%\n", result.stats.similarity_score * 100.0));
            output.push_str(&format!("Changes: {}\n", result.diff_result.changes.len()));

            if !result.refactoring_patterns.is_empty() {
                output.push_str(&format!("Refactoring Patterns: {}\n", result.refactoring_patterns.len()));
            }

            output.push_str("\n");

            // Changes section
            if !result.diff_result.changes.is_empty() {
                let changes_header = "Changes Detected";
                if no_color {
                    output.push_str(&format!("{}\n{}\n", changes_header, "-".repeat(changes_header.len())));
                } else {
                    output.push_str(&format!("{}\n{}\n",
                        changes_header.bold(),
                        "-".repeat(changes_header.len()).dim()));
                }

                for (i, change) in result.diff_result.changes.iter().enumerate() {
                    let change_desc = format!("{}. {:?}: {}",
                        i + 1,
                        change.change_type,
                        change.details.description
                    );

                    if no_color {
                        output.push_str(&format!("{}\n", change_desc));
                    } else {
                        let colored_desc = match change.change_type {
                            smart_diff_parser::ChangeType::Add => change_desc.green(),
                            smart_diff_parser::ChangeType::Delete => change_desc.red(),
                            smart_diff_parser::ChangeType::Modify => change_desc.yellow(),
                            smart_diff_parser::ChangeType::Rename => change_desc.blue(),
                            smart_diff_parser::ChangeType::Move => change_desc.magenta(),
                            smart_diff_parser::ChangeType::CrossFileMove => change_desc.cyan(),
                        };
                        output.push_str(&format!("{}\n", colored_desc));
                    }
                }
                output.push_str("\n");
            }

            // Refactoring patterns section
            if !result.refactoring_patterns.is_empty() {
                let patterns_header = "Refactoring Patterns";
                if no_color {
                    output.push_str(&format!("{}\n{}\n", patterns_header, "-".repeat(patterns_header.len())));
                } else {
                    output.push_str(&format!("{}\n{}\n",
                        patterns_header.bold().yellow(),
                        "-".repeat(patterns_header.len()).dim()));
                }

                for (i, pattern) in result.refactoring_patterns.iter().enumerate() {
                    output.push_str(&format!("{}. {:?} (confidence: {:.3})\n",
                        i + 1,
                        pattern.pattern_type,
                        pattern.confidence
                    ));
                    output.push_str(&format!("   Description: {}\n", pattern.description));
                    output.push_str(&format!("   Affected: {:?}\n", pattern.affected_elements));
                    output.push_str(&format!("   Complexity: {:?}\n", pattern.complexity.complexity_level));
                }
                output.push_str("\n");
            }

            // Similarity scores section
            if let Some(ref scores) = result.similarity_scores {
                if !scores.is_empty() {
                    let similarity_header = "Function Similarities";
                    if no_color {
                        output.push_str(&format!("{}\n{}\n", similarity_header, "-".repeat(similarity_header.len())));
                    } else {
                        output.push_str(&format!("{}\n{}\n",
                            similarity_header.bold().cyan(),
                            "-".repeat(similarity_header.len()).dim()));
                    }

                    for (func_pair, score) in scores {
                        output.push_str(&format!("{}: {:.3}\n", func_pair, score));
                    }
                    output.push_str("\n");
                }
            }

            // Cross-file moves section
            if !result.cross_file_moves.is_empty() {
                let moves_header = "Cross-File Moves";
                if no_color {
                    output.push_str(&format!("{}\n{}\n", moves_header, "-".repeat(moves_header.len())));
                } else {
                    output.push_str(&format!("{}\n{}\n",
                        moves_header.bold().magenta(),
                        "-".repeat(moves_header.len()).dim()));
                }

                for (i, move_info) in result.cross_file_moves.iter().enumerate() {
                    output.push_str(&format!("{}. {} (confidence: {:.3})\n",
                        i + 1,
                        move_info.description,
                        move_info.confidence
                    ));
                }
                output.push_str("\n");
            }

            if results.len() > 1 && index < results.len() - 1 {
                output.push_str(&format!("{}\n\n", "=".repeat(80).dim()));
            }
        }

        // Statistics section
        if let Some(stats) = stats {
            let stats_header = "Overall Statistics";
            if no_color {
                output.push_str(&format!("{}\n{}\n", stats_header, "=".repeat(stats_header.len())));
            } else {
                output.push_str(&format!("{}\n{}\n",
                    stats_header.bold().green(),
                    "=".repeat(stats_header.len()).dim()));
            }

            output.push_str(&format!("Files compared: {}\n", stats.files_compared));
            output.push_str(&format!("Functions analyzed: {}\n", stats.functions_compared));
            output.push_str(&format!("Total changes: {}\n", stats.changes_detected));
            output.push_str(&format!("Refactoring patterns: {}\n", stats.refactoring_patterns));
            output.push_str(&format!("Cross-file moves: {}\n", stats.cross_file_moves));
            output.push_str(&format!("Average similarity: {:.1}%\n", stats.similarity_score * 100.0));
            output.push_str(&format!("Total processing time: {}\n", Self::format_duration(stats.total_time)));
        }

        Ok(output)
    }

    /// Format as JSON
    fn format_json(results: &[ComparisonResult], stats: Option<&ComparisonStats>) -> Result<String> {
        let output = serde_json::json!({
            "results": results,
            "stats": stats,
            "format_version": "1.0",
            "generated_at": chrono::Utc::now().to_rfc3339()
        });

        serde_json::to_string_pretty(&output)
            .context("Failed to serialize results to JSON")
    }

    /// Format as compact JSON
    fn format_json_compact(results: &[ComparisonResult], stats: Option<&ComparisonStats>) -> Result<String> {
        let output = serde_json::json!({
            "results": results,
            "stats": stats
        });

        serde_json::to_string(&output)
            .context("Failed to serialize results to compact JSON")
    }

    /// Format as HTML
    fn format_html(results: &[ComparisonResult], stats: Option<&ComparisonStats>) -> Result<String> {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("    <title>Smart Code Diff Results</title>\n");
        html.push_str("    <style>\n");
        html.push_str(include_str!("assets/diff.css"));
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        // Main content
        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <h1>Smart Code Diff Results</h1>\n");

        // Process each comparison
        for (index, result) in results.iter().enumerate() {
            html.push_str(&format!("        <div class=\"comparison\" id=\"comparison-{}\">\n", index));

            if results.len() > 1 {
                html.push_str(&format!("            <h2>Comparison {}: {} → {}</h2>\n",
                    index + 1,
                    result.source_file.display(),
                    result.target_file.display()
                ));
            }

            // Basic info
            html.push_str("            <div class=\"info-panel\">\n");
            html.push_str(&format!("                <div class=\"info-item\"><strong>Language:</strong> {:?}</div>\n", result.language));
            html.push_str(&format!("                <div class=\"info-item\"><strong>Similarity:</strong> {:.1}%</div>\n", result.stats.similarity_score * 100.0));
            html.push_str(&format!("                <div class=\"info-item\"><strong>Changes:</strong> {}</div>\n", result.diff_result.changes.len()));
            html.push_str("            </div>\n");

            // Changes
            if !result.diff_result.changes.is_empty() {
                html.push_str("            <div class=\"changes-section\">\n");
                html.push_str("                <h3>Changes Detected</h3>\n");
                html.push_str("                <ul class=\"changes-list\">\n");

                for change in &result.diff_result.changes {
                    let class_name = match change.change_type {
                        smart_diff_parser::ChangeType::Add => "addition",
                        smart_diff_parser::ChangeType::Delete => "deletion",
                        smart_diff_parser::ChangeType::Modify => "modification",
                        smart_diff_parser::ChangeType::Rename => "rename",
                        smart_diff_parser::ChangeType::Move => "move",
                        smart_diff_parser::ChangeType::CrossFileMove => "cross-file-move",
                    };

                    html.push_str(&format!("                    <li class=\"change-item {}\">\n", class_name));
                    html.push_str(&format!("                        <span class=\"change-type\">{:?}</span>\n", change.change_type));
                    html.push_str(&format!("                        <span class=\"change-desc\">{}</span>\n",
                        html_escape(&change.details.description)));
                    html.push_str("                    </li>\n");
                }

                html.push_str("                </ul>\n");
                html.push_str("            </div>\n");
            }

            // Refactoring patterns
            if !result.refactoring_patterns.is_empty() {
                html.push_str("            <div class=\"patterns-section\">\n");
                html.push_str("                <h3>Refactoring Patterns</h3>\n");
                html.push_str("                <div class=\"patterns-grid\">\n");

                for pattern in &result.refactoring_patterns {
                    html.push_str("                    <div class=\"pattern-card\">\n");
                    html.push_str(&format!("                        <div class=\"pattern-type\">{:?}</div>\n", pattern.pattern_type));
                    html.push_str(&format!("                        <div class=\"pattern-confidence\">Confidence: {:.1}%</div>\n", pattern.confidence * 100.0));
                    html.push_str(&format!("                        <div class=\"pattern-desc\">{}</div>\n",
                        html_escape(&pattern.description)));
                    html.push_str("                    </div>\n");
                }

                html.push_str("                </div>\n");
                html.push_str("            </div>\n");
            }

            html.push_str("        </div>\n");
        }

        // Statistics
        if let Some(stats) = stats {
            html.push_str("        <div class=\"stats-section\">\n");
            html.push_str("            <h2>Overall Statistics</h2>\n");
            html.push_str("            <div class=\"stats-grid\">\n");
            html.push_str(&format!("                <div class=\"stat-item\"><strong>Files:</strong> {}</div>\n", stats.files_compared));
            html.push_str(&format!("                <div class=\"stat-item\"><strong>Functions:</strong> {}</div>\n", stats.functions_compared));
            html.push_str(&format!("                <div class=\"stat-item\"><strong>Changes:</strong> {}</div>\n", stats.changes_detected));
            html.push_str(&format!("                <div class=\"stat-item\"><strong>Patterns:</strong> {}</div>\n", stats.refactoring_patterns));
            html.push_str(&format!("                <div class=\"stat-item\"><strong>Similarity:</strong> {:.1}%</div>\n", stats.similarity_score * 100.0));
            html.push_str(&format!("                <div class=\"stat-item\"><strong>Time:</strong> {}</div>\n", Self::format_duration(stats.total_time)));
            html.push_str("            </div>\n");
            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");
        html.push_str("</body>\n</html>");

        Ok(html)
    }

    /// Format as XML
    fn format_xml(results: &[ComparisonResult], stats: Option<&ComparisonStats>) -> Result<String> {
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<smart-diff-results>\n");

        // Metadata
        xml.push_str("  <metadata>\n");
        xml.push_str("    <format-version>1.0</format-version>\n");
        xml.push_str(&format!("    <generated-at>{}</generated-at>\n", chrono::Utc::now().to_rfc3339()));
        xml.push_str("  </metadata>\n");

        // Results
        xml.push_str("  <comparisons>\n");
        for (index, result) in results.iter().enumerate() {
            xml.push_str(&format!("    <comparison id=\"{}\">\n", index));
            xml.push_str(&format!("      <source-file>{}</source-file>\n", xml_escape(&result.source_file.to_string_lossy())));
            xml.push_str(&format!("      <target-file>{}</target-file>\n", xml_escape(&result.target_file.to_string_lossy())));
            xml.push_str(&format!("      <language>{:?}</language>\n", result.language));
            xml.push_str(&format!("      <similarity>{:.6}</similarity>\n", result.stats.similarity_score));

            // Changes
            xml.push_str("      <changes>\n");
            for change in &result.diff_result.changes {
                xml.push_str("        <change>\n");
                xml.push_str(&format!("          <type>{:?}</type>\n", change.change_type));
                xml.push_str(&format!("          <description>{}</description>\n", xml_escape(&change.details.description)));
                xml.push_str(&format!("          <confidence>{:.6}</confidence>\n", change.confidence));
                xml.push_str("        </change>\n");
            }
            xml.push_str("      </changes>\n");

            // Refactoring patterns
            if !result.refactoring_patterns.is_empty() {
                xml.push_str("      <refactoring-patterns>\n");
                for pattern in &result.refactoring_patterns {
                    xml.push_str("        <pattern>\n");
                    xml.push_str(&format!("          <type>{:?}</type>\n", pattern.pattern_type));
                    xml.push_str(&format!("          <confidence>{:.6}</confidence>\n", pattern.confidence));
                    xml.push_str(&format!("          <description>{}</description>\n", xml_escape(&pattern.description)));
                    xml.push_str("        </pattern>\n");
                }
                xml.push_str("      </refactoring-patterns>\n");
            }

            xml.push_str("    </comparison>\n");
        }
        xml.push_str("  </comparisons>\n");

        // Statistics
        if let Some(stats) = stats {
            xml.push_str("  <statistics>\n");
            xml.push_str(&format!("    <files-compared>{}</files-compared>\n", stats.files_compared));
            xml.push_str(&format!("    <functions-compared>{}</functions-compared>\n", stats.functions_compared));
            xml.push_str(&format!("    <changes-detected>{}</changes-detected>\n", stats.changes_detected));
            xml.push_str(&format!("    <refactoring-patterns>{}</refactoring-patterns>\n", stats.refactoring_patterns));
            xml.push_str(&format!("    <similarity-score>{:.6}</similarity-score>\n", stats.similarity_score));
            xml.push_str(&format!("    <total-time-ms>{}</total-time-ms>\n", stats.total_time.as_millis()));
            xml.push_str("  </statistics>\n");
        }

        xml.push_str("</smart-diff-results>\n");

        Ok(xml)
    }

    /// Format as CSV
    fn format_csv(results: &[ComparisonResult], _stats: Option<&ComparisonStats>) -> Result<String> {
        let mut csv = String::new();

        // Header
        csv.push_str("source_file,target_file,language,similarity,changes,refactoring_patterns,processing_time_ms\n");

        // Data rows
        for result in results {
            csv.push_str(&format!("{},{},{:?},{:.6},{},{},{}\n",
                csv_escape(&result.source_file.to_string_lossy()),
                csv_escape(&result.target_file.to_string_lossy()),
                result.language,
                result.stats.similarity_score,
                result.diff_result.changes.len(),
                result.refactoring_patterns.len(),
                result.stats.total_time.as_millis()
            ));
        }

        Ok(csv)
    }

    /// Format as Markdown
    fn format_markdown(results: &[ComparisonResult], stats: Option<&ComparisonStats>) -> Result<String> {
        let mut md = String::new();

        // Title
        md.push_str("# Smart Code Diff Results\n\n");

        // Process each comparison
        for (index, result) in results.iter().enumerate() {
            if results.len() > 1 {
                md.push_str(&format!("## Comparison {}: {} → {}\n\n",
                    index + 1,
                    result.source_file.display(),
                    result.target_file.display()
                ));
            }

            // Basic information
            md.push_str("### Overview\n\n");
            md.push_str(&format!("- **Language**: {:?}\n", result.language));
            md.push_str(&format!("- **Similarity**: {:.1}%\n", result.stats.similarity_score * 100.0));
            md.push_str(&format!("- **Changes**: {}\n", result.diff_result.changes.len()));
            md.push_str(&format!("- **Processing Time**: {}\n\n", Self::format_duration(result.stats.total_time)));

            // Changes
            if !result.diff_result.changes.is_empty() {
                md.push_str("### Changes Detected\n\n");
                for (i, change) in result.diff_result.changes.iter().enumerate() {
                    let emoji = match change.change_type {
                        smart_diff_parser::ChangeType::Add => "➕",
                        smart_diff_parser::ChangeType::Delete => "➖",
                        smart_diff_parser::ChangeType::Modify => "✏️",
                        smart_diff_parser::ChangeType::Rename => "🏷️",
                        smart_diff_parser::ChangeType::Move => "📦",
                        smart_diff_parser::ChangeType::CrossFileMove => "🔄",
                    };

                    md.push_str(&format!("{}. {} **{:?}**: {}\n",
                        i + 1, emoji, change.change_type, change.details.description));
                }
                md.push_str("\n");
            }

            // Refactoring patterns
            if !result.refactoring_patterns.is_empty() {
                md.push_str("### Refactoring Patterns\n\n");
                for (i, pattern) in result.refactoring_patterns.iter().enumerate() {
                    md.push_str(&format!("{}. **{:?}** (confidence: {:.1}%)\n",
                        i + 1, pattern.pattern_type, pattern.confidence * 100.0));
                    md.push_str(&format!("   - {}\n", pattern.description));
                    md.push_str(&format!("   - Affected: {}\n", pattern.affected_elements.join(", ")));
                    md.push_str(&format!("   - Complexity: {:?}\n", pattern.complexity.complexity_level));
                }
                md.push_str("\n");
            }

            if results.len() > 1 && index < results.len() - 1 {
                md.push_str("---\n\n");
            }
        }

        // Statistics
        if let Some(stats) = stats {
            md.push_str("## Overall Statistics\n\n");
            md.push_str("| Metric | Value |\n");
            md.push_str("|--------|-------|\n");
            md.push_str(&format!("| Files Compared | {} |\n", stats.files_compared));
            md.push_str(&format!("| Functions Analyzed | {} |\n", stats.functions_compared));
            md.push_str(&format!("| Total Changes | {} |\n", stats.changes_detected));
            md.push_str(&format!("| Refactoring Patterns | {} |\n", stats.refactoring_patterns));
            md.push_str(&format!("| Average Similarity | {:.1}% |\n", stats.similarity_score * 100.0));
            md.push_str(&format!("| Total Processing Time | {} |\n", Self::format_duration(stats.total_time)));
        }

        Ok(md)
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
}

// Utility functions for escaping

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn csv_escape(text: &str) -> String {
    if text.contains(',') || text.contains('"') || text.contains('\n') {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}
