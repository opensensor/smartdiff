//! Binary Ninja MCP HTTP client implementation

use crate::types::{BinaryInfo, BinaryNinjaServer, FunctionInfo, StatusResponse};
use crate::{BinaryNinjaError, ClientConfig};
use anyhow::Result;
use reqwest::Client;
use tracing::{debug, info};

/// HTTP client for Binary Ninja MCP servers
pub struct BinaryNinjaClient {
    config: ClientConfig,
    client: Client,
}

impl BinaryNinjaClient {
    /// Create a new client with default configuration
    pub fn new() -> Self {
        Self::with_config(ClientConfig::default())
    }

    /// Create a new client with custom configuration
    pub fn with_config(config: ClientConfig) -> Self {
        let client = Client::builder()
            .connect_timeout(config.connect_timeout)
            .timeout(config.read_timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    /// Discover available Binary Ninja MCP servers
    ///
    /// Scans ports starting from base_port to find running servers.
    /// Returns a list of discovered servers with their metadata.
    pub async fn discover_servers(&self) -> Result<Vec<BinaryNinjaServer>> {
        info!("Discovering Binary Ninja MCP servers...");
        let mut servers = Vec::new();

        for offset in 0..self.config.max_servers {
            let port = self.config.base_port + offset as u16;
            let url = format!("{}:{}", self.config.base_url, port);

            debug!("Checking port {}", port);

            match self.check_server(&url).await {
                Ok(Some(server)) => {
                    info!("Found server: {} on port {}", server.filename, port);
                    servers.push(server);
                }
                Ok(None) => {
                    debug!("No server on port {}", port);
                }
                Err(e) => {
                    debug!("Error checking port {}: {}", port, e);
                }
            }
        }

        info!("Discovery complete. Found {} servers", servers.len());
        Ok(servers)
    }

    /// Check if a server is running at the given URL
    async fn check_server(&self, url: &str) -> Result<Option<BinaryNinjaServer>> {
        let status_url = format!("{}/status", url);

        match self.client.get(&status_url).send().await {
            Ok(response) if response.status().is_success() => {
                let status: StatusResponse = response.json().await?;

                if status.loaded {
                    let port = url
                        .split(':')
                        .last()
                        .and_then(|p| p.parse::<u16>().ok())
                        .unwrap_or(9009);

                    let binary_id = format!("port_{}", port);
                    let filename = status.filename.unwrap_or_else(|| "unknown".to_string());

                    Ok(Some(BinaryNinjaServer::new(
                        binary_id,
                        url.to_string(),
                        port,
                        filename,
                    )))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    /// Get information about a specific binary server
    pub async fn get_binary_info(&self, binary_id: &str) -> Result<BinaryInfo> {
        let url = self.get_server_url(binary_id)?;
        let status_url = format!("{}/status", url);

        let response = self.client.get(&status_url).send().await?;

        if !response.status().is_success() {
            return Err(BinaryNinjaError::ServerNotFound(binary_id.to_string()).into());
        }

        let status: StatusResponse = response.json().await?;

        Ok(BinaryInfo {
            binary_id: binary_id.to_string(),
            filename: status.filename.unwrap_or_else(|| "unknown".to_string()),
            loaded: status.loaded,
            metadata: None, // TODO: Fetch additional metadata
        })
    }

    /// List all functions in a binary
    ///
    /// Returns a list of function names. Use `get_function_info` to get
    /// detailed information about specific functions.
    pub async fn list_functions(&self, binary_id: &str) -> Result<Vec<FunctionInfo>> {
        let url = self.get_server_url(binary_id)?;
        let methods_url = format!("{}/methods", url);

        debug!("Fetching functions from {}", methods_url);

        let response = self.client.get(&methods_url).send().await?;

        if !response.status().is_success() {
            return Err(BinaryNinjaError::RequestFailed(format!(
                "Status: {}",
                response.status()
            ))
            .into());
        }

        // Binary Ninja MCP returns JSON with a "functions" array
        let text = response.text().await?;

        // Try to parse as JSON first
        if let Ok(json_response) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(functions_array) = json_response.get("functions").and_then(|v| v.as_array()) {
                let functions: Vec<FunctionInfo> = functions_array
                    .iter()
                    .filter_map(|func| {
                        let name = func.get("name")?.as_str()?.to_string();
                        let address = func.get("address")?.as_str()?.to_string();
                        Some(FunctionInfo::new(name, address))
                    })
                    .collect();

                debug!("Found {} functions", functions.len());
                return Ok(functions);
            }
        }

        // Fallback to line-separated format for backwards compatibility
        let lines: Vec<&str> = text.lines().collect();
        let functions: Vec<FunctionInfo> = lines
            .into_iter()
            .map(|name| FunctionInfo::from_name(name.to_string()))
            .collect();

        debug!("Found {} functions", functions.len());
        Ok(functions)
    }

    /// Search for functions by name
    pub async fn search_functions(
        &self,
        binary_id: &str,
        search_term: &str,
    ) -> Result<Vec<FunctionInfo>> {
        let url = self.get_server_url(binary_id)?;
        let search_url = format!("{}/searchFunctions?query={}", url, search_term);

        debug!("Searching functions: {}", search_term);

        let response = self.client.get(&search_url).send().await?;

        if !response.status().is_success() {
            return Err(BinaryNinjaError::RequestFailed(format!(
                "Status: {}",
                response.status()
            ))
            .into());
        }

        let text = response.text().await?;

        // Try to parse as JSON first
        if let Ok(json_response) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(matches_array) = json_response.get("matches").and_then(|v| v.as_array()) {
                let functions: Vec<FunctionInfo> = matches_array
                    .iter()
                    .filter_map(|func| {
                        let name = func.get("name")?.as_str()?.to_string();
                        let address = func.get("address")?.as_str()?.to_string();
                        Some(FunctionInfo::new(name, address))
                    })
                    .collect();

                debug!("Found {} matching functions", functions.len());
                return Ok(functions);
            }
        }

        // Fallback to line-based parsing
        let lines: Vec<&str> = text.lines().collect();
        let functions = lines
            .into_iter()
            .filter(|name| !name.is_empty())
            .map(|name| FunctionInfo::from_name(name.to_string()))
            .collect();

        Ok(functions)
    }

    /// Decompile a function and return the decompiled code
    pub async fn decompile_function(
        &self,
        binary_id: &str,
        function_name: &str,
    ) -> Result<String> {
        let url = self.get_server_url(binary_id)?;
        let decompile_url = format!("{}/decompile", url);

        debug!("Decompiling function: {}", function_name);

        let response = self
            .client
            .post(&decompile_url)
            .form(&[("name", function_name)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BinaryNinjaError::FunctionNotFound(function_name.to_string()).into());
        }

        let code = response.text().await?;

        if code.starts_with("Error") || code.starts_with("Function not found") {
            return Err(BinaryNinjaError::FunctionNotFound(function_name.to_string()).into());
        }

        Ok(code)
    }

    /// Get detailed information about a function including decompiled code
    pub async fn get_function_info(
        &self,
        binary_id: &str,
        function_name: &str,
    ) -> Result<FunctionInfo> {
        // First, get the decompiled code
        let decompiled_code = self.decompile_function(binary_id, function_name).await?;

        Ok(FunctionInfo {
            name: function_name.to_string(),
            address: String::new(), // TODO: Extract from decompiled output or separate call
            raw_name: None,
            symbol: None,
            decompiled_code: Some(decompiled_code),
        })
    }

    /// Get the server URL for a given binary ID
    fn get_server_url(&self, binary_id: &str) -> Result<String> {
        // Extract port from binary_id (format: "port_9009")
        let port = binary_id
            .strip_prefix("port_")
            .and_then(|p| p.parse::<u16>().ok())
            .ok_or_else(|| BinaryNinjaError::ServerNotFound(binary_id.to_string()))?;

        Ok(format!("{}:{}", self.config.base_url, port))
    }
}

impl Default for BinaryNinjaClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_server_url() {
        let client = BinaryNinjaClient::new();
        let url = client.get_server_url("port_9009").unwrap();
        assert_eq!(url, "http://localhost:9009");
    }

    #[test]
    fn test_get_server_url_invalid() {
        let client = BinaryNinjaClient::new();
        let result = client.get_server_url("invalid");
        assert!(result.is_err());
    }
}

