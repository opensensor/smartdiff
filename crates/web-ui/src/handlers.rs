//! HTTP request handlers

use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, Json as ResponseJson},
};
use serde_json::{json, Value};

use crate::models::{CompareRequest, CompareResponse};

/// Root handler - serves basic info about the API
pub async fn root() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Smart Code Diff API</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .endpoint { background: #f5f5f5; padding: 10px; margin: 10px 0; border-radius: 5px; }
        </style>
    </head>
    <body>
        <h1>Smart Code Diff API</h1>
        <p>A next-generation code diffing tool that performs structural and semantic comparison.</p>
        
        <h2>Available Endpoints</h2>
        <div class="endpoint">
            <strong>GET /api/health</strong> - Health check
        </div>
        <div class="endpoint">
            <strong>POST /api/compare</strong> - Compare code files
        </div>
        
        <h2>Example Usage</h2>
        <pre>
curl -X POST http://localhost:3000/api/compare \
  -H "Content-Type: application/json" \
  -d '{
    "file1": {"path": "old.py", "content": "def hello(): pass"},
    "file2": {"path": "new.py", "content": "def hello_world(): pass"}
  }'
        </pre>
    </body>
    </html>
    "#)
}

/// Health check endpoint
pub async fn health() -> ResponseJson<Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "smart-code-diff",
        "version": "0.1.0"
    }))
}

/// Compare endpoint - main functionality
pub async fn compare(
    Json(request): Json<CompareRequest>,
) -> Result<ResponseJson<CompareResponse>, StatusCode> {
    tracing::info!("Received compare request for {} and {}", 
                   request.file1.path, request.file2.path);
    
    // TODO: Implement actual comparison logic using the diff engine
    let response = CompareResponse {
        similarity: 0.85,
        changes: vec![
            "Function renamed from 'hello' to 'hello_world'".to_string(),
        ],
        functions_added: vec![],
        functions_removed: vec![],
        functions_modified: vec!["hello -> hello_world".to_string()],
        execution_time_ms: 42,
    };
    
    Ok(ResponseJson(response))
}
