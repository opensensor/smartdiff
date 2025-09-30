# SSE Transport Setup for Augment Code

The stdio transport has a ~14 second timeout in Augment, which causes issues with large code comparisons. The Python SSE bridge solves this by wrapping the stdio MCP server with an HTTP/SSE interface that doesn't have timeout limitations.

## Setup

### 1. Install Dependencies

The SSE bridge requires Python 3 and aiohttp:

```bash
cd crates/mcp-server
pipenv install aiohttp
```

### 2. Start the SSE Bridge

```bash
cd crates/mcp-server
PIPENV_IGNORE_VIRTUALENVS=1 pipenv run python sse_bridge.py --binary ../../target/release/smart-diff-mcp --port 8011
```

Or use the provided start script:

```bash
./crates/mcp-server/start_sse_bridge.sh
```

The bridge will start on `http://127.0.0.1:8011` with the following endpoints:
- SSE endpoint: `http://127.0.0.1:8011/sse`
- Message endpoint: `http://127.0.0.1:8011/message`
- Health check: `http://127.0.0.1:8011/health`

### 3. Configure Augment

Update your Augment MCP configuration at:
`~/.config/Code/User/globalStorage/augment.vscode-augment/augment-global-state/mcpServers.json`

Change the smart-diff entry from:

```json
{
  "type": "stdio",
  "name": "smart-diff",
  "command": "/home/matteius/codediff/target/release/smart-diff-mcp",
  "arguments": "",
  "useShellInterpolation": true,
  "id": "8dde13fe-cff7-424e-b3ae-677fb08bedd3",
  "tools": [],
  "disabled": false
}
```

To:

```json
{
  "type": "sse",
  "name": "smart-diff",
  "url": "http://127.0.0.1:8011/sse",
  "id": "8dde13fe-cff7-424e-b3ae-677fb08bedd3",
  "tools": [],
  "disabled": false
}
```

### 4. Restart Augment

Restart VS Code or toggle the smart-diff server off and on in Augment's MCP settings.

## Benefits

- ✅ No timeout limitations - comparisons can take as long as needed
- ✅ Better for large codebases (thousands of functions)
- ✅ Same functionality as stdio transport
- ✅ Easy to debug (HTTP logs)

## Troubleshooting

### Check if the bridge is running

```bash
curl http://127.0.0.1:8011/health
```

Should return: `{"status": "healthy"}`

### View bridge logs

The bridge outputs logs to stdout showing all requests and responses.

### Test the SSE endpoint

```bash
curl http://127.0.0.1:8011/sse
```

Should stream SSE events.

### Port already in use

If port 8011 is already in use, specify a different port:

```bash
pipenv run python sse_bridge.py --port 8012
```

And update the URL in Augment's config accordingly.

