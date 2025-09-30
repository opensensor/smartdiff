# Start Scripts Summary

This document provides an overview of all the start scripts created for Smart Code Diff.

## Quick Reference

| Script | Platform | Purpose | When to Use |
|--------|----------|---------|-------------|
| `install.sh` | Linux/macOS | One-time setup | First time installation |
| `start.sh` | Linux/macOS | Full start with checks | Production-like start |
| `start.bat` | Windows | Full start with checks | Production-like start |
| `dev.sh` | Linux/macOS | Quick dev start | Active development |
| `stop.sh` | Linux/macOS | Stop all services | Cleanup |
| `stop.bat` | Windows | Stop all services | Cleanup |
| `make start` | Linux/macOS | Makefile wrapper | Alternative to start.sh |
| `make stop` | Linux/macOS | Makefile wrapper | Alternative to stop.sh |

## Installation Scripts

### `install.sh` (Linux/macOS)

**Purpose:** One-command installation and setup

**What it does:**
1. Checks for Rust (installs if missing)
2. Checks for Node.js (prompts to install if missing)
3. Makes all scripts executable
4. Installs frontend dependencies
5. Builds backend in release mode

**Usage:**
```bash
./install.sh
```

**First-time users:** Start here!

## Start Scripts

### `start.sh` (Linux/macOS)

**Purpose:** Full-featured start script with comprehensive checks

**Features:**
- ✓ Prerequisite checking (Rust, Node.js, npm)
- ✓ Port availability checking
- ✓ Automatic dependency installation
- ✓ Automatic backend building
- ✓ Health checks for both services
- ✓ Graceful shutdown on Ctrl+C
- ✓ Detailed logging
- ✓ Error handling and recovery

**Usage:**
```bash
./start.sh
```

**Ports:**
- Backend: 8080
- Frontend: 3000

**Logs:**
- `backend.log`
- `frontend.log`

**Stop:**
- Press `Ctrl+C` in the terminal
- Or run `./stop.sh` in another terminal

---

### `start.bat` (Windows)

**Purpose:** Windows equivalent of start.sh

**Features:**
- ✓ Prerequisite checking
- ✓ Port availability checking
- ✓ Automatic dependency installation
- ✓ Automatic backend building
- ✓ Health checks for both services
- ✓ Opens browser automatically
- ✓ Detailed logging in `logs/` directory

**Usage:**
```cmd
start.bat
```

**Ports:**
- Backend: 8080
- Frontend: 3000

**Logs:**
- `logs\backend.log`
- `logs\frontend.log`

**Stop:**
- Press any key in the terminal
- Or run `stop.bat` in another terminal

---

### `dev.sh` (Linux/macOS)

**Purpose:** Quick development start with minimal checks

**Features:**
- ✓ Fast startup (no prerequisite checks)
- ✓ Uses tmux for split-screen (if available)
- ✓ Hot-reload enabled for both services
- ✓ Real-time log viewing
- ✓ Easy detach/reattach

**Usage:**
```bash
./dev.sh
```

**With tmux:**
- Backend in left pane
- Frontend in right pane
- Detach: `Ctrl+B` then `D`
- Reattach: `tmux attach -t smartdiff`
- Kill: `tmux kill-session -t smartdiff`

**Without tmux:**
- Runs in background
- Shows PIDs for manual control
- Check logs: `tail -f backend.log frontend.log`

**When to use:**
- Active development
- Quick iterations
- Testing changes
- When you want to see both services at once

## Stop Scripts

### `stop.sh` (Linux/macOS)

**Purpose:** Stop all Smart Code Diff services

**What it stops:**
- ✓ Backend on port 8080
- ✓ Frontend on port 3000
- ✓ tmux session (if exists)

**Usage:**
```bash
./stop.sh
```

**Safe to run:** Won't error if services aren't running

---

### `stop.bat` (Windows)

**Purpose:** Windows equivalent of stop.sh

**What it stops:**
- ✓ Backend processes
- ✓ Frontend processes
- ✓ Any processes on ports 8080 and 3000

**Usage:**
```cmd
stop.bat
```

## Makefile Targets

### `make start`

Wrapper for `./start.sh`

```bash
make start
```

### `make dev-start`

Wrapper for `./dev.sh`

```bash
make dev-start
```

### `make stop`

Wrapper for `./stop.sh`

```bash
make stop
```

### `make setup`

First-time setup (installs dependencies and builds)

```bash
make setup
```

### `make start-backend`

Start only the backend

```bash
make start-backend
```

### `make start-frontend`

Start only the frontend

```bash
make start-frontend
```

### `make install-frontend`

Install frontend dependencies only

```bash
make install-frontend
```

### `make build-frontend`

Build frontend for production

```bash
make build-frontend
```

## Comparison Matrix

### Feature Comparison

