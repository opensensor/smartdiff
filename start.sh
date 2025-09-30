#!/bin/bash
# Smart Code Diff - Easy Start Script (Linux/macOS)
# This script starts both the Rust backend and Next.js frontend

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if a port is in use
port_in_use() {
    lsof -i :"$1" >/dev/null 2>&1 || netstat -an | grep -q ":$1.*LISTEN" 2>/dev/null
}

# Function to kill process on port
kill_port() {
    local port=$1
    print_info "Killing process on port $port..."
    lsof -ti:$port | xargs kill -9 2>/dev/null || true
}

# Cleanup function
cleanup() {
    print_info "Shutting down services..."
    if [ ! -z "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null || true
    fi
    if [ ! -z "$FRONTEND_PID" ]; then
        kill $FRONTEND_PID 2>/dev/null || true
    fi
    print_success "Services stopped"
    exit 0
}

# Set up trap for cleanup
trap cleanup SIGINT SIGTERM

# Print banner
echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║                                                       ║"
echo "║          Smart Code Diff - Easy Start                ║"
echo "║                                                       ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""

# Check prerequisites
print_info "Checking prerequisites..."

if ! command_exists cargo; then
    print_error "Rust/Cargo is not installed. Please install from https://rustup.rs/"
    exit 1
fi
print_success "Rust/Cargo found"

if ! command_exists node; then
    print_error "Node.js is not installed. Please install from https://nodejs.org/"
    exit 1
fi
print_success "Node.js found"

if ! command_exists npm; then
    print_error "npm is not installed. Please install Node.js from https://nodejs.org/"
    exit 1
fi
print_success "npm found"

# Check if ports are available
BACKEND_PORT=8080
FRONTEND_PORT=3000

if port_in_use $BACKEND_PORT; then
    print_warning "Port $BACKEND_PORT is already in use"
    read -p "Kill the process and continue? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        kill_port $BACKEND_PORT
        sleep 2
    else
        print_error "Cannot start backend on port $BACKEND_PORT"
        exit 1
    fi
fi

if port_in_use $FRONTEND_PORT; then
    print_warning "Port $FRONTEND_PORT is already in use"
    read -p "Kill the process and continue? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        kill_port $FRONTEND_PORT
        sleep 2
    else
        print_error "Cannot start frontend on port $FRONTEND_PORT"
        exit 1
    fi
fi

# Install frontend dependencies if needed
if [ ! -d "nextjs-frontend/node_modules" ]; then
    print_info "Installing frontend dependencies (this may take a few minutes)..."
    cd nextjs-frontend
    npm install
    cd ..
    print_success "Frontend dependencies installed"
else
    print_success "Frontend dependencies already installed"
fi

# Build backend if needed
if [ ! -f "target/release/smart-diff-server" ]; then
    print_info "Building Rust backend (this may take a few minutes)..."
    cargo build --release --bin smart-diff-server
    print_success "Backend built successfully"
else
    print_success "Backend already built"
fi

# Start backend
print_info "Starting Rust backend on port $BACKEND_PORT..."
RUST_LOG=info cargo run --release --bin smart-diff-server > backend.log 2>&1 &
BACKEND_PID=$!

# Wait for backend to start
print_info "Waiting for backend to be ready..."
for i in {1..30}; do
    if curl -s http://localhost:$BACKEND_PORT/api/health > /dev/null 2>&1; then
        print_success "Backend is ready!"
        break
    fi
    if [ $i -eq 30 ]; then
        print_error "Backend failed to start. Check backend.log for details."
        cat backend.log
        cleanup
        exit 1
    fi
    sleep 1
done

# Start frontend
print_info "Starting Next.js frontend on port $FRONTEND_PORT..."
cd nextjs-frontend
npm run dev > ../frontend.log 2>&1 &
FRONTEND_PID=$!
cd ..

# Wait for frontend to start
print_info "Waiting for frontend to be ready..."
for i in {1..30}; do
    if curl -s http://localhost:$FRONTEND_PORT > /dev/null 2>&1; then
        print_success "Frontend is ready!"
        break
    fi
    if [ $i -eq 30 ]; then
        print_error "Frontend failed to start. Check frontend.log for details."
        cat frontend.log
        cleanup
        exit 1
    fi
    sleep 1
done

# Success!
echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║                                                       ║"
echo "║              🚀 Services Started!                     ║"
echo "║                                                       ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""
print_success "Backend running at:  http://localhost:$BACKEND_PORT"
print_success "Frontend running at: http://localhost:$FRONTEND_PORT"
echo ""
print_info "Logs:"
echo "  - Backend:  tail -f backend.log"
echo "  - Frontend: tail -f frontend.log"
echo ""
print_warning "Press Ctrl+C to stop all services"
echo ""

# Keep script running and wait for Ctrl+C
wait

