//! Comparison manager for handling multiple comparison contexts

use super::context::{ComparisonContext, ComparisonId, ComparisonParams, FunctionChange};
use anyhow::{Context as AnyhowContext, Result};
use smart_diff_engine::{SmartMatcher, SmartMatcherConfig};
use smart_diff_parser::{
    tree_sitter::TreeSitterParser, Function, Language, LanguageDetector, Parser,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use tracing::{debug, info, warn};
use walkdir::WalkDir;

/// Manages multiple comparison contexts
pub struct ComparisonManager {
    contexts: Arc<RwLock<HashMap<ComparisonId, ComparisonContext>>>,
    parser: Arc<Mutex<TreeSitterParser>>,
    smart_matcher: Arc<Mutex<SmartMatcher>>,
}

impl ComparisonManager {
    pub fn new() -> Self {
        let config = SmartMatcherConfig {
            similarity_threshold: 0.7,
            enable_cross_file_matching: true,
            cross_file_penalty: 0.5,
        };

        // Configure parser with large max_text_length to avoid truncating function bodies
        // Default is 200 bytes which is too small for most functions
        let parser = TreeSitterParser::builder()
            .max_text_length(1_000_000) // 1MB should be enough for any reasonable function
            .include_comments(true)
            .extract_signatures(true)
            .build_symbol_table(true)
            .enable_optimization(true)
            .enable_analysis(false) // Disable analysis warnings for MCP usage
            .build()
            .expect("Failed to create parser");

        Self {
            contexts: Arc::new(RwLock::new(HashMap::new())),
            parser: Arc::new(Mutex::new(parser)),
            smart_matcher: Arc::new(Mutex::new(SmartMatcher::new(config))),
        }
    }

    /// Create a new comparison
    pub async fn create_comparison(&self, params: ComparisonParams) -> Result<ComparisonId> {
        info!(
            "Creating comparison: {} vs {}",
            params.source_path, params.target_path
        );

        let mut context = ComparisonContext::new(params.clone());

        // Parse source and target with base paths for relative path calculation
        let source_base = Path::new(&params.source_path);
        let target_base = Path::new(&params.target_path);

        context.source_functions = self
            .parse_location(&params.source_path, &params, source_base)
            .await?;
        context.target_functions = self
            .parse_location(&params.target_path, &params, target_base)
            .await?;

        info!(
            "Parsed {} source functions and {} target functions",
            context.source_functions.len(),
            context.target_functions.len()
        );

        // Perform comparison using smart matcher
        let match_result = self
            .smart_matcher
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?
            .match_functions(&context.source_functions, &context.target_functions);

        // Extract function changes from match result
        let (function_changes, unchanged_moves) = self.extract_function_changes_from_match_result(
            &match_result,
            &context.source_functions,
            &context.target_functions,
            source_base,
            target_base,
        )?;
        context.function_changes = function_changes;
        context.unchanged_moves = unchanged_moves;

        // Calculate change magnitudes
        for change in &mut context.function_changes {
            change.change_magnitude = change.calculate_magnitude();
        }

        let id = context.id;

        // Store context
        self.contexts
            .write()
            .map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?
            .insert(id, context);

        info!("Comparison {} created successfully", id);

        Ok(id)
    }

    /// Get a comparison context
    pub fn get_comparison(&self, id: ComparisonId) -> Result<ComparisonContext> {
        self.contexts
            .read()
            .map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?
            .get(&id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Comparison {} not found", id))
    }

    /// List all comparisons
    pub fn list_comparisons(&self) -> Result<Vec<ComparisonId>> {
        Ok(self
            .contexts
            .read()
            .map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?
            .keys()
            .copied()
            .collect())
    }

    /// Delete a comparison
    #[allow(dead_code)]
    pub fn delete_comparison(&self, id: ComparisonId) -> Result<()> {
        self.contexts
            .write()
            .map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?
            .remove(&id)
            .ok_or_else(|| anyhow::anyhow!("Comparison {} not found", id))?;
        Ok(())
    }

    /// Parse a location (file or directory) and extract functions
    async fn parse_location(
        &self,
        path: &str,
        _params: &ComparisonParams,
        base_path: &Path,
    ) -> Result<Vec<Function>> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
        }

        let mut all_functions = Vec::new();

        if path.is_file() {
            // Parse single file
            let functions = self.parse_file(path, base_path).await?;
            all_functions.extend(functions);
        } else if path.is_dir() {
            // Parse directory recursively (always recursive for directories)
            for entry in WalkDir::new(path)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        let ext_str = ext.to_str().unwrap_or("");
                        if self.is_supported_extension(ext_str) {
                            match self.parse_file(entry.path(), base_path).await {
                                Ok(functions) => {
                                    all_functions.extend(functions);
                                }
                                Err(e) => {
                                    warn!("Failed to parse {}: {}", entry.path().display(), e);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(all_functions)
    }

    /// Parse a single file and extract functions
    async fn parse_file(&self, path: &Path, base_path: &Path) -> Result<Vec<Function>> {
        debug!("Parsing file: {}", path.display());

        let content = tokio::fs::read_to_string(path)
            .await
            .context("Failed to read file")?;

        // Detect language
        let language = LanguageDetector::detect_from_path(path);

        if language == Language::Unknown {
            return Ok(Vec::new());
        }

        // Parse the file
        let parse_result = self
            .parser
            .lock()
            .map_err(|e| anyhow::anyhow!("Lock poisoned: {}", e))?
            .parse(&content, language)?;

        // Extract functions from AST
        let functions = self.extract_functions_from_ast(&parse_result.ast, path, base_path)?;

        debug!(
            "Extracted {} functions from {}",
            functions.len(),
            path.display()
        );

        Ok(functions)
    }

    /// Extract functions from an AST
    fn extract_functions_from_ast(
        &self,
        ast: &smart_diff_parser::ASTNode,
        path: &Path,
        base_path: &Path,
    ) -> Result<Vec<Function>> {
        use smart_diff_parser::NodeType;

        let mut functions = Vec::new();

        // Make path relative to base_path
        let file_path = if let Ok(rel_path) = path.strip_prefix(base_path) {
            rel_path.to_string_lossy().to_string()
        } else {
            path.to_string_lossy().to_string()
        };

        // Find all function nodes
        let function_nodes = ast.find_by_type(&NodeType::Function);
        let method_nodes = ast.find_by_type(&NodeType::Method);

        for node in function_nodes.iter().chain(method_nodes.iter()) {
            // Only process function_definition nodes, not function_declarator
            // function_declarator is just the signature without the body
            if let Some(kind) = node.metadata.attributes.get("kind") {
                if kind == "function_declarator" {
                    continue; // Skip declarators, we only want full definitions
                }
            }

            if let Some(name) = node.metadata.attributes.get("name") {
                let signature = smart_diff_parser::FunctionSignature::new(name.clone());
                let function = Function::new(signature, (*node).clone(), file_path.clone());
                functions.push(function);
            }
        }

        Ok(functions)
    }

    /// Check if file extension is supported
    fn is_supported_extension(&self, ext: &str) -> bool {
        matches!(
            ext.to_lowercase().as_str(),
            "rs" | "py" | "js" | "ts" | "java" | "c" | "cpp" | "h" | "hpp"
        )
    }

    /// Extract function changes from match result
    fn extract_function_changes_from_match_result(
        &self,
        match_result: &smart_diff_parser::MatchResult,
        source_functions: &[Function],
        target_functions: &[Function],
        _source_base: &Path,
        _target_base: &Path,
    ) -> Result<(Vec<FunctionChange>, usize)> {
        // Create hash maps for quick lookup of full function objects
        let source_map: std::collections::HashMap<_, _> = source_functions
            .iter()
            .map(|f| (f.hash.clone(), f))
            .collect();
        let target_map: std::collections::HashMap<_, _> = target_functions
            .iter()
            .map(|f| (f.hash.clone(), f))
            .collect();

        let mut changes = Vec::new();
        let mut unchanged_moves = 0;

        // Process all changes from the match result
        for change in match_result.changes.iter() {
            if let (Some(source), Some(target)) = (&change.source, &change.target) {
                let similarity = change.details.similarity_score.unwrap_or(0.0);
                let is_cross_file_move = source.file_path != target.file_path;
                let is_renamed = source.name != target.name;
                let is_modified = similarity < 0.95; // Consider <95% similarity as modified

                // Determine the most appropriate change type
                // Priority: moved > renamed > modified
                // Note: A function can be moved AND modified - the similarity score indicates modification level
                let change_type_str = match change.change_type {
                    smart_diff_parser::ChangeType::CrossFileMove => {
                        // If moved and significantly modified, still call it "moved"
                        // The similarity score will show it was also modified
                        "moved"
                    }
                    smart_diff_parser::ChangeType::Move => "moved",
                    smart_diff_parser::ChangeType::Rename => {
                        if is_modified {
                            // Renamed and modified - prioritize the rename
                            "renamed"
                        } else {
                            "renamed"
                        }
                    }
                    smart_diff_parser::ChangeType::Modify => "modified",
                    _ => "modified",
                };

                // Look up the full function objects to get body content from AST
                let source_func = source_map.get(&source.hash);
                let target_func = target_map.get(&target.hash);

                let source_content = source_func.map(|f| f.body.metadata.original_text.clone());
                let target_content = target_func.map(|f| f.body.metadata.original_text.clone());

                // Mark high-similarity moves as unchanged moves (file reorganization)
                let is_unchanged_move = is_cross_file_move && similarity >= 0.95 && !is_renamed;
                if is_unchanged_move {
                    unchanged_moves += 1;
                }

                // Create a more descriptive summary
                let diff_summary = if is_cross_file_move && is_modified {
                    Some(format!(
                        "Function moved from {} to {} and modified ({:.0}% similar)",
                        source.file_path,
                        target.file_path,
                        similarity * 100.0
                    ))
                } else if is_cross_file_move {
                    Some(format!(
                        "Function moved from {} to {} (unchanged)",
                        source.file_path, target.file_path
                    ))
                } else if is_renamed && is_modified {
                    Some(format!(
                        "Function renamed from '{}' to '{}' and modified ({:.0}% similar)",
                        source.name,
                        target.name,
                        similarity * 100.0
                    ))
                } else {
                    Some(change.details.description.clone())
                };

                changes.push(FunctionChange {
                    function_name: source.name.clone(),
                    source_file: Some(source.file_path.clone()),
                    target_file: Some(target.file_path.clone()),
                    change_type: change_type_str.to_string(),
                    similarity_score: change.details.similarity_score.unwrap_or(0.0),
                    change_magnitude: 0.0, // Will be calculated later
                    source_signature: source.signature.clone(),
                    target_signature: target.signature.clone(),
                    source_content,
                    target_content,
                    source_start_line: Some(source.start_line),
                    source_end_line: Some(source.end_line),
                    target_start_line: Some(target.start_line),
                    target_end_line: Some(target.end_line),
                    diff_summary,
                    is_unchanged_move,
                });
            } else if let Some(source) = &change.source {
                // Deleted function
                let source_func = source_map.get(&source.hash);
                let source_content = source_func.map(|f| f.body.metadata.original_text.clone());

                changes.push(FunctionChange {
                    function_name: source.name.clone(),
                    source_file: Some(source.file_path.clone()),
                    target_file: None,
                    change_type: "deleted".to_string(),
                    similarity_score: 0.0,
                    change_magnitude: 1.0,
                    source_signature: source.signature.clone(),
                    target_signature: None,
                    source_content,
                    target_content: None,
                    source_start_line: Some(source.start_line),
                    source_end_line: Some(source.end_line),
                    target_start_line: None,
                    target_end_line: None,
                    diff_summary: Some("Function deleted".to_string()),
                    is_unchanged_move: false,
                });
            } else if let Some(target) = &change.target {
                // Added function
                let target_func = target_map.get(&target.hash);
                let target_content = target_func.map(|f| f.body.metadata.original_text.clone());

                changes.push(FunctionChange {
                    function_name: target.name.clone(),
                    source_file: None,
                    target_file: Some(target.file_path.clone()),
                    change_type: "added".to_string(),
                    similarity_score: 0.0,
                    change_magnitude: 1.0,
                    source_signature: None,
                    target_signature: target.signature.clone(),
                    source_content: None,
                    target_content,
                    source_start_line: None,
                    source_end_line: None,
                    target_start_line: Some(target.start_line),
                    target_end_line: Some(target.end_line),
                    diff_summary: Some("Function added".to_string()),
                    is_unchanged_move: false,
                });
            }
        }

        Ok((changes, unchanged_moves))
    }
}

impl Default for ComparisonManager {
    fn default() -> Self {
        Self::new()
    }
}