| Feature | start.sh | start.bat | dev.sh |
|---------|----------|-----------|--------|
| Prerequisite checks | ✓ | ✓ | ✗ |
| Port checks | ✓ | ✓ | ✗ |
| Auto-install deps | ✓ | ✓ | ✗ |
| Auto-build backend | ✓ | ✓ | ✗ |
| Health checks | ✓ | ✓ | ✗ |
| Graceful shutdown | ✓ | ✓ | ✓ |
| Logging | ✓ | ✓ | ✓ |
| tmux support | ✗ | N/A | ✓ |
| Auto-open browser | ✗ | ✓ | ✗ |
| Startup time | Slow | Slow | Fast |

### When to Use Each

**Use `install.sh`:**
- First time setup
- After pulling major updates
- When dependencies change

**Use `start.sh` / `start.bat`:**
- First time running
- Production-like environment
- When you want everything automated
- When ports might be in use
- When you're not sure if everything is set up

**Use `dev.sh`:**
- Active development
- Quick iterations
- When you know everything is set up
- When you want split-screen view
- When you want hot-reload

**Use `stop.sh` / `stop.bat`:**
- When services are running in background
- To free up ports
- Before switching branches
- Before pulling updates

**Use `make` targets:**
- When you prefer Makefile workflow
- For CI/CD integration
- For scripting and automation

## Workflow Examples

### First Time Setup

```bash
# 1. Clone repository
git clone https://github.com/smart-code-diff/smart-code-diff.git
cd smart-code-diff

# 2. Run installer
./install.sh

# 3. Start services
./start.sh

# 4. Open browser to http://localhost:3000
```

### Daily Development

```bash
# Morning: Start development
./dev.sh

# Work on code...
# (hot-reload handles changes automatically)

# Evening: Stop services
./stop.sh
```

### Testing Changes

```bash
# Start with full checks
./start.sh

# Test your changes...

# Stop
./stop.sh
```

### Switching Branches

```bash
# Stop services
./stop.sh

# Switch branch
git checkout feature-branch

# Reinstall dependencies if needed
cd nextjs-frontend && npm install && cd ..

# Rebuild backend if needed
cargo build --release --bin smart-diff-server

# Start again
./start.sh
```

## Troubleshooting

### Scripts Won't Execute (Linux/macOS)

**Problem:** Permission denied

**Solution:**
```bash
chmod +x install.sh start.sh dev.sh stop.sh
```

### Port Already in Use

**Problem:** Port 8080 or 3000 is busy

**Solution:**
```bash
./stop.sh  # Kill any existing services
./start.sh # Try again
```

### Services Won't Start

**Problem:** Backend or frontend fails to start

**Solution:**
1. Check logs: `cat backend.log` or `cat frontend.log`
2. Rebuild: `cargo clean && cargo build --release --bin smart-diff-server`
3. Reinstall: `cd nextjs-frontend && rm -rf node_modules && npm install`

### tmux Not Found

**Problem:** dev.sh says tmux not found

**Solution:**
```bash
# Linux
sudo apt install tmux

# macOS
brew install tmux

# Or just use start.sh instead
```

## Advanced Usage

### Custom Ports

Edit the scripts to change ports:

**Backend port (default 8080):**
- Edit `crates/web-ui/src/main.rs`
- Change `0.0.0.0:8080` to your desired port

**Frontend port (default 3000):**
- Edit `nextjs-frontend/package.json`
- Change `"dev": "next dev"` to `"dev": "next dev -p YOUR_PORT"`

### Environment Variables

**Backend:**
```bash
RUST_LOG=debug ./start.sh  # More verbose logging
RUST_BACKTRACE=1 ./start.sh  # Show backtraces
```

**Frontend:**
Create `nextjs-frontend/.env.local`:
```env
NEXT_PUBLIC_API_URL=http://localhost:8080
```

### Running in Background

**Linux/macOS:**
```bash
nohup ./start.sh > /dev/null 2>&1 &
```

**Windows:**
Use Task Scheduler or run as a service

## Best Practices

1. **Always use `./stop.sh` before pulling updates**
2. **Run `./install.sh` after major updates**
3. **Use `./dev.sh` for development, `./start.sh` for testing**
4. **Check logs if something goes wrong**
5. **Keep scripts executable with `chmod +x`**
6. **Don't commit log files to git**

## Files Created

The scripts create these files:
- `backend.log` - Backend logs
- `frontend.log` - Frontend logs
- `logs/` - Log directory (Windows)
- `nextjs-frontend/node_modules/` - Dependencies
- `target/` - Rust build artifacts

Add to `.gitignore`:
```
backend.log
frontend.log
logs/
```

## Summary

- **Installation:** `./install.sh` (one time)
- **Start:** `./start.sh` or `start.bat`
- **Development:** `./dev.sh`
- **Stop:** `./stop.sh` or `stop.bat`
- **Help:** See `QUICK_START.md`

**Happy coding! 🚀**

