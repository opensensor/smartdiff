//! SSE (Server-Sent Events) transport for MCP communication

use super::messages::{JsonRpcMessage, JsonRpcRequest, JsonRpcResponse};
use anyhow::Result;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
    routing::{get, post},
    Json, Router,
};
use futures::stream::{Stream, StreamExt};
use serde_json::Value;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info};

use crate::server::McpServer;

/// SSE transport state
#[derive(Clone)]
pub struct SseTransportState {
    server: Arc<McpServer>,
    /// Channel for sending responses back to SSE clients
    response_tx: Arc<RwLock<Option<mpsc::UnboundedSender<JsonRpcMessage>>>>,
}

impl SseTransportState {
    pub fn new(server: Arc<McpServer>) -> Self {
        Self {
            server,
            response_tx: Arc::new(RwLock::new(None)),
        }
    }
}

/// Create SSE router
pub fn create_sse_router(server: Arc<McpServer>) -> Router {
    let state = SseTransportState::new(server);

    Router::new()
        .route("/sse", get(sse_handler))
        .route("/message", post(message_handler))
        .with_state(state)
}

/// SSE endpoint handler
async fn sse_handler(
    State(state): State<SseTransportState>,
) -> impl IntoResponse {
    info!("SSE client connected");

    let (tx, mut rx) = mpsc::unbounded_channel::<JsonRpcMessage>();

    // Store the sender for this connection
    *state.response_tx.write().await = Some(tx);

    let stream = async_stream::stream! {
        // Send endpoint event
        let endpoint_event = serde_json::json!({
            "endpoint": "/message"
        });
        yield Ok::<_, Infallible>(Event::default()
            .event("endpoint")
            .data(serde_json::to_string(&endpoint_event).unwrap_or_default()));

        // Stream responses
        while let Some(message) = rx.recv().await {
            debug!("Sending SSE message: {:?}", message);
            if let Ok(json) = serde_json::to_string(&message) {
                yield Ok::<_, Infallible>(Event::default()
                    .event("message")
                    .data(json));
            }
        }

        info!("SSE client disconnected");
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// Message endpoint handler (receives JSON-RPC requests)
async fn message_handler(
    State(state): State<SseTransportState>,
    Json(message): Json<JsonRpcMessage>,
) -> Response {
    debug!("Received message: {:?}", message);

    // Handle the request
    let response = match message {
        JsonRpcMessage::Request(request) => {
            let response = state.server.handle_request(request).await;
            
            // Send response via SSE
            if let Some(tx) = state.response_tx.read().await.as_ref() {
                if let Err(e) = tx.send(JsonRpcMessage::Response(response.clone())) {
                    error!("Failed to send response via SSE: {}", e);
                }
            }
            
            // Also return as HTTP response
            Json(response).into_response()
        }
        JsonRpcMessage::Notification(notification) => {
            info!("Received notification: {}", notification.method);
            // Notifications don't get responses
            StatusCode::OK.into_response()
        }
        JsonRpcMessage::Response(_) => {
            error!("Received unexpected response message");
            StatusCode::BAD_REQUEST.into_response()
        }
    };

    response
}

