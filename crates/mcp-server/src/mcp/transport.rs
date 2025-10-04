//! Stdio transport for MCP communication

use super::messages::JsonRpcMessage;
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// Stdio-based transport for MCP
#[allow(dead_code)]
pub struct StdioTransport {
    tx: mpsc::UnboundedSender<JsonRpcMessage>,
    rx: mpsc::UnboundedReceiver<JsonRpcMessage>,
}

impl StdioTransport {
    pub fn new() -> (Self, mpsc::UnboundedSender<JsonRpcMessage>) {
        let (outbound_tx, outbound_rx) = mpsc::unbounded_channel();
        let (inbound_tx, inbound_rx) = mpsc::unbounded_channel();

        let transport = Self {
            tx: outbound_tx.clone(),
            rx: inbound_rx,
        };

        // Spawn reader task
        tokio::spawn(Self::read_loop(inbound_tx));

        // Spawn writer task
        tokio::spawn(Self::write_loop(outbound_rx));

        (transport, outbound_tx)
    }

    /// Read messages from stdin
    async fn read_loop(tx: mpsc::UnboundedSender<JsonRpcMessage>) {
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    info!("Stdin closed, shutting down");
                    break;
                }
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    debug!("Received: {}", trimmed);

                    match serde_json::from_str::<JsonRpcMessage>(trimmed) {
                        Ok(message) => {
                            if tx.send(message).is_err() {
                                error!("Failed to send message to handler");
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse JSON-RPC message: {}", e);
                            // Continue processing other messages
                        }
                    }
                }
                Err(e) => {
                    error!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }
    }

    /// Write messages to stdout
    async fn write_loop(mut rx: mpsc::UnboundedReceiver<JsonRpcMessage>) {
        let mut stdout = tokio::io::stdout();

        while let Some(message) = rx.recv().await {
            match serde_json::to_string(&message) {
                Ok(json) => {
                    debug!("Sending: {}", json);
                    if let Err(e) = stdout.write_all(json.as_bytes()).await {
                        error!("Failed to write to stdout: {}", e);
                        break;
                    }
                    if let Err(e) = stdout.write_all(b"\n").await {
                        error!("Failed to write newline to stdout: {}", e);
                        break;
                    }
                    if let Err(e) = stdout.flush().await {
                        error!("Failed to flush stdout: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                }
            }
        }
    }

    /// Receive a message
    pub async fn recv(&mut self) -> Option<JsonRpcMessage> {
        self.rx.recv().await
    }

    /// Send a message
    #[allow(dead_code)]
    pub fn send(&self, message: JsonRpcMessage) -> Result<()> {
        self.tx.send(message).context("Failed to send message")?;
        Ok(())
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new().0
    }
}
