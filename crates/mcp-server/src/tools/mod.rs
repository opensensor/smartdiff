//! MCP tools implementation

use crate::comparison::{ComparisonId, ComparisonManager, ComparisonParams};
use crate::mcp::protocol::{CallToolResult, ToolContent, ToolInfo};
use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, info};

/// Tool handler
pub struct ToolHandler {
    comparison_manager: Arc<ComparisonManager>,
}

impl ToolHandler {
    pub fn new(comparison_manager: Arc<ComparisonManager>) -> Self {
        Self { comparison_manager }
    }

    /// List all available tools
    pub fn list_tools(&self) -> Vec<ToolInfo> {
        vec![
            ToolInfo {
                name: "compare_locations".to_string(),
                description: "Compare two code locations (files or directories) and analyze changes. Returns a comparison ID for querying results.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "source_path": {
                            "type": "string",
                            "description": "Path to the source code location (file or directory)"
                        },
                        "target_path": {
                            "type": "string",
                            "description": "Path to the target code location (file or directory)"
                        },
                        "recursive": {
                            "type": "boolean",
                            "description": "Whether to recursively scan directories",
                            "default": true
                        },
                        "file_patterns": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "File patterns to include (e.g., ['*.rs', '*.py'])",
                            "default": []
                        },
                        "ignore_patterns": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "File patterns to ignore",
                            "default": []
                        }
                    },
                    "required": ["source_path", "target_path"]
                }),
            },
            ToolInfo {
                name: "list_changed_functions".to_string(),
                description: "List all changed functions from a comparison, sorted by change magnitude (most changed first). Includes additions, deletions, modifications, renames, and moves.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "comparison_id": {
                            "type": "string",
                            "description": "The comparison ID returned from compare_locations"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of functions to return",
                            "default": 100
                        },
                        "change_types": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "enum": ["added", "deleted", "modified", "renamed", "moved"]
                            },
                            "description": "Filter by change types"
                        },
                        "min_magnitude": {
                            "type": "number",
                            "description": "Minimum change magnitude (0.0 to 1.0)",
                            "minimum": 0.0,
                            "maximum": 1.0
                        }
                    },
                    "required": ["comparison_id"]
                }),
            },
            ToolInfo {
                name: "get_function_diff".to_string(),
                description: "Get detailed diff information for a specific function, including source and target content, line numbers, and change analysis.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "comparison_id": {
                            "type": "string",
                            "description": "The comparison ID"
                        },
                        "function_name": {
                            "type": "string",
                            "description": "Name of the function to get diff for"
                        },
                        "include_content": {
                            "type": "boolean",
                            "description": "Whether to include full source and target content",
                            "default": true
                        }
                    },
                    "required": ["comparison_id", "function_name"]
                }),
            },
            ToolInfo {
                name: "get_comparison_summary".to_string(),
                description: "Get summary statistics for a comparison, including counts of added, deleted, modified, renamed, and moved functions.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "comparison_id": {
                            "type": "string",
                            "description": "The comparison ID"
                        }
                    },
                    "required": ["comparison_id"]
                }),
            },
        ]
    }

    /// Execute a tool
    pub async fn call_tool(&self, name: &str, arguments: Option<Value>) -> Result<CallToolResult> {
        info!("Calling tool: {}", name);
        debug!("Arguments: {:?}", arguments);

        match name {
            "compare_locations" => self.compare_locations(arguments).await,
            "list_changed_functions" => self.list_changed_functions(arguments).await,
            "get_function_diff" => self.get_function_diff(arguments).await,
            "get_comparison_summary" => self.get_comparison_summary(arguments).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }

    /// Compare two locations
    async fn compare_locations(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let source_path = args["source_path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing source_path"))?
            .to_string();

        let target_path = args["target_path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing target_path"))?
            .to_string();

        let recursive = args["recursive"].as_bool().unwrap_or(true);
        let file_patterns = args["file_patterns"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let ignore_patterns = args["ignore_patterns"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let params = ComparisonParams {
            source_path,
            target_path,
            recursive,
            file_patterns,
            ignore_patterns,
        };

        let comparison_id = self.comparison_manager.create_comparison(params).await?;
        let context = self.comparison_manager.get_comparison(comparison_id)?;
        let summary = context.get_summary();

        let result_text = format!(
            "Comparison created successfully!\n\n\
            Comparison ID: {}\n\
            Source: {}\n\
            Target: {}\n\n\
            Summary:\n\
            - Total functions: {}\n\
            - Added: {}\n\
            - Deleted: {}\n\
            - Modified: {}\n\
            - Renamed: {}\n\
            - Moved: {}\n\
            - Unchanged: {}\n\
            - File reorganizations: {} (functions moved without changes, filtered from results)\n\n\
            Use list_changed_functions with this comparison_id to see detailed changes.",
            comparison_id,
            context.params.source_path,
            context.params.target_path,
            summary.total_functions,
            summary.added,
            summary.deleted,
            summary.modified,
            summary.renamed,
            summary.moved,
            summary.unchanged,
            summary.unchanged_moves
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// List changed functions
    async fn list_changed_functions(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: ComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let limit = args["limit"].as_u64().unwrap_or(100) as usize;
        let min_magnitude = args["min_magnitude"].as_f64();
        let change_types: Option<Vec<String>> = args["change_types"].as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        });

        let context = self.comparison_manager.get_comparison(comparison_id)?;
        let mut changes = context.get_sorted_changes();

        // Filter out unchanged moves (file reorganizations) by default
        changes.retain(|c| !c.is_unchanged_move);

        // Apply filters
        if let Some(types) = change_types {
            changes.retain(|c| types.contains(&c.change_type));
        }

        if let Some(min_mag) = min_magnitude {
            changes.retain(|c| c.change_magnitude >= min_mag);
        }

        // Apply limit
        changes.truncate(limit);

        // Format output
        let mut result_text = format!(
            "Changed Functions (showing {} of {}):\n\n",
            changes.len(),
            context.function_changes.len()
        );

        for (i, change) in changes.iter().enumerate() {
            result_text.push_str(&format!(
                "{}. {} - {} (magnitude: {:.2}, similarity: {:.2})\n",
                i + 1,
                change.function_name,
                change.change_type,
                change.change_magnitude,
                change.similarity_score
            ));

            if let Some(source_file) = &change.source_file {
                result_text.push_str(&format!(
                    "   Source: {} (lines {}-{})\n",
                    source_file,
                    change.source_start_line.unwrap_or(0),
                    change.source_end_line.unwrap_or(0)
                ));
            }

            if let Some(target_file) = &change.target_file {
                result_text.push_str(&format!(
                    "   Target: {} (lines {}-{})\n",
                    target_file,
                    change.target_start_line.unwrap_or(0),
                    change.target_end_line.unwrap_or(0)
                ));
            }

            if let Some(summary) = &change.diff_summary {
                result_text.push_str(&format!("   Summary: {}\n", summary));
            }

            result_text.push('\n');
        }

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Get function diff
    async fn get_function_diff(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: ComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let function_name = args["function_name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing function_name"))?;

        let include_content = args["include_content"].as_bool().unwrap_or(true);

        let context = self.comparison_manager.get_comparison(comparison_id)?;
        let change = context
            .get_function_change(function_name)
            .ok_or_else(|| anyhow::anyhow!("Function not found: {}", function_name))?;

        let mut result_text = format!(
            "Function: {}\n\
            Change Type: {}\n\
            Change Magnitude: {:.2}\n\
            Similarity Score: {:.2}\n\n",
            change.function_name,
            change.change_type,
            change.change_magnitude,
            change.similarity_score
        );

        if let Some(source_file) = &change.source_file {
            result_text.push_str(&format!(
                "Source File: {}\n\
                Source Lines: {}-{}\n",
                source_file,
                change.source_start_line.unwrap_or(0),
                change.source_end_line.unwrap_or(0)
            ));
            if let Some(sig) = &change.source_signature {
                result_text.push_str(&format!("Source Signature: {}\n", sig));
            }
        }

        if let Some(target_file) = &change.target_file {
            result_text.push_str(&format!(
                "\nTarget File: {}\n\
                Target Lines: {}-{}\n",
                target_file,
                change.target_start_line.unwrap_or(0),
                change.target_end_line.unwrap_or(0)
            ));
            if let Some(sig) = &change.target_signature {
                result_text.push_str(&format!("Target Signature: {}\n", sig));
            }
        }

        if let Some(summary) = &change.diff_summary {
            result_text.push_str(&format!("\nSummary: {}\n", summary));
        }

        // Include actual content if requested
        if include_content {
            if let Some(source_content) = &change.source_content {
                result_text.push_str(&format!("\n--- Source Content ---\n{}\n", source_content));
            }

            if let Some(target_content) = &change.target_content {
                result_text.push_str(&format!("\n+++ Target Content +++\n{}\n", target_content));
            }

            // Generate unified diff if both source and target exist
            if let (Some(source_content), Some(target_content)) =
                (&change.source_content, &change.target_content)
            {
                result_text.push_str("\n=== Unified Diff ===\n");
                let diff = self.generate_unified_diff(source_content, target_content);
                result_text.push_str(&diff);
            }
        }

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Generate a unified diff between two strings
    fn generate_unified_diff(&self, source: &str, target: &str) -> String {
        use similar::{ChangeTag, TextDiff};

        let diff = TextDiff::from_lines(source, target);
        let mut result = String::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            result.push_str(&format!("{}{}", sign, change));
        }

        result
    }

    /// Get comparison summary
    async fn get_comparison_summary(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: ComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let context = self.comparison_manager.get_comparison(comparison_id)?;
        let summary = context.get_summary();

        let result_text = format!(
            "Comparison Summary\n\
            ==================\n\n\
            Comparison ID: {}\n\
            Source: {}\n\
            Target: {}\n\
            Created: {}\n\n\
            Statistics:\n\
            - Total functions: {}\n\
            - Added: {}\n\
            - Deleted: {}\n\
            - Modified: {}\n\
            - Renamed: {}\n\
            - Moved: {}\n\
            - Unchanged: {}\n\
            - File reorganizations: {} (functions moved without changes)\n",
            comparison_id,
            context.params.source_path,
            context.params.target_path,
            context.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            summary.total_functions,
            summary.added,
            summary.deleted,
            summary.modified,
            summary.renamed,
            summary.moved,
            summary.unchanged,
            summary.unchanged_moves
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }
}
