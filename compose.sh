#!/bin/bash

# Wrapper script for Docker Compose / Podman Compose
# Automatically detects which tool to use

# Function to detect available compose runtime
detect_compose_runtime() {
    if command -v podman-compose >/dev/null 2>&1; then
        echo "podman-compose"
    elif command -v docker-compose >/dev/null 2>&1; then
        echo "docker-compose"
    elif command -v docker >/dev/null 2>&1 && docker compose version >/dev/null 2>&1; then
        echo "docker compose"
    else
        echo "none"
    fi
}

# Detect compose runtime
COMPOSE_RUNTIME=$(detect_compose_runtime)

if [ "$COMPOSE_RUNTIME" = "none" ]; then
    echo "❌ Error: Neither Docker Compose nor Podman Compose are available"
    echo "Please install one of the following:"
    echo "  - Docker Compose: https://docs.docker.com/compose/install/"
    echo "  - Podman Compose: pip install podman-compose"
    exit 1
fi

# Help function
show_help() {
    echo "🐳 Container Compose Wrapper"
    echo "Detected runtime: $COMPOSE_RUNTIME"
    echo ""
    echo "Usage: $0 <command> [options]"
    echo ""
    echo "Main commands:"
    echo "  up              - Start services"
    echo "  down            - Stop services"
    echo "  build           - Build images"
    echo "  ps              - List services"
    echo "  logs            - Show logs"
    echo "  --profile dev   - Use development profile"
    echo ""
    echo "Examples:"
    echo "  $0 up                    # Start main service"
    echo "  $0 --profile dev up      # Start development services too"
    echo "  $0 down                  # Stop all services"
    echo "  $0 logs -f               # Follow logs in real-time"
    echo ""
    echo "Supported environment variables:"
    echo "  POCKET_ADDRESS, POCKET_PORT, POCKET_MAX_THREADS, POCKET_SESSION_EXPIRATION"
}

# If no arguments or help is requested
if [ $# -eq 0 ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
    exit 0
fi

# Pass all arguments to the compose runtime
echo "🐳 Executing: $COMPOSE_RUNTIME $*"
exec $COMPOSE_RUNTIME "$@"