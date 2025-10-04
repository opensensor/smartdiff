//! Type definitions for Binary Ninja MCP client

use serde::{Deserialize, Serialize};

/// Information about a Binary Ninja MCP server instance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryNinjaServer {
    /// Unique identifier for this server (e.g., "port_9009")
    pub binary_id: String,

    /// Full URL to the server (e.g., "http://localhost:9009")
    pub url: String,

    /// Port number the server is running on
    pub port: u16,

    /// Filename of the loaded binary
    pub filename: String,

    /// Additional metadata about the binary
    #[serde(flatten)]
    pub metadata: Option<BinaryMetadata>,
}

/// Metadata about a loaded binary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryMetadata {
    /// Architecture (e.g., "x86_64", "arm64")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Platform (e.g., "linux", "windows", "macos")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    /// Entry point address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,

    /// Number of functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_count: Option<usize>,
}

/// Information about a binary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BinaryInfo {
    /// Binary server ID
    pub binary_id: String,

    /// Filename
    pub filename: String,

    /// Whether the binary is loaded
    pub loaded: bool,

    /// Metadata
    #[serde(flatten)]
    pub metadata: Option<BinaryMetadata>,
}

/// Information about a function in a binary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,

    /// Function address (hex string, e.g., "0x401000")
    pub address: String,

    /// Raw/mangled name (if different from name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_name: Option<String>,

    /// Symbol information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<SymbolInfo>,

    /// Decompiled code (if fetched)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decompiled_code: Option<String>,
}

/// Symbol information for a function
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SymbolInfo {
    /// Symbol type
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_type: Option<String>,

    /// Full symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

/// Response from Binary Ninja MCP status endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// Whether a binary is loaded
    pub loaded: bool,

    /// Filename of the loaded binary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// Response from Binary Ninja MCP methods endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodsResponse {
    /// List of function names
    pub methods: Vec<String>,
}

/// Response from Binary Ninja MCP search endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// List of matching functions
    pub functions: Vec<FunctionInfo>,
}

/// Response from Binary Ninja MCP decompile endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompileResponse {
    /// Decompiled code
    pub code: String,
}

impl FunctionInfo {
    /// Create a new FunctionInfo with just name and address
    pub fn new(name: String, address: String) -> Self {
        Self {
            name,
            address,
            raw_name: None,
            symbol: None,
            decompiled_code: None,
        }
    }

    /// Create from a simple name string (address will be empty)
    pub fn from_name(name: String) -> Self {
        Self {
            name,
            address: String::new(),
            raw_name: None,
            symbol: None,
            decompiled_code: None,
        }
    }
}

impl BinaryNinjaServer {
    /// Create a new server info
    pub fn new(binary_id: String, url: String, port: u16, filename: String) -> Self {
        Self {
            binary_id,
            url,
            port,
            filename,
            metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_info_new() {
        let func = FunctionInfo::new("main".to_string(), "0x401000".to_string());
        assert_eq!(func.name, "main");
        assert_eq!(func.address, "0x401000");
        assert!(func.decompiled_code.is_none());
    }

    #[test]
    fn test_function_info_from_name() {
        let func = FunctionInfo::from_name("process_data".to_string());
        assert_eq!(func.name, "process_data");
        assert_eq!(func.address, "");
    }

    #[test]
    fn test_server_new() {
        let server = BinaryNinjaServer::new(
            "port_9009".to_string(),
            "http://localhost:9009".to_string(),
            9009,
            "test.exe".to_string(),
        );
        assert_eq!(server.binary_id, "port_9009");
        assert_eq!(server.port, 9009);
        assert_eq!(server.filename, "test.exe");
    }
}

