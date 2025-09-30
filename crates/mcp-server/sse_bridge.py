#!/usr/bin/env python3
"""
SSE Bridge for Smart Diff MCP Server

This script wraps the stdio-based MCP server and exposes it via SSE (Server-Sent Events)
over HTTP. This avoids the timeout issues with stdio transport in Augment.

Usage:
    python3 sse_bridge.py [--port PORT] [--binary PATH]
"""

import asyncio
import json
import subprocess
import sys
from typing import Optional
from aiohttp import web
import argparse
import logging

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class McpSseBridge:
    def __init__(self, binary_path: str):
        self.binary_path = binary_path
        self.process: Optional[subprocess.Popen] = None
        self.clients = set()
        
    async def start_mcp_server(self):
        """Start the MCP server subprocess"""
        logger.info(f"Starting MCP server: {self.binary_path}")

        # Enable debug logging for MCP server
        import os
        env = os.environ.copy()
        env['RUST_LOG'] = 'debug'

        # Open stderr log file
        stderr_log = open('/tmp/mcp-server-stderr.log', 'w')

        self.process = subprocess.Popen(
            [self.binary_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=stderr_log,
            bufsize=0,
            env=env,
        )
        logger.info("MCP server started, stderr logging to /tmp/mcp-server-stderr.log")

    async def stop_mcp_server(self):
        """Stop the MCP server subprocess"""
        if self.process:
            logger.info("Stopping MCP server")
            self.process.terminate()
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                self.process.kill()
            logger.info("MCP server stopped")
            
    async def send_to_mcp(self, message: dict) -> dict:
        """Send a message to the MCP server and get response"""
        if not self.process:
            raise RuntimeError("MCP server not started")

        # Send message
        message_json = json.dumps(message) + '\n'
        logger.debug(f"Sending to MCP: {message_json.strip()}")
        self.process.stdin.write(message_json.encode())
        self.process.stdin.flush()

        # Read response - skip any non-JSON lines (logs)
        while True:
            response_line = self.process.stdout.readline()
            if not response_line:
                raise RuntimeError("MCP server closed connection")

            line = response_line.decode().strip()
            if not line:
                continue

            # Try to parse as JSON
            try:
                response = json.loads(line)
                logger.debug(f"Received from MCP: {json.dumps(response)[:200]}")
                return response
            except json.JSONDecodeError:
                # This is a log line, skip it
                logger.debug(f"Skipping non-JSON line: {line[:100]}")
                continue
        
    async def broadcast_to_clients(self, message: dict):
        """Broadcast a message to all SSE clients"""
        message_json = json.dumps(message)
        dead_clients = set()

        logger.info(f"Broadcasting to {len(self.clients)} clients")

        for client_queue in self.clients:
            try:
                await client_queue.put(message_json)
                logger.debug(f"Sent to client queue")
            except Exception as e:
                logger.warning(f"Failed to send to client: {e}")
                dead_clients.add(client_queue)

        # Remove dead clients
        self.clients -= dead_clients


# Global bridge instance
bridge: Optional[McpSseBridge] = None


async def sse_handler(request):
    """SSE endpoint - streams responses to clients"""
    response = web.StreamResponse()
    response.headers['Content-Type'] = 'text/event-stream'
    response.headers['Cache-Control'] = 'no-cache'
    response.headers['Connection'] = 'keep-alive'
    response.headers['Access-Control-Allow-Origin'] = '*'
    
    await response.prepare(request)
    
    # Create a queue for this client
    client_queue = asyncio.Queue()
    bridge.clients.add(client_queue)
    
    logger.info("SSE client connected")
    
    try:
        # Send endpoint event (just the path, not JSON)
        await response.write(f"event: endpoint\ndata: /message\n\n".encode())
        logger.info("Sent endpoint event")

        # Stream messages to client
        while True:
            message = await client_queue.get()
            logger.debug(f"Streaming message to SSE client: {message[:100]}")
            event_data = f"event: message\ndata: {message}\n\n"
            try:
                await response.write(event_data.encode())
            except Exception as e:
                logger.warning(f"Failed to write to SSE client: {e}")
                break

    except asyncio.CancelledError:
        logger.info("SSE client disconnected")
    except Exception as e:
        logger.error(f"SSE handler error: {e}")
    finally:
        bridge.clients.discard(client_queue)
        logger.info(f"SSE client removed, {len(bridge.clients)} clients remaining")
        
    return response


async def message_handler(request):
    """Message endpoint - receives JSON-RPC requests"""
    try:
        message = await request.json()
        logger.info(f"Received message: {message.get('method', 'unknown')}")
        
        # Handle notification (no response needed)
        if 'id' not in message:
            logger.info(f"Received notification: {message.get('method')}")
            return web.Response(status=200)
        
        # Send to MCP server and get response
        response = await bridge.send_to_mcp(message)
        
        # Broadcast response to SSE clients
        await bridge.broadcast_to_clients(response)
        
        # Also return as HTTP response
        return web.json_response(response)
        
    except Exception as e:
        logger.error(f"Error handling message: {e}", exc_info=True)
        error_response = {
            "jsonrpc": "2.0",
            "id": message.get('id') if 'message' in locals() else None,
            "error": {
                "code": -32603,
                "message": str(e)
            }
        }
        return web.json_response(error_response, status=500)


async def health_handler(request):
    """Health check endpoint"""
    if bridge and bridge.process and bridge.process.poll() is None:
        return web.json_response({"status": "healthy"})
    else:
        return web.json_response({"status": "unhealthy"}, status=503)


async def start_background_tasks(app):
    """Start the MCP server when the web app starts"""
    await bridge.start_mcp_server()


async def cleanup_background_tasks(app):
    """Stop the MCP server when the web app stops"""
    await bridge.stop_mcp_server()


def main():
    parser = argparse.ArgumentParser(description='SSE Bridge for Smart Diff MCP Server')
    parser.add_argument('--port', type=int, default=8011, help='Port to listen on (default: 8011)')
    parser.add_argument('--binary', type=str, 
                       default='./target/release/smart-diff-mcp',
                       help='Path to MCP server binary')
    parser.add_argument('--host', type=str, default='127.0.0.1', help='Host to bind to')
    args = parser.parse_args()
    
    global bridge
    bridge = McpSseBridge(args.binary)
    
    app = web.Application()
    app.router.add_get('/sse', sse_handler)
    app.router.add_post('/message', message_handler)
    app.router.add_get('/health', health_handler)
    
    app.on_startup.append(start_background_tasks)
    app.on_cleanup.append(cleanup_background_tasks)
    
    logger.info(f"Starting SSE bridge on http://{args.host}:{args.port}")
    logger.info(f"SSE endpoint: http://{args.host}:{args.port}/sse")
    logger.info(f"Message endpoint: http://{args.host}:{args.port}/message")
    logger.info(f"Health endpoint: http://{args.host}:{args.port}/health")
    
    web.run_app(app, host=args.host, port=args.port)


if __name__ == '__main__':
    main()

