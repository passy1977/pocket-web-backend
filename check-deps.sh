#!/bin/bash

# Quick dependency check script for pocket-web-backend

echo "🔍 Checking pocket-web-backend dependencies..."
echo "============================================"

ALL_OK=true

# Function to check command availability
check_command() {
    local cmd="$1"
    local install_hint="$2"
    
    if command -v "$cmd" >/dev/null 2>&1; then
        echo "✅ $cmd: $(${cmd} --version 2>/dev/null | head -n1)"
    else
        echo "❌ $cmd: NOT FOUND"
        echo "   Install with: $install_hint"
        ALL_OK=false
    fi
}

# Function to check library availability
check_library() {
    local lib="$1"
    local install_hint="$2"
    
    if pkg-config --exists "$lib" 2>/dev/null; then
        echo "✅ $lib: $(pkg-config --modversion $lib 2>/dev/null)"
    else
        echo "❌ $lib: NOT FOUND"
        echo "   Install with: $install_hint"
        ALL_OK=false
    fi
}

# Function to check file/library existence
check_file() {
    local file_pattern="$1"
    local name="$2"
    local install_hint="$3"
    
    if ldconfig -p 2>/dev/null | grep -q "$file_pattern"; then
        echo "✅ $name: FOUND"
    else
        echo "❌ $name: NOT FOUND"
        echo "   Install with: $install_hint"
        ALL_OK=false
    fi
}

echo ""
echo "📋 Core Tools:"
check_command "gcc" "sudo apt install build-essential"
check_command "g++" "sudo apt install build-essential"
check_command "clang" "sudo apt install clang"
check_command "cmake" "sudo apt install cmake"
check_command "pkg-config" "sudo apt install pkg-config"
check_command "git" "sudo apt install git"
check_command "curl" "sudo apt install curl"

echo ""
echo "🦀 Rust Toolchain:"
check_command "rustc" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
check_command "cargo" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
check_command "rustup" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"

echo ""
echo "📚 Development Libraries:"
check_library "openssl" "sudo apt install libssl-dev"
check_library "sqlite3" "sudo apt install libsqlite3-dev"
check_file "libclang" "libclang" "sudo apt install libclang-dev llvm-dev"

echo ""
echo "🐳 Container Runtimes (Optional):"
if command -v docker >/dev/null 2>&1; then
    echo "✅ docker: $(docker --version 2>/dev/null)"
elif command -v podman >/dev/null 2>&1; then
    echo "✅ podman: $(podman --version 2>/dev/null)"
else
    echo "⚠️  No container runtime found (Docker/Podman)"
    echo "   Install one with: https://docs.docker.com/get-docker/ or https://podman.io/"
fi

echo ""
echo "============================================"
if [ "$ALL_OK" = true ]; then
    echo "🎉 All dependencies are satisfied!"
    echo "   You can now run: cargo build --release"
    echo "   Or try Docker build: ./run-docker.sh"
else
    echo "❌ Some dependencies are missing."
    echo "   Install the missing packages and run this script again."
    echo "   For a complete setup, run:"
    echo "   sudo apt update && sudo apt install -y build-essential cmake pkg-config clang libclang-dev llvm-dev libssl-dev libsqlite3-dev git curl"
fi