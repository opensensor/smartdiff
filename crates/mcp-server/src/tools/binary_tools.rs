//! Binary comparison tools using Binary Ninja MCP client

use anyhow::Result;
use serde_json::{json, Value};
use smart_diff_binary_ninja_client::BinaryNinjaClient;
use smart_diff_engine::{BinaryFunctionInfo, BinaryFunctionMatch, BinaryFunctionMatcher, BinaryMatcherConfig};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};
use regex::Regex;

use crate::comparison::{
    BinaryComparisonContext, BinaryComparisonId, BinaryComparisonManager, BinaryComparisonParams,
};
use crate::mcp::protocol::{CallToolResult, ToolContent, ToolInfo};

/// Binary comparison tool handler
pub struct BinaryToolHandler {
    bn_client: BinaryNinjaClient,
    comparison_manager: Arc<Mutex<BinaryComparisonManager>>,
}

impl BinaryToolHandler {
    pub fn new() -> Self {
        Self {
            bn_client: BinaryNinjaClient::new(),
            comparison_manager: Arc::new(Mutex::new(BinaryComparisonManager::new())),
        }
    }

    /// List binary-specific tools
    pub fn list_tools(&self) -> Vec<ToolInfo> {
        vec![
            ToolInfo {
                name: "list_binja_servers".to_string(),
                description: "List available Binary Ninja MCP servers with loaded binaries. Each server represents a binary loaded in Binary Ninja.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            ToolInfo {
                name: "list_binary_functions".to_string(),
                description: "List all functions in a binary loaded in Binary Ninja.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "binary_id": {
                            "type": "string",
                            "description": "Binary server ID (e.g., 'port_9009')"
                        },
                        "search": {
                            "type": "string",
                            "description": "Optional search term to filter functions"
                        }
                    },
                    "required": ["binary_id"]
                }),
            },
            ToolInfo {
                name: "decompile_binary_function".to_string(),
                description: "Decompile a specific function from a binary and return the decompiled code.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "binary_id": {
                            "type": "string",
                            "description": "Binary server ID (e.g., 'port_9009')"
                        },
                        "function_name": {
                            "type": "string",
                            "description": "Name of the function to decompile"
                        }
                    },
                    "required": ["binary_id", "function_name"]
                }),
            },
            ToolInfo {
                name: "compare_binaries".to_string(),
                description: "Compare two binaries loaded in Binary Ninja and identify matching functions. Returns a comparison ID for querying results.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "binary_a_id": {
                            "type": "string",
                            "description": "Binary server ID for first binary (e.g., 'port_9009')"
                        },
                        "binary_b_id": {
                            "type": "string",
                            "description": "Binary server ID for second binary (e.g., 'port_9010')"
                        },
                        "use_decompiled_code": {
                            "type": "boolean",
                            "description": "Whether to use decompiled code for similarity comparison",
                            "default": false
                        },
                        "similarity_threshold": {
                            "type": "number",
                            "description": "Minimum similarity threshold (0.0 to 1.0)",
                            "default": 0.7,
                            "minimum": 0.0,
                            "maximum": 1.0
                        }
                    },
                    "required": ["binary_a_id", "binary_b_id"]
                }),
            },
            ToolInfo {
                name: "list_binary_matches".to_string(),
                description: "List matched functions from a binary comparison, sorted by similarity (most changed first).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "comparison_id": {
                            "type": "string",
                            "description": "The comparison ID returned from compare_binaries"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of matches to return",
                            "default": 1000
                        },
                        "min_similarity": {
                            "type": "number",
                            "description": "Minimum similarity score (0.0 to 1.0)",
                            "minimum": 0.0,
                            "maximum": 1.0
                        }
                    },
                    "required": ["comparison_id"]
                }),
            },
            ToolInfo {
                name: "get_binary_function_diff".to_string(),
                description: "Get detailed diff for a specific function match, including decompiled code from both binaries.".to_string(),
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
                        }
                    },
                    "required": ["comparison_id", "function_name"]
                }),
            },
            ToolInfo {
                name: "get_binary_comparison_summary".to_string(),
                description: "Get summary statistics for a binary comparison, including counts of added, deleted, modified, renamed, and moved functions.".to_string(),
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
            ToolInfo {
                name: "list_all_binary_functions".to_string(),
                description: "List all functions from a binary comparison, categorized as matched, added, or deleted. Supports filtering and search.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "comparison_id": {
                            "type": "string",
                            "description": "The comparison ID"
                        },
                        "category": {
                            "type": "string",
                            "enum": ["all", "matched", "added", "deleted"],
                            "description": "Filter by function category (default: all)"
                        },
                        "search": {
                            "type": "string",
                            "description": "Optional search term to filter function names"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of functions to return (default: 1000, max: 1000)"
                        },
                        "sort_by": {
                            "type": "string",
                            "enum": ["name", "similarity", "address"],
                            "description": "Sort functions by (default: similarity for matched, name for others)"
                        }
                    },
                    "required": ["comparison_id"]
                }),
            },
            ToolInfo {
                name: "analyze_binary_function_diff".to_string(),
                description: "Get enhanced analysis of binary function differences including constants, API calls, control flow, and data structure changes.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "comparison_id": {
                            "type": "string",
                            "description": "The comparison ID"
                        },
                        "function_name": {
                            "type": "string",
                            "description": "Name of the function to analyze"
                        },
                        "include_constants": {
                            "type": "boolean",
                            "description": "Include constant/magic number analysis (default: true)"
                        },
                        "include_api_calls": {
                            "type": "boolean",
                            "description": "Include API call analysis (default: true)"
                        },
                        "include_control_flow": {
                            "type": "boolean",
                            "description": "Include control flow analysis (default: true)"
                        }
                    },
                    "required": ["comparison_id", "function_name"]
                }),
            },
            ToolInfo {
                name: "search_binary_functions".to_string(),
                description: "Search for functions across a binary comparison with fuzzy matching and suggestions.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "comparison_id": {
                            "type": "string",
                            "description": "The comparison ID"
                        },
                        "query": {
                            "type": "string",
                            "description": "Function name or partial name to search for"
                        },
                        "fuzzy": {
                            "type": "boolean",
                            "description": "Enable fuzzy matching (default: true)"
                        },
                        "max_results": {
                            "type": "integer",
                            "description": "Maximum number of results (default: 100)"
                        }
                    },
                    "required": ["comparison_id", "query"]
                }),
            },
        ]
    }

    /// Execute a binary tool
    pub async fn call_tool(&self, name: &str, arguments: Option<Value>) -> Result<CallToolResult> {
        info!("Calling binary tool: {}", name);
        debug!("Arguments: {:?}", arguments);

        match name {
            "list_binja_servers" => self.list_binja_servers().await,
            "list_binary_functions" => self.list_binary_functions(arguments).await,
            "decompile_binary_function" => self.decompile_binary_function(arguments).await,
            "compare_binaries" => self.compare_binaries(arguments).await,
            "list_binary_matches" => self.list_binary_matches(arguments).await,
            "get_binary_function_diff" => self.get_binary_function_diff(arguments).await,
            "analyze_binary_function_diff" => self.analyze_binary_function_diff(arguments).await,
            "get_binary_comparison_summary" => self.get_binary_comparison_summary(arguments).await,
            "list_all_binary_functions" => self.list_all_binary_functions(arguments).await,
            "search_binary_functions" => self.search_binary_functions(arguments).await,
            _ => Err(anyhow::anyhow!("Unknown binary tool: {}", name)),
        }
    }

    /// List available Binary Ninja servers
    async fn list_binja_servers(&self) -> Result<CallToolResult> {
        info!("Discovering Binary Ninja servers...");

        let servers = self.bn_client.discover_servers().await?;

        if servers.is_empty() {
            return Ok(CallToolResult {
                content: vec![ToolContent::Text {
                    text: "No Binary Ninja servers found.\n\n\
                          Make sure:\n\
                          1. Binary Ninja is running\n\
                          2. You have loaded binaries\n\
                          3. MCP server is started for each binary (Plugins > MCP Server > Start Server for This Binary)"
                        .to_string(),
                }],
                is_error: Some(false),
            });
        }

        let mut result_text = format!("Found {} Binary Ninja server(s):\n\n", servers.len());

        for server in &servers {
            result_text.push_str(&format!(
                "Binary ID: {}\n\
                 Filename: {}\n\
                 Port: {}\n\
                 URL: {}\n\n",
                server.binary_id, server.filename, server.port, server.url
            ));
        }

        result_text.push_str("Use list_binary_functions with a binary_id to see functions in a binary.");

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// List functions in a binary
    async fn list_binary_functions(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let binary_id = args["binary_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing binary_id"))?;

        let search_term = args["search"].as_str();

        info!("Listing functions for binary: {}", binary_id);

        let functions = if let Some(search) = search_term {
            self.bn_client
                .search_functions(binary_id, search)
                .await?
        } else {
            self.bn_client.list_functions(binary_id).await?
        };

        let mut result_text = if let Some(search) = search_term {
            format!(
                "Found {} function(s) matching '{}' in {}:\n\n",
                functions.len(),
                search,
                binary_id
            )
        } else {
            format!(
                "Found {} function(s) in {}:\n\n",
                functions.len(),
                binary_id
            )
        };

        // Show first 100 functions
        for (i, func) in functions.iter().take(100).enumerate() {
            result_text.push_str(&format!("{}. {}\n", i + 1, func.name));
        }

        if functions.len() > 100 {
            result_text.push_str(&format!(
                "\n... and {} more functions (use search to filter)\n",
                functions.len() - 100
            ));
        }

        result_text.push_str("\nUse decompile_binary_function to see decompiled code for a specific function.");

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Decompile a function
    async fn decompile_binary_function(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let binary_id = args["binary_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing binary_id"))?;

        let function_name = args["function_name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing function_name"))?;

        info!(
            "Decompiling function '{}' from binary: {}",
            function_name, binary_id
        );

        let decompiled_code = self
            .bn_client
            .decompile_function(binary_id, function_name)
            .await?;

        let result_text = format!(
            "Decompiled code for function '{}':\n\n\
             ```c\n\
             {}\n\
             ```",
            function_name, decompiled_code
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Compare two binaries
    async fn compare_binaries(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let binary_a_id = args["binary_a_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing binary_a_id"))?;

        let binary_b_id = args["binary_b_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing binary_b_id"))?;

        let use_decompiled_code = args["use_decompiled_code"].as_bool().unwrap_or(false);
        let similarity_threshold = args["similarity_threshold"].as_f64().unwrap_or(0.7);

        info!(
            "Comparing binaries: {} vs {}",
            binary_a_id, binary_b_id
        );

        // Get binary info
        let info_a = self.bn_client.get_binary_info(binary_a_id).await?;
        let info_b = self.bn_client.get_binary_info(binary_b_id).await?;

        // Get function lists
        let functions_a_raw = self.bn_client.list_functions(binary_a_id).await?;
        let functions_b_raw = self.bn_client.list_functions(binary_b_id).await?;

        // Convert to BinaryFunctionInfo
        let functions_a: Vec<BinaryFunctionInfo> = functions_a_raw
            .into_iter()
            .map(|f| BinaryFunctionInfo::new(f.name, f.address))
            .collect();

        let functions_b: Vec<BinaryFunctionInfo> = functions_b_raw
            .into_iter()
            .map(|f| BinaryFunctionInfo::new(f.name, f.address))
            .collect();

        // Perform matching
        let config = BinaryMatcherConfig {
            match_threshold: similarity_threshold,
            enable_code_comparison: use_decompiled_code,
            ..Default::default()
        };

        let matcher = BinaryFunctionMatcher::with_config(config);
        let matches = matcher.match_functions(&functions_a, &functions_b)?;

        // Create comparison context
        let params = BinaryComparisonParams {
            binary_a_id: binary_a_id.to_string(),
            binary_b_id: binary_b_id.to_string(),
            binary_a_filename: info_a.filename,
            binary_b_filename: info_b.filename,
            use_decompiled_code,
            similarity_threshold,
        };

        let mut context = BinaryComparisonContext::new(params, matches);

        // Identify added/deleted functions
        let matched_a: std::collections::HashSet<_> =
            context.matches.iter().map(|m| &m.function_a.name).collect();
        let matched_b: std::collections::HashSet<_> =
            context.matches.iter().map(|m| &m.function_b.name).collect();

        context.deleted_functions = functions_a
            .iter()
            .filter(|f| !matched_a.contains(&f.name))
            .map(|f| f.name.clone())
            .collect();

        context.added_functions = functions_b
            .iter()
            .filter(|f| !matched_b.contains(&f.name))
            .map(|f| f.name.clone())
            .collect();

        // Store comparison
        let comparison_id = {
            let mut manager = self.comparison_manager.lock().unwrap();
            manager.store_comparison(context.clone())
        };

        let summary = context.get_summary();

        let result_text = format!(
            "Binary comparison created successfully!\n\n\
            Comparison ID: {}\n\
            Binary A: {}\n\
            Binary B: {}\n\n\
            Summary:\n\
            - Total matches: {}\n\
            - Exact name matches: {}\n\
            - Fuzzy name matches: {}\n\
            - Code similarity matches: {}\n\
            - Hybrid matches: {}\n\
            - Added functions: {}\n\
            - Deleted functions: {}\n\
            - Average similarity: {:.2}%\n\n\
            Use list_binary_matches with this comparison_id to see detailed matches.",
            comparison_id,
            summary.binary_a_filename,
            summary.binary_b_filename,
            summary.total_matches,
            summary.exact_matches,
            summary.fuzzy_matches,
            summary.code_matches,
            summary.hybrid_matches,
            summary.added_functions,
            summary.deleted_functions,
            summary.average_similarity * 100.0
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// List binary function matches
    async fn list_binary_matches(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: BinaryComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let limit = args["limit"].as_u64().unwrap_or(1000) as usize;
        let min_similarity = args["min_similarity"].as_f64();

        let manager = self.comparison_manager.lock().unwrap();
        let context = manager.get_comparison(comparison_id)?;

        let mut matches = context.get_sorted_matches();

        // Apply filters
        if let Some(min_sim) = min_similarity {
            matches.retain(|m| m.similarity >= min_sim);
        }

        // Limit results
        matches.truncate(limit);

        let mut result_text = format!(
            "Found {} function matches (sorted by similarity, most changed first):\n\n",
            matches.len()
        );

        for (i, m) in matches.iter().enumerate() {
            result_text.push_str(&format!(
                "{}. {} <-> {} (similarity: {:.1}%, type: {:?}, confidence: {:.1}%)\n",
                i + 1,
                m.function_a.name,
                m.function_b.name,
                m.similarity * 100.0,
                m.match_type,
                m.confidence * 100.0
            ));
        }

        result_text.push_str("\nUse get_binary_function_diff to see detailed diff for a specific function.");

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Get binary function diff
    async fn get_binary_function_diff(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: BinaryComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let function_name = args["function_name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing function_name"))?;

        let manager = self.comparison_manager.lock().unwrap();
        let context = manager.get_comparison(comparison_id)?;

        // First try to find in matched functions
        if let Some(m) = context.get_match_by_name(function_name) {
            return self.get_matched_function_diff(context, m).await;
        }

        // Check if it's an added function
        if context.added_functions.contains(&function_name.to_string()) {
            return self.get_added_function_diff(context, function_name).await;
        }

        // Check if it's a deleted function
        if context.deleted_functions.contains(&function_name.to_string()) {
            return self.get_deleted_function_diff(context, function_name).await;
        }

        // Function not found - provide helpful error with suggestions
        self.function_not_found_error(context, function_name).await
    }

    /// Get diff for a matched function
    async fn get_matched_function_diff(
        &self,
        context: &BinaryComparisonContext,
        m: &BinaryFunctionMatch,
    ) -> Result<CallToolResult> {
        // Get decompiled code for both functions
        let code_a = self
            .bn_client
            .decompile_function(&context.params.binary_a_id, &m.function_a.name)
            .await?;

        let code_b = self
            .bn_client
            .decompile_function(&context.params.binary_b_id, &m.function_b.name)
            .await?;

        let result_text = format!(
            "Function Diff: {} <-> {}\n\
            Similarity: {:.1}%\n\
            Match Type: {:?}\n\
            Confidence: {:.1}%\n\n\
            === Binary A: {} ===\n\
            Address: {}\n\
            ```c\n\
            {}\n\
            ```\n\n\
            === Binary B: {} ===\n\
            Address: {}\n\
            ```c\n\
            {}\n\
            ```",
            m.function_a.name,
            m.function_b.name,
            m.similarity * 100.0,
            m.match_type,
            m.confidence * 100.0,
            context.params.binary_a_filename,
            m.function_a.address,
            code_a,
            context.params.binary_b_filename,
            m.function_b.address,
            code_b
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Get diff for an added function (only in binary B)
    async fn get_added_function_diff(
        &self,
        context: &BinaryComparisonContext,
        function_name: &str,
    ) -> Result<CallToolResult> {
        // Get function info from binary B
        let functions_b = self.bn_client.list_functions(&context.params.binary_b_id).await?;
        let function_b = functions_b
            .iter()
            .find(|f| f.name == function_name)
            .ok_or_else(|| anyhow::anyhow!("Function {} not found in binary B", function_name))?;

        let code_b = self
            .bn_client
            .decompile_function(&context.params.binary_b_id, function_name)
            .await?;

        let result_text = format!(
            "Function Diff: {} (ADDED)\n\
            Status: New function in {}\n\
            This function does not exist in {}\n\n\
            === Binary B: {} ===\n\
            Address: {}\n\
            ```c\n\
            {}\n\
            ```\n\n\
            === Binary A: {} ===\n\
            Status: Function does not exist\n\n\
            This is a newly added function in the target binary.",
            function_name,
            context.params.binary_b_filename,
            context.params.binary_a_filename,
            context.params.binary_b_filename,
            function_b.address,
            code_b,
            context.params.binary_a_filename
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Get diff for a deleted function (only in binary A)
    async fn get_deleted_function_diff(
        &self,
        context: &BinaryComparisonContext,
        function_name: &str,
    ) -> Result<CallToolResult> {
        // Get function info from binary A
        let functions_a = self.bn_client.list_functions(&context.params.binary_a_id).await?;
        let function_a = functions_a
            .iter()
            .find(|f| f.name == function_name)
            .ok_or_else(|| anyhow::anyhow!("Function {} not found in binary A", function_name))?;

        let code_a = self
            .bn_client
            .decompile_function(&context.params.binary_a_id, function_name)
            .await?;

        let result_text = format!(
            "Function Diff: {} (DELETED)\n\
            Status: Function removed from {}\n\
            This function does not exist in {}\n\n\
            === Binary A: {} ===\n\
            Address: {}\n\
            ```c\n\
            {}\n\
            ```\n\n\
            === Binary B: {} ===\n\
            Status: Function does not exist\n\n\
            This function was removed in the target binary.",
            function_name,
            context.params.binary_a_filename,
            context.params.binary_b_filename,
            context.params.binary_a_filename,
            function_a.address,
            code_a,
            context.params.binary_b_filename
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Handle function not found error with helpful suggestions
    async fn function_not_found_error(
        &self,
        context: &BinaryComparisonContext,
        function_name: &str,
    ) -> Result<CallToolResult> {
        // Find similar function names for suggestions
        let mut suggestions = Vec::new();

        // Helper function to calculate simple similarity
        let similarity = |s1: &str, s2: &str| -> f64 {
            let s1_lower = s1.to_lowercase();
            let s2_lower = s2.to_lowercase();

            if s1_lower.contains(&s2_lower) || s2_lower.contains(&s1_lower) {
                let longer = s1_lower.len().max(s2_lower.len()) as f64;
                let shorter = s1_lower.len().min(s2_lower.len()) as f64;
                return shorter / longer;
            }

            // Count common characters
            let common = s1_lower.chars()
                .filter(|c| s2_lower.contains(*c))
                .count() as f64;
            let max_len = s1_lower.len().max(s2_lower.len()) as f64;
            common / max_len
        };

        // Check matched functions
        for m in &context.matches {
            let sim_a = similarity(&m.function_a.name, function_name);
            let sim_b = similarity(&m.function_b.name, function_name);
            if sim_a > 0.5 {
                suggestions.push((m.function_a.name.clone(), sim_a, "matched".to_string()));
            }
            if sim_b > 0.5 && m.function_b.name != m.function_a.name {
                suggestions.push((m.function_b.name.clone(), sim_b, "matched".to_string()));
            }
        }

        // Check added functions
        for name in &context.added_functions {
            let sim = similarity(name, function_name);
            if sim > 0.5 {
                suggestions.push((name.clone(), sim, "added".to_string()));
            }
        }

        // Check deleted functions
        for name in &context.deleted_functions {
            let sim = similarity(name, function_name);
            if sim > 0.5 {
                suggestions.push((name.clone(), sim, "deleted".to_string()));
            }
        }

        // Sort suggestions by similarity
        suggestions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        suggestions.truncate(5);

        let mut result_text = format!(
            "Function '{}' not found in comparison: {}\n\n\
            The function was not found in:\n\
            - {} matched functions\n\
            - {} added functions (new in {})\n\
            - {} deleted functions (removed from {})\n\n",
            function_name,
            context.id,
            context.matches.len(),
            context.added_functions.len(),
            context.params.binary_b_filename,
            context.deleted_functions.len(),
            context.params.binary_a_filename
        );

        if !suggestions.is_empty() {
            result_text.push_str("Did you mean one of these similar functions?\n");
            for (name, sim, category) in suggestions {
                result_text.push_str(&format!(
                    "- {} [{}] (similarity: {:.1}%)\n",
                    name, category.to_uppercase(), sim * 100.0
                ));
            }
            result_text.push('\n');
        }

        result_text.push_str(
            "Suggestions:\n\
            - Use search_binary_functions to find functions with fuzzy matching\n\
            - Use list_all_binary_functions to browse all functions by category\n\
            - Check the exact spelling and case of the function name\n\
            - The function might have been renamed between versions"
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(true),
        })
    }

    /// Enhanced binary function diff analysis
    async fn analyze_binary_function_diff(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let function_name = args["function_name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing function_name"))?;

        let include_constants = args["include_constants"].as_bool().unwrap_or(true);
        let include_api_calls = args["include_api_calls"].as_bool().unwrap_or(true);
        let include_control_flow = args["include_control_flow"].as_bool().unwrap_or(true);

        let comparison_id: BinaryComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let manager = self.comparison_manager.lock().unwrap();
        let context = manager.get_comparison(comparison_id)?;

        // Find the function match
        let m = context.get_match_by_name(function_name)
            .ok_or_else(|| anyhow::anyhow!("Function {} not found in comparison", function_name))?;

        // Get decompiled code for both functions
        let code_a = self
            .bn_client
            .decompile_function(&context.params.binary_a_id, &m.function_a.name)
            .await?;

        let code_b = self
            .bn_client
            .decompile_function(&context.params.binary_b_id, &m.function_b.name)
            .await?;

        // Perform enhanced analysis
        let mut analysis_results = Vec::new();

        if include_constants {
            let constants_analysis = self.analyze_constants(&code_a, &code_b);
            analysis_results.push(constants_analysis);
        }

        if include_api_calls {
            let api_calls_analysis = self.analyze_api_calls(&code_a, &code_b);
            analysis_results.push(api_calls_analysis);
        }

        if include_control_flow {
            let control_flow_analysis = self.analyze_control_flow(&code_a, &code_b);
            analysis_results.push(control_flow_analysis);
        }

        let result_text = format!(
            "Enhanced Binary Function Analysis: {} <-> {}\n\
            Similarity: {:.1}%\n\
            Match Type: {:?}\n\
            Confidence: {:.1}%\n\n\
            === Binary A: {} ===\n\
            Address: {}\n\n\
            === Binary B: {} ===\n\
            Address: {}\n\n\
            {}",
            m.function_a.name,
            m.function_b.name,
            m.similarity * 100.0,
            m.match_type,
            m.confidence * 100.0,
            context.params.binary_a_filename,
            m.function_a.address,
            context.params.binary_b_filename,
            m.function_b.address,
            analysis_results.join("\n\n")
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Get comparison summary
    async fn get_binary_comparison_summary(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: BinaryComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let manager = self.comparison_manager.lock().unwrap();
        let context = manager.get_comparison(comparison_id)?;

        let summary = context.get_summary();

        // Calculate additional statistics
        let total_functions_a = context.matches.len() + context.deleted_functions.len();
        let total_functions_b = context.matches.len() + context.added_functions.len();
        let net_function_change = context.added_functions.len() as i32 - context.deleted_functions.len() as i32;

        // Calculate similarity distribution
        let mut high_similarity = 0;  // > 95%
        let mut medium_similarity = 0; // 70-95%
        let mut low_similarity = 0;   // < 70%

        for m in &context.matches {
            if m.similarity > 0.95 {
                high_similarity += 1;
            } else if m.similarity >= 0.70 {
                medium_similarity += 1;
            } else {
                low_similarity += 1;
            }
        }

        // Find most and least similar functions
        let mut sorted_matches = context.matches.clone();
        sorted_matches.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        let most_similar = sorted_matches.first();
        let least_similar = sorted_matches.last();

        let mut result_text = format!(
            "Binary Comparison Summary\n\n\
            Comparison ID: {}\n\
            Binary A: {} ({} functions)\n\
            Binary B: {} ({} functions)\n\
            Created: {}\n\n\
            === OVERVIEW ===\n\
            Net function change: {:+} functions\n\
            Total unique functions: {} (A) + {} (B) = {} total\n\n\
            === FUNCTION MATCHES ({}) ===\n\
            - Exact name matches: {} ({:.1}%)\n\
            - Fuzzy name matches: {} ({:.1}%)\n\
            - Code similarity matches: {} ({:.1}%)\n\
            - Hybrid matches: {} ({:.1}%)\n\n\
            === FUNCTION CHANGES ===\n\
            - Added functions: {} (new in {})\n\
            - Deleted functions: {} (removed from {})\n\n\
            === SIMILARITY ANALYSIS ===\n\
            - Average similarity: {:.2}%\n\
            - High similarity (>95%): {} functions\n\
            - Medium similarity (70-95%): {} functions\n\
            - Low similarity (<70%): {} functions\n",
            summary.comparison_id,
            summary.binary_a_filename, total_functions_a,
            summary.binary_b_filename, total_functions_b,
            context.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
            net_function_change,
            total_functions_a, total_functions_b, total_functions_a + total_functions_b,
            summary.total_matches,
            summary.exact_matches, if summary.total_matches > 0 { summary.exact_matches as f64 / summary.total_matches as f64 * 100.0 } else { 0.0 },
            summary.fuzzy_matches, if summary.total_matches > 0 { summary.fuzzy_matches as f64 / summary.total_matches as f64 * 100.0 } else { 0.0 },
            summary.code_matches, if summary.total_matches > 0 { summary.code_matches as f64 / summary.total_matches as f64 * 100.0 } else { 0.0 },
            summary.hybrid_matches, if summary.total_matches > 0 { summary.hybrid_matches as f64 / summary.total_matches as f64 * 100.0 } else { 0.0 },
            summary.added_functions, summary.binary_b_filename,
            summary.deleted_functions, summary.binary_a_filename,
            summary.average_similarity * 100.0,
            high_similarity,
            medium_similarity,
            low_similarity
        );

        // Add most/least similar function info
        if let Some(most) = most_similar {
            result_text.push_str(&format!(
                "\n=== MOST SIMILAR FUNCTION ===\n\
                {} <-> {} ({:.1}% similarity)\n",
                most.function_a.name, most.function_b.name, most.similarity * 100.0
            ));
        }

        if let Some(least) = least_similar {
            if least.similarity < 1.0 {
                result_text.push_str(&format!(
                    "\n=== LEAST SIMILAR FUNCTION ===\n\
                    {} <-> {} ({:.1}% similarity)\n",
                    least.function_a.name, least.function_b.name, least.similarity * 100.0
                ));
            }
        }

        result_text.push_str(
            "\n=== AVAILABLE TOOLS ===\n\
            - list_all_binary_functions: Browse functions by category\n\
            - search_binary_functions: Search with fuzzy matching\n\
            - list_binary_matches: See detailed match list\n\
            - get_binary_function_diff: Get detailed function diffs"
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// List all binary functions with categorization
    async fn list_all_binary_functions(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: BinaryComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let category = args["category"].as_str().unwrap_or("all");
        let search = args["search"].as_str();
        let limit = args["limit"].as_u64().unwrap_or(1000).min(1000) as usize;
        let sort_by = args["sort_by"].as_str().unwrap_or("similarity");

        let manager = self.comparison_manager.lock().unwrap();
        let context = manager.get_comparison(comparison_id)?;

        let mut result_text = format!(
            "Binary Functions - Comparison: {}\n\
            Binary A: {}\n\
            Binary B: {}\n\n",
            comparison_id,
            context.params.binary_a_filename,
            context.params.binary_b_filename
        );

        let mut total_shown = 0;

        // Helper function to filter by search term
        let matches_search = |name: &str, search_term: Option<&str>| -> bool {
            match search_term {
                Some(term) => name.to_lowercase().contains(&term.to_lowercase()),
                None => true,
            }
        };

        // Show matched functions
        if category == "all" || category == "matched" {
            let mut matched_functions: Vec<_> = context.matches.iter().collect();

            // Sort matched functions
            match sort_by {
                "name" => matched_functions.sort_by(|a, b| a.function_a.name.cmp(&b.function_a.name)),
                "address" => matched_functions.sort_by(|a, b| a.function_a.address.cmp(&b.function_a.address)),
                _ => matched_functions.sort_by(|a, b| a.similarity.partial_cmp(&b.similarity).unwrap()),
            }

            let filtered_matched: Vec<_> = matched_functions
                .into_iter()
                .filter(|m| matches_search(&m.function_a.name, search))
                .take(limit.saturating_sub(total_shown))
                .collect();

            if !filtered_matched.is_empty() {
                result_text.push_str(&format!("=== MATCHED FUNCTIONS ({}) ===\n", filtered_matched.len()));
                for (i, m) in filtered_matched.iter().enumerate() {
                    result_text.push_str(&format!(
                        "{}. {} <-> {} (similarity: {:.1}%, type: {:?})\n   A: {} | B: {}\n",
                        i + 1,
                        m.function_a.name,
                        m.function_b.name,
                        m.similarity * 100.0,
                        m.match_type,
                        m.function_a.address,
                        m.function_b.address
                    ));
                }
                result_text.push('\n');
                total_shown += filtered_matched.len();
            }
        }

        // Show added functions (only in binary B)
        if (category == "all" || category == "added") && total_shown < limit {
            let mut added_functions: Vec<_> = context.added_functions.iter().collect();
            added_functions.sort();

            let filtered_added: Vec<_> = added_functions
                .into_iter()
                .filter(|name| matches_search(name, search))
                .take(limit.saturating_sub(total_shown))
                .collect();

            if !filtered_added.is_empty() {
                result_text.push_str(&format!("=== ADDED FUNCTIONS ({}) ===\n", filtered_added.len()));
                for (i, name) in filtered_added.iter().enumerate() {
                    result_text.push_str(&format!("{}. {} (new in {})\n", i + 1, name, context.params.binary_b_filename));
                }
                result_text.push('\n');
                total_shown += filtered_added.len();
            }
        }

        // Show deleted functions (only in binary A)
        if (category == "all" || category == "deleted") && total_shown < limit {
            let mut deleted_functions: Vec<_> = context.deleted_functions.iter().collect();
            deleted_functions.sort();

            let filtered_deleted: Vec<_> = deleted_functions
                .into_iter()
                .filter(|name| matches_search(name, search))
                .take(limit.saturating_sub(total_shown))
                .collect();

            if !filtered_deleted.is_empty() {
                result_text.push_str(&format!("=== DELETED FUNCTIONS ({}) ===\n", filtered_deleted.len()));
                for (i, name) in filtered_deleted.iter().enumerate() {
                    result_text.push_str(&format!("{}. {} (removed from {})\n", i + 1, name, context.params.binary_a_filename));
                }
                result_text.push('\n');
                total_shown += filtered_deleted.len();
            }
        }

        result_text.push_str(&format!(
            "Showing {} functions (limit: {})\n\
            Use search_binary_functions for fuzzy search or get_binary_function_diff for detailed diffs.",
            total_shown, limit
        ));

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Search binary functions with fuzzy matching
    async fn search_binary_functions(&self, arguments: Option<Value>) -> Result<CallToolResult> {
        let args = arguments.ok_or_else(|| anyhow::anyhow!("Missing arguments"))?;

        let comparison_id_str = args["comparison_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing comparison_id"))?;

        let comparison_id: BinaryComparisonId =
            serde_json::from_str(&format!("\"{}\"", comparison_id_str))?;

        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query"))?;

        let fuzzy = args["fuzzy"].as_bool().unwrap_or(true);
        let max_results = args["max_results"].as_u64().unwrap_or(100) as usize;

        let manager = self.comparison_manager.lock().unwrap();
        let context = manager.get_comparison(comparison_id)?;

        let mut result_text = format!(
            "Function Search Results for '{}'\n\
            Comparison: {}\n\
            Fuzzy matching: {}\n\n",
            query, comparison_id, fuzzy
        );

        #[derive(Debug)]
        struct SearchResult {
            name: String,
            category: String,
            similarity: f64,
            details: String,
        }

        let mut results = Vec::new();
        let _query_lower = query.to_lowercase();

        // Helper function to calculate string similarity
        let calculate_similarity = |s1: &str, s2: &str| -> f64 {
            if s1 == s2 {
                return 1.0;
            }

            let s1_lower = s1.to_lowercase();
            let s2_lower = s2.to_lowercase();

            // Exact match (case insensitive)
            if s1_lower == s2_lower {
                return 0.95;
            }

            // Contains match
            if s1_lower.contains(&s2_lower) || s2_lower.contains(&s1_lower) {
                let longer = s1_lower.len().max(s2_lower.len()) as f64;
                let shorter = s1_lower.len().min(s2_lower.len()) as f64;
                return 0.7 + (shorter / longer) * 0.2;
            }

            if fuzzy {
                // Simple fuzzy matching based on common characters
                let common_chars = s1_lower.chars()
                    .filter(|c| s2_lower.contains(*c))
                    .count() as f64;
                let max_len = s1_lower.len().max(s2_lower.len()) as f64;
                return common_chars / max_len * 0.6;
            }

            0.0
        };

        // Search in matched functions
        for m in &context.matches {
            let sim_a = calculate_similarity(&m.function_a.name, query);
            let sim_b = calculate_similarity(&m.function_b.name, query);
            let max_sim = sim_a.max(sim_b);

            if max_sim > 0.3 || (!fuzzy && max_sim > 0.0) {
                results.push(SearchResult {
                    name: m.function_a.name.clone(),
                    category: "matched".to_string(),
                    similarity: max_sim,
                    details: format!(
                        "{} <-> {} (match similarity: {:.1}%, type: {:?})",
                        m.function_a.name,
                        m.function_b.name,
                        m.similarity * 100.0,
                        m.match_type
                    ),
                });
            }
        }

        // Search in added functions
        for name in &context.added_functions {
            let sim = calculate_similarity(name, query);
            if sim > 0.3 || (!fuzzy && sim > 0.0) {
                results.push(SearchResult {
                    name: name.clone(),
                    category: "added".to_string(),
                    similarity: sim,
                    details: format!("{} (added in {})", name, context.params.binary_b_filename),
                });
            }
        }

        // Search in deleted functions
        for name in &context.deleted_functions {
            let sim = calculate_similarity(name, query);
            if sim > 0.3 || (!fuzzy && sim > 0.0) {
                results.push(SearchResult {
                    name: name.clone(),
                    category: "deleted".to_string(),
                    similarity: sim,
                    details: format!("{} (deleted from {})", name, context.params.binary_a_filename),
                });
            }
        }

        // Sort by similarity (highest first)
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        results.truncate(max_results);

        if results.is_empty() {
            result_text.push_str("No functions found matching the query.\n\n");

            // Provide suggestions
            result_text.push_str("Suggestions:\n");
            result_text.push_str("- Try a shorter or more general search term\n");
            result_text.push_str("- Use list_all_binary_functions to browse all functions\n");
            result_text.push_str("- Check the function name spelling\n");
        } else {
            result_text.push_str(&format!("Found {} matching functions:\n\n", results.len()));

            for (i, result) in results.iter().enumerate() {
                result_text.push_str(&format!(
                    "{}. [{}] {} (similarity: {:.1}%)\n   {}\n\n",
                    i + 1,
                    result.category.to_uppercase(),
                    result.name,
                    result.similarity * 100.0,
                    result.details
                ));
            }
        }

        result_text.push_str("Use get_binary_function_diff with the exact function name to see detailed differences.");

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: result_text }],
            is_error: Some(false),
        })
    }

    /// Analyze constants and magic numbers in function code
    fn analyze_constants(&self, code_a: &str, code_b: &str) -> String {
        let mut result = String::from("=== CONSTANTS & MAGIC NUMBERS ANALYSIS ===\n");

        // Extract hex constants (potential ioctl numbers, error codes, etc.)
        let hex_regex = Regex::new(r"0x[0-9a-fA-F]+").unwrap();
        let decimal_regex = Regex::new(r"\b\d{3,}\b").unwrap(); // Numbers with 3+ digits

        let constants_a: HashSet<_> = hex_regex.find_iter(code_a)
            .chain(decimal_regex.find_iter(code_a))
            .map(|m| m.as_str())
            .collect();

        let constants_b: HashSet<_> = hex_regex.find_iter(code_b)
            .chain(decimal_regex.find_iter(code_b))
            .map(|m| m.as_str())
            .collect();

        let added_constants: Vec<_> = constants_b.difference(&constants_a).collect();
        let removed_constants: Vec<_> = constants_a.difference(&constants_b).collect();
        let common_constants: Vec<_> = constants_a.intersection(&constants_b).collect();

        if !added_constants.is_empty() {
            result.push_str(&format!("Added Constants ({}): {}\n",
                added_constants.len(),
                added_constants.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
        }

        if !removed_constants.is_empty() {
            result.push_str(&format!("Removed Constants ({}): {}\n",
                removed_constants.len(),
                removed_constants.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
        }

        result.push_str(&format!("Common Constants ({}): {}\n",
            common_constants.len(),
            if common_constants.len() > 10 {
                format!("{} and {} more...",
                    common_constants.iter().take(10).map(|s| s.to_string()).collect::<Vec<_>>().join(", "),
                    common_constants.len() - 10)
            } else {
                common_constants.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
            }));

        // Analyze potential ioctl numbers (typically 0x4xxx, 0x8xxx patterns)
        let ioctl_pattern = Regex::new(r"0x[48][0-9a-fA-F]{3,7}").unwrap();
        let ioctl_a: HashSet<_> = ioctl_pattern.find_iter(code_a).map(|m| m.as_str()).collect();
        let ioctl_b: HashSet<_> = ioctl_pattern.find_iter(code_b).map(|m| m.as_str()).collect();

        if !ioctl_a.is_empty() || !ioctl_b.is_empty() {
            result.push_str("\nPotential IOCTL Numbers:\n");
            if !ioctl_a.is_empty() {
                result.push_str(&format!("  Binary A: {}\n", ioctl_a.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
            }
            if !ioctl_b.is_empty() {
                result.push_str(&format!("  Binary B: {}\n", ioctl_b.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
            }

            let ioctl_changes: Vec<_> = ioctl_b.difference(&ioctl_a).collect();
            if !ioctl_changes.is_empty() {
                result.push_str(&format!("  ⚠️  IOCTL CHANGES DETECTED: {}\n",
                    ioctl_changes.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
            }
        }

        result
    }

    /// Analyze API calls and function invocations
    fn analyze_api_calls(&self, code_a: &str, code_b: &str) -> String {
        let mut result = String::from("=== API CALLS ANALYSIS ===\n");

        // Extract function calls (pattern: word followed by parentheses)
        let call_regex = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();

        let calls_a: HashSet<_> = call_regex.captures_iter(code_a)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

        let calls_b: HashSet<_> = call_regex.captures_iter(code_b)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

        let added_calls: Vec<_> = calls_b.difference(&calls_a).collect();
        let removed_calls: Vec<_> = calls_a.difference(&calls_b).collect();
        let common_calls: Vec<_> = calls_a.intersection(&calls_b).collect();

        if !added_calls.is_empty() {
            result.push_str(&format!("Added API Calls ({}): {}\n",
                added_calls.len(),
                added_calls.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
        }

        if !removed_calls.is_empty() {
            result.push_str(&format!("Removed API Calls ({}): {}\n",
                removed_calls.len(),
                removed_calls.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")));
        }

        result.push_str(&format!("Common API Calls ({}): {}\n",
            common_calls.len(),
            if common_calls.len() > 15 {
                format!("{} and {} more...",
                    common_calls.iter().take(15).map(|s| s.to_string()).collect::<Vec<_>>().join(", "),
                    common_calls.len() - 15)
            } else {
                common_calls.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
            }));

        // Analyze parameter count changes for common functions
        let mut param_changes = Vec::new();
        for call in &common_calls {
            let pattern = format!(r"\b{}\s*\(([^)]*)\)", regex::escape(call));
            let call_regex = Regex::new(&pattern).unwrap();

            let params_a: Vec<_> = call_regex.captures_iter(code_a)
                .map(|cap| cap.get(1).unwrap().as_str().split(',').filter(|s| !s.trim().is_empty()).count())
                .collect();
            let params_b: Vec<_> = call_regex.captures_iter(code_b)
                .map(|cap| cap.get(1).unwrap().as_str().split(',').filter(|s| !s.trim().is_empty()).count())
                .collect();

            if !params_a.is_empty() && !params_b.is_empty() {
                let avg_params_a = params_a.iter().sum::<usize>() as f64 / params_a.len() as f64;
                let avg_params_b = params_b.iter().sum::<usize>() as f64 / params_b.len() as f64;

                if (avg_params_a - avg_params_b).abs() > 0.5 {
                    param_changes.push(format!("{}(): {:.1} → {:.1} params",
                        call, avg_params_a, avg_params_b));
                }
            }
        }

        if !param_changes.is_empty() {
            result.push_str(&format!("\nParameter Count Changes:\n  {}\n",
                param_changes.join("\n  ")));
        }

        result
    }

    /// Analyze control flow structures
    fn analyze_control_flow(&self, code_a: &str, code_b: &str) -> String {
        let mut result = String::from("=== CONTROL FLOW ANALYSIS ===\n");

        // Count control flow structures
        let if_count_a = code_a.matches("if (").count();
        let if_count_b = code_b.matches("if (").count();
        let for_count_a = code_a.matches("for (").count();
        let for_count_b = code_b.matches("for (").count();
        let while_count_a = code_a.matches("while (").count();
        let while_count_b = code_b.matches("while (").count();
        let switch_count_a = code_a.matches("switch (").count();
        let switch_count_b = code_b.matches("switch (").count();

        result.push_str(&format!("Control Flow Structure Changes:\n"));
        result.push_str(&format!("  if statements: {} → {} ({})\n",
            if_count_a, if_count_b,
            if if_count_b > if_count_a { format!("+{}", if_count_b - if_count_a) }
            else if if_count_a > if_count_b { format!("-{}", if_count_a - if_count_b) }
            else { "no change".to_string() }));

        result.push_str(&format!("  for loops: {} → {} ({})\n",
            for_count_a, for_count_b,
            if for_count_b > for_count_a { format!("+{}", for_count_b - for_count_a) }
            else if for_count_a > for_count_b { format!("-{}", for_count_a - for_count_b) }
            else { "no change".to_string() }));

        result.push_str(&format!("  while loops: {} → {} ({})\n",
            while_count_a, while_count_b,
            if while_count_b > while_count_a { format!("+{}", while_count_b - while_count_a) }
            else if while_count_a > while_count_b { format!("-{}", while_count_a - while_count_b) }
            else { "no change".to_string() }));

        result.push_str(&format!("  switch statements: {} → {} ({})\n",
            switch_count_a, switch_count_b,
            if switch_count_b > switch_count_a { format!("+{}", switch_count_b - switch_count_a) }
            else if switch_count_a > switch_count_b { format!("-{}", switch_count_a - switch_count_b) }
            else { "no change".to_string() }));

        // Analyze complexity changes
        let total_structures_a = if_count_a + for_count_a + while_count_a + switch_count_a;
        let total_structures_b = if_count_b + for_count_b + while_count_b + switch_count_b;

        result.push_str(&format!("\nComplexity Analysis:\n"));
        result.push_str(&format!("  Total control structures: {} → {} ({})\n",
            total_structures_a, total_structures_b,
            if total_structures_b > total_structures_a {
                format!("+{} (increased complexity)", total_structures_b - total_structures_a)
            } else if total_structures_a > total_structures_b {
                format!("-{} (reduced complexity)", total_structures_a - total_structures_b)
            } else {
                "no change".to_string()
            }));

        // Analyze nesting depth (rough estimate)
        let max_brace_depth_a = self.calculate_max_brace_depth(code_a);
        let max_brace_depth_b = self.calculate_max_brace_depth(code_b);

        result.push_str(&format!("  Max nesting depth: {} → {} ({})\n",
            max_brace_depth_a, max_brace_depth_b,
            if max_brace_depth_b > max_brace_depth_a {
                format!("+{} (deeper nesting)", max_brace_depth_b - max_brace_depth_a)
            } else if max_brace_depth_a > max_brace_depth_b {
                format!("-{} (flatter structure)", max_brace_depth_a - max_brace_depth_b)
            } else {
                "no change".to_string()
            }));

        result
    }

    /// Calculate maximum brace nesting depth
    fn calculate_max_brace_depth(&self, code: &str) -> usize {
        let mut max_depth = 0;
        let mut current_depth = 0;

        for ch in code.chars() {
            match ch {
                '{' => {
                    current_depth += 1;
                    max_depth = max_depth.max(current_depth);
                }
                '}' => {
                    if current_depth > 0 {
                        current_depth -= 1;
                    }
                }
                _ => {}
            }
        }

        max_depth
    }
}

impl Default for BinaryToolHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tool_handler_creation() {
        let handler = BinaryToolHandler::new();
        let tools = handler.list_tools();
        assert_eq!(tools.len(), 9);
        assert_eq!(tools[0].name, "list_binja_servers");
        assert_eq!(tools[1].name, "list_binary_functions");
        assert_eq!(tools[2].name, "decompile_binary_function");
        assert_eq!(tools[3].name, "compare_binaries");
        assert_eq!(tools[4].name, "list_binary_matches");
        assert_eq!(tools[5].name, "get_binary_function_diff");
        assert_eq!(tools[6].name, "analyze_binary_function_diff");
        assert_eq!(tools[7].name, "get_binary_comparison_summary");
        assert_eq!(tools[8].name, "list_all_binary_functions");
    }
}

