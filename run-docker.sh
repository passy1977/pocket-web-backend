#!/bin/bash

# Example script to run the container with custom configuration
# Automatically supports Docker and Podman

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

echo "🐳 Using container runtime: $CONTAINER_RUNTIME"

# Default values
ADDRESS="0.0.0.0"
PORT="8080"
MAX_THREADS="2"
SESSION_EXPIRATION="300"

# Argument parsing
while [[ $# -gt 0 ]]; do
    case $1 in
        --address)
            ADDRESS="$2"
            shift 2
            ;;
        --port)
            PORT="$2"
            shift 2
            ;;
        --max-threads)
            MAX_THREADS="$2"
            shift 2
            ;;
        --session-expiration)
            SESSION_EXPIRATION="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --address ADDRESS          Bind address (default: 0.0.0.0)"
            echo "  --port PORT               Listening port (default: 8080)"
            echo "  --max-threads THREADS     Maximum number of threads (default: 2)"
            echo "  --session-expiration SEC  Session expiration time in seconds (default: 300)"
            echo "  --help                    Show this message"
            echo ""
            echo "Detected container runtime: $CONTAINER_RUNTIME"
            echo ""
            echo "The application supports both long and short flags:"
            echo "  -a, --address    -p, --port    -m, --max-threads    -s, --session-expiration-time"
            echo ""
            echo "Examples:"
            echo "  $0 --address 192.168.1.100 --port 9090"
            echo "  $0 --port 3000 --max-threads 4"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help to see available options"
            exit 1
            ;;
    esac
done

echo "Starting pocket-web-backend container with $CONTAINER_RUNTIME:"
echo "  Address: $ADDRESS"
echo "  Port: $PORT"
echo "  Max Threads: $MAX_THREADS"
echo "  Session Expiration: $SESSION_EXPIRATION seconds"
echo "  Backend URL (constants.mjs): http://$ADDRESS:$PORT"
echo ""

# Build image if it doesn't exist
if ! $CONTAINER_RUNTIME image inspect pocket-web-backend >/dev/null 2>&1; then
    echo "Building $CONTAINER_RUNTIME image..."
    $CONTAINER_RUNTIME build -t pocket-web-backend .
fi

# Run the container
$CONTAINER_RUNTIME run -it --rm \
    -p ${PORT}:${PORT} \
    -e POCKET_ADDRESS=${ADDRESS} \
    -e POCKET_PORT=${PORT} \
    -e POCKET_MAX_THREADS=${MAX_THREADS} \
    -e POCKET_SESSION_EXPIRATION=${SESSION_EXPIRATION} \
    --name pocket-web-backend-instance \
    pocket-web-backend

# Costruisci l'immagine se non esiste
if ! $CONTAINER_RUNTIME image inspect pocket-web-backend >/dev/null 2>&1; then
    echo "Costruzione dell'immagine $CONTAINER_RUNTIME..."
    $CONTAINER_RUNTIME build -t pocket-web-backend .
fi

# Esegui il container
$CONTAINER_RUNTIME run -it --rm \
    -p ${PORT}:${PORT} \
    -e POCKET_ADDRESS=${ADDRESS} \
    -e POCKET_PORT=${PORT} \
    -e POCKET_MAX_THREADS=${MAX_THREADS} \
    -e POCKET_SESSION_EXPIRATION=${SESSION_EXPIRATION} \
    --name pocket-web-backend-instance \
    pocket-web-backend