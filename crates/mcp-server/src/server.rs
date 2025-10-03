//! Main MCP server implementation

use crate::comparison::ComparisonManager;
use crate::mcp::{
    messages::{ErrorCode, JsonRpcError, JsonRpcMessage, JsonRpcRequest, JsonRpcResponse},
    protocol::{
        CallToolRequest, InitializeParams, InitializeResult, ListResourcesResult, ListToolsResult,
        ReadResourceRequest, ReadResourceResult, ResourcesCapability, ServerCapabilities,
        ServerInfo, ToolsCapability,
    },
    transport::StdioTransport,
};
use crate::resources::ResourceHandler;
use crate::tools::ToolHandler;
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Main MCP server
pub struct McpServer {
    comparison_manager: Arc<ComparisonManager>,
    tool_handler: Arc<ToolHandler>,
    resource_handler: Arc<ResourceHandler>,
}

impl McpServer {
    pub fn new() -> Self {
        let comparison_manager = Arc::new(ComparisonManager::new());
        let tool_handler = Arc::new(ToolHandler::new(comparison_manager.clone()));
        let resource_handler = Arc::new(ResourceHandler::new(comparison_manager.clone()));

        Self {
            comparison_manager,
            tool_handler,
            resource_handler,
        }
    }

    /// Run the server with stdio transport (MCP standard)
    pub async fn run(self) -> Result<()> {
        info!("MCP Server starting with stdio transport...");

        let (mut transport, tx) = StdioTransport::new();

        info!("MCP Server ready, waiting for messages...");

        while let Some(message) = transport.recv().await {
            match message {
                JsonRpcMessage::Request(request) => {
                    let response = self.handle_request(request).await;
                    if let Err(e) = tx.send(JsonRpcMessage::Response(response)) {
                        error!("Failed to send response: {}", e);
                        break;
                    }
                }
                JsonRpcMessage::Notification(notification) => {
                    debug!("Received notification: {}", notification.method);
                    // Handle notifications if needed
                }
                JsonRpcMessage::Response(response) => {
                    warn!("Received unexpected response: {:?}", response);
                }
            }
        }

        info!("MCP Server shutting down");
        Ok(())
    }

    /// Handle a JSON-RPC request
    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        info!("Handling request: {}", request.method);

        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params).await,
            "ping" => self.handle_ping().await,
            "tools/list" => self.handle_list_tools().await,
            "tools/call" => self.handle_call_tool(request.params).await,
            "resources/list" => self.handle_list_resources().await,
            "resources/templates/list" => self.handle_list_resource_templates().await,
            "resources/read" => self.handle_read_resource(request.params).await,
            _ => Err(JsonRpcError::new(
                ErrorCode::MethodNotFound,
                format!("Method not found: {}", request.method),
            )),
        };

        match result {
            Ok(value) => JsonRpcResponse::success(request.id, value),
            Err(error) => JsonRpcResponse::error(request.id, error),
        }
    }

    /// Handle ping request (health check)
    async fn handle_ping(&self) -> Result<Value, JsonRpcError> {
        debug!("Handling ping request");
        Ok(serde_json::json!({}))
    }

    /// Handle initialize request
    async fn handle_initialize(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        info!("Initializing MCP server");

        let init_params: InitializeParams = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| {
                JsonRpcError::new(ErrorCode::InvalidParams, format!("Invalid params: {}", e))
            })?
        } else {
            return Err(JsonRpcError::new(
                ErrorCode::InvalidParams,
                "Missing initialization parameters".to_string(),
            ));
        };

        // Accept any protocol version from the client
        let result = InitializeResult {
            protocol_version: init_params.protocol_version,
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(false),
                    list_changed: Some(true),
                }),
                prompts: None,
            },
            server_info: ServerInfo {
                name: "smart-diff-mcp-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        serde_json::to_value(result).map_err(|e| {
            JsonRpcError::new(
                ErrorCode::InternalError,
                format!("Failed to serialize result: {}", e),
            )
        })
    }

    /// Handle list tools request
    async fn handle_list_tools(&self) -> Result<Value, JsonRpcError> {
        let tools = self.tool_handler.list_tools();

        let result = ListToolsResult { tools };

        serde_json::to_value(result).map_err(|e| {
            JsonRpcError::new(
                ErrorCode::InternalError,
                format!("Failed to serialize result: {}", e),
            )
        })
    }

    /// Handle call tool request
    async fn handle_call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let call_request: CallToolRequest = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| {
                JsonRpcError::new(ErrorCode::InvalidParams, format!("Invalid params: {}", e))
            })?
        } else {
            return Err(JsonRpcError::new(
                ErrorCode::InvalidParams,
                "Missing tool call parameters".to_string(),
            ));
        };

        let result = self
            .tool_handler
            .call_tool(&call_request.name, call_request.arguments)
            .await
            .map_err(|e| {
                JsonRpcError::new(
                    ErrorCode::ToolExecutionError,
                    format!("Tool execution failed: {}", e),
                )
            })?;

        serde_json::to_value(result).map_err(|e| {
            JsonRpcError::new(
                ErrorCode::InternalError,
                format!("Failed to serialize result: {}", e),
            )
        })
    }

    /// Handle list resources request
    async fn handle_list_resources(&self) -> Result<Value, JsonRpcError> {
        let resources = self.resource_handler.list_resources().map_err(|e| {
            JsonRpcError::new(
                ErrorCode::InternalError,
                format!("Failed to list resources: {}", e),
            )
        })?;

        let result = ListResourcesResult {
            resources,
            next_cursor: None,
        };

        serde_json::to_value(result).map_err(|e| {
            JsonRpcError::new(
                ErrorCode::InternalError,
                format!("Failed to serialize result: {}", e),
            )
        })
    }

    /// Handle list resource templates request
    async fn handle_list_resource_templates(&self) -> Result<Value, JsonRpcError> {
        let templates = self.resource_handler.list_templates();

        let result = serde_json::json!({
            "resourceTemplates": templates
        });

        Ok(result)
    }

    /// Handle read resource request
    async fn handle_read_resource(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let read_request: ReadResourceRequest = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| {
                JsonRpcError::new(ErrorCode::InvalidParams, format!("Invalid params: {}", e))
            })?
        } else {
            return Err(JsonRpcError::new(
                ErrorCode::InvalidParams,
                "Missing resource URI".to_string(),
            ));
        };

        let contents = self
            .resource_handler
            .read_resource(&read_request.uri)
            .map_err(|e| {
                JsonRpcError::new(
                    ErrorCode::ResourceNotFound,
                    format!("Failed to read resource: {}", e),
                )
            })?;

        let result = ReadResourceResult { contents };

        serde_json::to_value(result).map_err(|e| {
            JsonRpcError::new(
                ErrorCode::InternalError,
                format!("Failed to serialize result: {}", e),
            )
        })
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let _server = MCPServer::new();
    }
}
