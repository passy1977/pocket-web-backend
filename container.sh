#!/bin/bash

# Universal wrapper script for managing Docker/Podman
# This script can be used as an alias for docker/podman commands

# Function to detect available container runtime
detect_container_runtime() {
    # Check common paths for podman
    if command -v podman >/dev/null 2>&1; then
        echo "podman"
    elif [ -x "/usr/bin/podman" ]; then
        echo "/usr/bin/podman"
    elif [ -x "/usr/local/bin/podman" ]; then
        echo "/usr/local/bin/podman"
    # Check common paths for docker
    elif command -v docker >/dev/null 2>&1; then
        echo "docker"
    elif [ -x "/usr/bin/docker" ]; then
        echo "/usr/bin/docker"
    elif [ -x "/usr/local/bin/docker" ]; then
        echo "/usr/local/bin/docker"
    else
        echo "none"
    fi
}

# Detect container runtime
CONTAINER_RUNTIME=$(detect_container_runtime)

if [ "$CONTAINER_RUNTIME" = "none" ]; then
    echo "❌ Error: Neither Docker nor Podman are installed on this system"
    echo "Please install one of them to continue:"
    echo "  - Docker: https://docs.docker.com/get-docker/"
    echo "  - Podman: https://podman.io/getting-started/installation"
    exit 1
fi

# Help function
show_help() {
    echo "🐳 Container Runtime Wrapper"
    echo "Detected runtime: $CONTAINER_RUNTIME"
    echo ""
    echo "Usage: $0 <command> [options]"
    echo ""
    echo "Available commands:"
    echo "  build <args>     - Build a container image"
    echo "  run <args>       - Run a container"
    echo "  ps <args>        - List running containers"
    echo "  images <args>    - List available images"
    echo "  stop <args>      - Stop a container"
    echo "  rm <args>        - Remove a container"
    echo "  rmi <args>       - Remove an image"
    echo "  exec <args>      - Execute a command in a container"
    echo "  logs <args>      - Show container logs"
    echo "  help             - Show this message"
    echo ""
    echo "Examples:"
    echo "  $0 build -t myapp ."
    echo "  $0 run -p 8080:8080 myapp"
    echo "  $0 ps"
    echo ""
    echo "Note: All commands are passed directly to $CONTAINER_RUNTIME"
}

# If no arguments or help is requested
if [ $# -eq 0 ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
    exit 0
fi

# Pass all arguments to the container runtime
echo "🐳 Executing: $CONTAINER_RUNTIME $*"
exec $CONTAINER_RUNTIME "$@"