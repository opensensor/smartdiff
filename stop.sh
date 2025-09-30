#!/bin/bash
# Smart Code Diff - Stop All Services

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Stopping Smart Code Diff services...${NC}\n"

# Kill processes on ports
kill_port() {
    local port=$1
    local name=$2
    
    if lsof -i :$port >/dev/null 2>&1; then
        echo -e "${YELLOW}Stopping $name on port $port...${NC}"
        lsof -ti:$port | xargs kill -9 2>/dev/null || true
        echo -e "${GREEN}✓ $name stopped${NC}"
    else
        echo -e "${GREEN}✓ $name not running${NC}"
    fi
}

# Stop backend (port 8080)
kill_port 8080 "Backend"

# Stop frontend (port 3000)
kill_port 3000 "Frontend"

# Kill tmux session if exists
if tmux has-session -t smartdiff 2>/dev/null; then
    echo -e "${YELLOW}Stopping tmux session...${NC}"
    tmux kill-session -t smartdiff
    echo -e "${GREEN}✓ tmux session stopped${NC}"
fi

echo -e "\n${GREEN}All services stopped!${NC}"

