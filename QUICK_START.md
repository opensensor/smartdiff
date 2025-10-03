# Quick Start Guide

This guide will help you get Smart Code Diff up and running in minutes!

## Prerequisites

Before you start, make sure you have:

- **Rust** (1.75 or later) - [Install from rustup.rs](https://rustup.rs/)
- **Node.js** (18 or later) - [Install from nodejs.org](https://nodejs.org/)
- **npm** (comes with Node.js)

### Optional but Recommended

- **tmux** (Linux/macOS only) - For better terminal management
  - Linux: `sudo apt install tmux` or `sudo yum install tmux`
  - macOS: `brew install tmux`

## Quick Start (Recommended)

### Linux / macOS

1. **Make scripts executable:**
   ```bash
   chmod +x start.sh dev.sh stop.sh
   ```

2. **Start all services:**
   ```bash
   ./start.sh
   ```

3. **Open your browser:**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080

4. **Stop services:**
   - Press `Ctrl+C` in the terminal, or
   - Run `./stop.sh` in another terminal

### Windows

1. **Start all services:**
   ```cmd
   start.bat
   ```

2. **Open your browser:**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080

3. **Stop services:**
   - Press any key in the terminal, or
   - Run `stop.bat` in another terminal

## Development Mode

For active development with hot-reload:

### Linux / macOS

```bash
./dev.sh
```

This will:
- Start backend with hot-reload (using `cargo run`)
- Start frontend with hot-reload (using `npm run dev`)
- Use tmux for split-screen view (if available)

### Windows

Use the regular `start.bat` - it already runs in development mode.

## What Each Script Does

### `start.sh` / `start.bat` (Full Start)

**Features:**
- ✓ Checks all prerequisites (Rust, Node.js, npm)
- ✓ Checks if ports are available
- ✓ Installs frontend dependencies if needed
- ✓ Builds backend if needed
- ✓ Starts both services
- ✓ Waits for services to be ready
- ✓ Shows status and URLs
- ✓ Handles graceful shutdown

**When to use:**
- First time setup
- Production-like environment
- When you want everything automated

### `dev.sh` (Quick Development)

**Features:**
- ✓ Fast startup (no checks)
- ✓ Uses tmux for split-screen (if available)
- ✓ Hot-reload enabled
- ✓ Shows logs in real-time

**When to use:**
- Active development
- Quick iterations
- When you want to see both services at once

### `stop.sh` / `stop.bat` (Stop Services)

**Features:**
- ✓ Stops backend (port 8080)
- ✓ Stops frontend (port 3000)
- ✓ Kills tmux session (if exists)
- ✓ Clean shutdown

**When to use:**
- When services are running in background
- To free up ports
- Before switching branches

## Ports Used

| Service  | Port | URL                      |
|----------|------|--------------------------|
| Backend  | 8080 | http://localhost:8080    |
| Frontend | 3000 | http://localhost:3000    |

## Logs

Logs are saved to:
- **Backend:** `backend.log` (or `logs/backend.log` on Windows)
- **Frontend:** `frontend.log` (or `logs/frontend.log` on Windows)

View logs in real-time:
```bash
# Linux/macOS
tail -f backend.log
tail -f frontend.log

# Windows
type logs\backend.log
type logs\frontend.log
```

## Troubleshooting

### Port Already in Use

**Problem:** Error message about port 8080 or 3000 already in use.

**Solution:**
```bash
# Linux/macOS
./stop.sh

# Windows
stop.bat
```

Or manually kill the process:
```bash
# Linux/macOS
lsof -ti:8080 | xargs kill -9
lsof -ti:3000 | xargs kill -9

# Windows
netstat -ano | findstr :8080
taskkill /PID <PID> /F
```

### Backend Won't Start

**Problem:** Backend fails to start or crashes.

**Solution:**
1. Check the logs: `cat backend.log`
2. Rebuild: `cargo clean && cargo build --release --bin smart-diff-server`
3. Check Rust version: `rustc --version` (should be 1.75+)

### Frontend Won't Start

**Problem:** Frontend fails to start or shows errors.

**Solution:**
1. Check the logs: `cat frontend.log`
2. Reinstall dependencies:
   ```bash
   cd nextjs-frontend
   rm -rf node_modules package-lock.json
   npm install
   cd ..
   ```
3. Check Node version: `node --version` (should be 18+)

### Dependencies Not Installing

**Problem:** npm install fails.

**Solution:**
1. Clear npm cache: `npm cache clean --force`
2. Delete `node_modules` and `package-lock.json`
3. Try again: `npm install`
4. If still failing, try: `npm install --legacy-peer-deps`

### Services Start But Can't Connect

**Problem:** Services appear to start but browser can't connect.

**Solution:**
1. Wait a bit longer (services may still be initializing)
2. Check if services are actually running:
   ```bash
   # Linux/macOS
   curl http://localhost:8080/api/health
   curl http://localhost:3000
   
   # Windows
   curl http://localhost:8080/api/health
   curl http://localhost:3000
   ```
3. Check firewall settings
4. Try accessing via `127.0.0.1` instead of `localhost`

### tmux Session Won't Detach

**Problem:** Can't exit tmux session.

**Solution:**
- Detach: Press `Ctrl+B`, then press `D`
- Kill session: `tmux kill-session -t smartdiff`
- Force kill: `./stop.sh`

## Manual Start (Advanced)

If you prefer to start services manually:

### Backend

```bash
# Development
RUST_LOG=info cargo run --bin smart-diff-server

# Production
cargo build --release --bin smart-diff-server
./target/release/smart-diff-server
```

### Frontend

```bash
cd nextjs-frontend

# Development
npm run dev

# Production
npm run build
npm start
```

## Environment Variables

### Backend

- `RUST_LOG` - Log level (default: `info`)
  - Options: `error`, `warn`, `info`, `debug`, `trace`
- `RUST_BACKTRACE` - Show backtraces on errors (default: `0`)
  - Set to `1` for full backtraces

Example:
```bash
RUST_LOG=debug RUST_BACKTRACE=1 cargo run --bin smart-diff-server
```

### Frontend

Create `nextjs-frontend/.env.local`:
```env
NEXT_PUBLIC_API_URL=http://localhost:8080
```


## Next Steps

Once services are running:

1. **Open the frontend:** http://localhost:3000
2. **Try the examples:**
   - Compare two files
   - Analyze a directory
   - View the graph visualization
3. **Read the docs:**
   - [User Guide](docs/user-guide.md)
   - [Developer Guide](docs/developer-guide.md)
   - [API Documentation](docs/api/)

## Getting Help

- **Documentation:** Check the `docs/` directory
- **Issues:** Report bugs on GitHub
- **Logs:** Always check `backend.log` and `frontend.log` first

## Performance Tips

1. **First build is slow:** Rust compilation takes time. Subsequent builds are faster.
2. **Use release mode:** For better performance, always use `--release` flag
3. **Close other apps:** Free up RAM and CPU for better performance
4. **Use SSD:** Faster disk = faster builds and startup

## Updating

To update to the latest version:

```bash
# Pull latest changes
git pull

# Rebuild backend
cargo build --release --bin smart-diff-server

# Update frontend dependencies
cd nextjs-frontend
npm install
cd ..

# Restart services
./stop.sh
./start.sh
```

## Uninstalling

To completely remove:

```bash
# Stop services
./stop.sh

# Remove build artifacts
cargo clean
rm -rf nextjs-frontend/node_modules
rm -rf nextjs-frontend/.next

# Remove logs
rm -f backend.log frontend.log
rm -rf logs/
```

---

**Happy Coding! 🚀**

