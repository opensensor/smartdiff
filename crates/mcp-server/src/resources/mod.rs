//! MCP resources implementation

use crate::comparison::{ComparisonId, ComparisonManager};
use crate::mcp::protocol::{ResourceContents, ResourceInfo, ResourceTemplate};
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

/// Resource handler
pub struct ResourceHandler {
    comparison_manager: Arc<ComparisonManager>,
}

impl ResourceHandler {
    pub fn new(comparison_manager: Arc<ComparisonManager>) -> Self {
        Self {
            comparison_manager,
        }
    }

    /// List all available resources
    pub fn list_resources(&self) -> Result<Vec<ResourceInfo>> {
        let mut resources = Vec::new();

        // List all comparisons as resources
        let comparison_ids = self.comparison_manager.list_comparisons()?;

        for id in comparison_ids {
            let context = self.comparison_manager.get_comparison(id)?;

            // Add comparison summary resource
            resources.push(ResourceInfo {
                uri: format!("codediff://comparison/{}/summary", id),
                name: format!("Comparison {} Summary", id),
                title: Some(format!(
                    "{} vs {}",
                    context.params.source_path, context.params.target_path
                )),
                description: Some(format!(
                    "Summary of comparison between {} and {}",
                    context.params.source_path, context.params.target_path
                )),
                mime_type: Some("application/json".to_string()),
            });

            // Add functions list resource
            resources.push(ResourceInfo {
                uri: format!("codediff://comparison/{}/functions", id),
                name: format!("Comparison {} Functions", id),
                title: Some("Changed Functions List".to_string()),
                description: Some("List of all changed functions in this comparison".to_string()),
                mime_type: Some("application/json".to_string()),
            });
        }

        Ok(resources)
    }

    /// List resource templates
    pub fn list_templates(&self) -> Vec<ResourceTemplate> {
        vec![
            ResourceTemplate {
                uri_template: "codediff://comparison/{comparison_id}/summary".to_string(),
                name: "Comparison Summary".to_string(),
                title: Some("Comparison Summary".to_string()),
                description: Some(
                    "Get summary statistics for a specific comparison".to_string(),
                ),
                mime_type: Some("application/json".to_string()),
            },
            ResourceTemplate {
                uri_template: "codediff://comparison/{comparison_id}/functions".to_string(),
                name: "Changed Functions".to_string(),
                title: Some("Changed Functions List".to_string()),
                description: Some(
                    "Get list of all changed functions in a comparison".to_string(),
                ),
                mime_type: Some("application/json".to_string()),
            },
            ResourceTemplate {
                uri_template: "codediff://comparison/{comparison_id}/function/{function_name}"
                    .to_string(),
                name: "Function Diff".to_string(),
                title: Some("Individual Function Diff".to_string()),
                description: Some("Get detailed diff for a specific function".to_string()),
                mime_type: Some("application/json".to_string()),
            },
        ]
    }

    /// Read a resource
    pub fn read_resource(&self, uri: &str) -> Result<Vec<ResourceContents>> {
        info!("Reading resource: {}", uri);

        // Parse URI
        if !uri.starts_with("codediff://comparison/") {
            return Err(anyhow::anyhow!("Invalid URI scheme"));
        }

        let parts: Vec<&str> = uri.strip_prefix("codediff://comparison/")
            .unwrap()
            .split('/')
            .collect();

        if parts.is_empty() {
            return Err(anyhow::anyhow!("Invalid URI format"));
        }

        let comparison_id: ComparisonId =
            serde_json::from_str(&format!("\"{}\"", parts[0]))?;

        let context = self.comparison_manager.get_comparison(comparison_id)?;

        match parts.get(1) {
            Some(&"summary") => {
                let summary = context.get_summary();
                let json = serde_json::to_string_pretty(&summary)?;

                Ok(vec![ResourceContents {
                    uri: uri.to_string(),
                    name: "summary.json".to_string(),
                    title: Some("Comparison Summary".to_string()),
                    mime_type: Some("application/json".to_string()),
                    text: Some(json),
                    blob: None,
                }])
            }
            Some(&"functions") => {
                let changes = context.get_sorted_changes();
                let json = serde_json::to_string_pretty(&changes)?;

                Ok(vec![ResourceContents {
                    uri: uri.to_string(),
                    name: "functions.json".to_string(),
                    title: Some("Changed Functions".to_string()),
                    mime_type: Some("application/json".to_string()),
                    text: Some(json),
                    blob: None,
                }])
            }
            Some(&"function") => {
                if let Some(function_name) = parts.get(2) {
                    let change = context
                        .get_function_change(function_name)
                        .ok_or_else(|| anyhow::anyhow!("Function not found"))?;

                    let json = serde_json::to_string_pretty(&change)?;

                    Ok(vec![ResourceContents {
                        uri: uri.to_string(),
                        name: format!("{}.json", function_name),
                        title: Some(format!("Diff for {}", function_name)),
                        mime_type: Some("application/json".to_string()),
                        text: Some(json),
                        blob: None,
                    }])
                } else {
                    Err(anyhow::anyhow!("Missing function name in URI"))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown resource type")),
        }
    }
}

