#!/bin/bash
# Smart Code Diff - Quick Development Start
# Starts services in development mode with hot-reload

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting Smart Code Diff in development mode...${NC}\n"

# Check if tmux is available for better terminal management
if command -v tmux >/dev/null 2>&1; then
    echo -e "${GREEN}Using tmux for better terminal management${NC}"
    
    # Create new tmux session
    tmux new-session -d -s smartdiff
    
    # Split window horizontally
    tmux split-window -h
    
    # Run backend in left pane
    tmux send-keys -t smartdiff:0.0 'echo "Starting Rust Backend..." && RUST_LOG=info cargo run --bin smart-diff-server' C-m
    
    # Run frontend in right pane
    tmux send-keys -t smartdiff:0.1 'cd nextjs-frontend && echo "Starting Next.js Frontend..." && npm run dev' C-m
    
    # Attach to session
    echo -e "\n${YELLOW}Attaching to tmux session. Use Ctrl+B then D to detach.${NC}"
    echo -e "${YELLOW}To reattach later: tmux attach -t smartdiff${NC}"
    echo -e "${YELLOW}To kill session: tmux kill-session -t smartdiff${NC}\n"
    sleep 2
    tmux attach -t smartdiff
    
else
    # Fallback: use background processes
    echo -e "${YELLOW}tmux not found. Starting in background mode...${NC}"
    echo -e "${BLUE}Install tmux for better experience: sudo apt install tmux (Linux) or brew install tmux (macOS)${NC}\n"
    
    # Start backend
    echo -e "${GREEN}Starting backend...${NC}"
    RUST_LOG=info cargo run --bin smart-diff-server > backend.log 2>&1 &
    BACKEND_PID=$!
    
    # Start frontend
    echo -e "${GREEN}Starting frontend...${NC}"
    cd nextjs-frontend
    npm run dev > ../frontend.log 2>&1 &
    FRONTEND_PID=$!
    cd ..
    
    # Wait a bit for services to start
    sleep 3
    
    echo -e "\n${GREEN}✓ Services started!${NC}"
    echo -e "${BLUE}Backend:  http://localhost:8080 (PID: $BACKEND_PID)${NC}"
    echo -e "${BLUE}Frontend: http://localhost:3000 (PID: $FRONTEND_PID)${NC}"
    echo -e "\n${YELLOW}Logs:${NC}"
    echo -e "  Backend:  tail -f backend.log"
    echo -e "  Frontend: tail -f frontend.log"
    echo -e "\n${YELLOW}To stop:${NC}"
    echo -e "  kill $BACKEND_PID $FRONTEND_PID"
    echo -e "  or run: ./stop.sh"
fi

