#!/bin/bash
# Smart Code Diff - One-Command Installer
# This script sets up everything you need to run Smart Code Diff

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║                                                       ║"
echo "║       Smart Code Diff - One-Command Installer        ║"
echo "║                                                       ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""

# Check prerequisites
echo -e "${BLUE}Checking prerequisites...${NC}"

if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${RED}✗ Rust/Cargo not found${NC}"
    echo -e "${YELLOW}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed${NC}"
else
    echo -e "${GREEN}✓ Rust/Cargo found${NC}"
fi

if ! command -v node >/dev/null 2>&1; then
    echo -e "${RED}✗ Node.js not found${NC}"
    echo -e "${YELLOW}Please install Node.js from https://nodejs.org/${NC}"
    echo -e "${YELLOW}Or use a version manager like nvm:${NC}"
    echo -e "  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash"
    echo -e "  nvm install 18"
    exit 1
else
    echo -e "${GREEN}✓ Node.js found ($(node --version))${NC}"
fi

if ! command -v npm >/dev/null 2>&1; then
    echo -e "${RED}✗ npm not found${NC}"
    echo -e "${YELLOW}Please install Node.js from https://nodejs.org/${NC}"
    exit 1
else
    echo -e "${GREEN}✓ npm found ($(npm --version))${NC}"
fi

# Make scripts executable
echo -e "\n${BLUE}Making scripts executable...${NC}"
chmod +x start.sh dev.sh stop.sh
echo -e "${GREEN}✓ Scripts are executable${NC}"

# Install frontend dependencies
echo -e "\n${BLUE}Installing frontend dependencies...${NC}"
cd nextjs-frontend
npm install
echo -e "${GREEN}✓ Frontend dependencies installed${NC}"
cd ..

# Build backend
echo -e "\n${BLUE}Building Rust backend (this may take a few minutes)...${NC}"
cargo build --release --bin smart-diff-server
echo -e "${GREEN}✓ Backend built successfully${NC}"

# Success!
echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║                                                       ║"
echo "║              ✓ Installation Complete!                ║"
echo "║                                                       ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}You're all set!${NC}\n"
echo -e "${BLUE}To start Smart Code Diff:${NC}"
echo -e "  ${YELLOW}./start.sh${NC}     - Full start with checks"
echo -e "  ${YELLOW}./dev.sh${NC}       - Quick development start"
echo -e "  ${YELLOW}make start${NC}     - Using Makefile"
echo ""
echo -e "${BLUE}To stop services:${NC}"
echo -e "  ${YELLOW}./stop.sh${NC}      - Stop all services"
echo -e "  ${YELLOW}make stop${NC}      - Using Makefile"
echo ""
echo -e "${BLUE}For more information:${NC}"
echo -e "  ${YELLOW}cat QUICK_START.md${NC}"
echo ""

