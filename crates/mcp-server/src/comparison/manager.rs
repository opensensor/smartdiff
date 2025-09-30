//! Comparison manager for handling multiple comparison contexts

use super::context::{ComparisonContext, ComparisonId, ComparisonParams, FunctionChange};
use anyhow::{Context as AnyhowContext, Result};
use smart_diff_engine::DiffEngine;
use smart_diff_parser::{
    tree_sitter::TreeSitterParser, Function, Language, LanguageDetector, Parser,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};
use walkdir::WalkDir;

/// Manages multiple comparison contexts
pub struct ComparisonManager {
    contexts: Arc<RwLock<HashMap<ComparisonId, ComparisonContext>>>,
    parser: TreeSitterParser,
    diff_engine: DiffEngine,
}

impl ComparisonManager {
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(RwLock::new(HashMap::new())),
            parser: TreeSitterParser::new().expect("Failed to create parser"),
            diff_engine: DiffEngine::new(),
        }
    }

    /// Create a new comparison
    pub async fn create_comparison(&self, params: ComparisonParams) -> Result<ComparisonId> {
        info!(
            "Creating comparison: {} vs {}",
            params.source_path, params.target_path
        );

        let mut context = ComparisonContext::new(params.clone());

        // Parse source and target
        context.source_functions = self.parse_location(&params.source_path, &params).await?;
        context.target_functions = self.parse_location(&params.target_path, &params).await?;

        info!(
            "Parsed {} source functions and {} target functions",
            context.source_functions.len(),
            context.target_functions.len()
        );

        // Perform comparison
        let diff_result = self
            .diff_engine
            .compare_functions(&context.source_functions, &context.target_functions)?;

        eprintln!("DEBUG: Diff result has {} changes", diff_result.match_result.changes.len());
        eprintln!("DEBUG: Diff result has {} unmatched_source", diff_result.match_result.unmatched_source.len());
        eprintln!("DEBUG: Diff result has {} unmatched_target", diff_result.match_result.unmatched_target.len());

        // Extract function changes
        context.function_changes = self.extract_function_changes(&diff_result, &context);

        eprintln!("DEBUG: Extracted {} function changes", context.function_changes.len());

        // Calculate change magnitudes
        for change in &mut context.function_changes {
            change.change_magnitude = change.calculate_magnitude();
        }

        context.diff_result = Some(diff_result);

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
        params: &ComparisonParams,
    ) -> Result<Vec<Function>> {
        let path = Path::new(path);

        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
        }

        let mut all_functions = Vec::new();

        if path.is_file() {
            // Parse single file
            let functions = self.parse_file(path).await?;
            all_functions.extend(functions);
        } else if path.is_dir() && params.recursive {
            // Parse directory recursively
            for entry in WalkDir::new(path)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        let ext_str = ext.to_str().unwrap_or("");
                        eprintln!("DEBUG: Checking file {} with extension {}", entry.path().display(), ext_str);
                        if self.is_supported_extension(ext_str) {
                            eprintln!("DEBUG: Parsing file {}", entry.path().display());
                            match self.parse_file(entry.path()).await {
                                Ok(functions) => {
                                    eprintln!("DEBUG: Found {} functions in {}", functions.len(), entry.path().display());
                                    all_functions.extend(functions);
                                }
                                Err(e) => {
                                    warn!("Failed to parse {}: {}", entry.path().display(), e);
                                    eprintln!("DEBUG: Failed to parse {}: {}", entry.path().display(), e);
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
    async fn parse_file(&self, path: &Path) -> Result<Vec<Function>> {
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
        let parse_result = self.parser.parse(&content, language)?;

        // Extract functions from AST
        let functions = self.extract_functions_from_ast(&parse_result.ast, path)?;

        debug!("Extracted {} functions from {}", functions.len(), path.display());

        Ok(functions)
    }

    /// Extract functions from an AST
    fn extract_functions_from_ast(
        &self,
        ast: &smart_diff_parser::ASTNode,
        path: &Path,
    ) -> Result<Vec<Function>> {
        use smart_diff_parser::NodeType;

        let mut functions = Vec::new();
        let file_path = path.to_string_lossy().to_string();

        // Find all function nodes
        let function_nodes = ast.find_by_type(&NodeType::Function);
        let method_nodes = ast.find_by_type(&NodeType::Method);

        for node in function_nodes.iter().chain(method_nodes.iter()) {
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

    /// Extract function changes from diff result
    fn extract_function_changes(
        &self,
        diff_result: &smart_diff_engine::DiffResult,
        _context: &ComparisonContext,
    ) -> Vec<FunctionChange> {
        let mut changes = Vec::new();

        eprintln!("DEBUG: extract_function_changes processing {} changes", diff_result.match_result.changes.len());

        // Process matched functions (modified)
        for (i, change) in diff_result.match_result.changes.iter().enumerate() {
            eprintln!("DEBUG: Change {}: source={:?}, target={:?}", i, change.source.is_some(), change.target.is_some());
            if let (Some(source), Some(target)) = (&change.source, &change.target) {
                changes.push(FunctionChange {
                    function_name: source.name.clone(),
                    source_file: Some(source.file_path.clone()),
                    target_file: Some(target.file_path.clone()),
                    change_type: format!("{:?}", change.change_type).to_lowercase(),
                    similarity_score: change.details.similarity_score.unwrap_or(0.0),
                    change_magnitude: 0.0, // Will be calculated later
                    source_signature: source.signature.clone(),
                    target_signature: target.signature.clone(),
                    source_content: None,
                    target_content: None,
                    source_start_line: Some(source.start_line),
                    source_end_line: Some(source.end_line),
                    target_start_line: Some(target.start_line),
                    target_end_line: Some(target.end_line),
                    diff_summary: Some(change.details.description.clone()),
                });
            } else if let Some(source) = &change.source {
                // Deleted function
                changes.push(FunctionChange {
                    function_name: source.name.clone(),
                    source_file: Some(source.file_path.clone()),
                    target_file: None,
                    change_type: "deleted".to_string(),
                    similarity_score: 0.0,
                    change_magnitude: 1.0,
                    source_signature: source.signature.clone(),
                    target_signature: None,
                    source_content: None,
                    target_content: None,
                    source_start_line: Some(source.start_line),
                    source_end_line: Some(source.end_line),
                    target_start_line: None,
                    target_end_line: None,
                    diff_summary: Some("Function deleted".to_string()),
                });
            } else if let Some(target) = &change.target {
                // Added function
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
                    target_content: None,
                    source_start_line: None,
                    source_end_line: None,
                    target_start_line: Some(target.start_line),
                    target_end_line: Some(target.end_line),
                    diff_summary: Some("Function added".to_string()),
                });
            }
        }

        changes
    }
}

impl Default for ComparisonManager {
    fn default() -> Self {
        Self::new()
    }
}

