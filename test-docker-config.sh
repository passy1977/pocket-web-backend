#!/bin/bash

# Test script to verify automatic BACKEND_URL configuration
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

echo "🧪 Testing automatic BACKEND_URL configuration"
echo "Container runtime: $CONTAINER_RUNTIME"
echo "================================================"

# Test 1: Default configuration
echo ""
echo "📋 Test 1: Default configuration (localhost:8080)"
echo "Building image..."
$CONTAINER_RUNTIME build -t pocket-web-backend . > /dev/null 2>&1

echo "Starting container in background..."
CONTAINER_ID=$($CONTAINER_RUNTIME run -d --name pocket-test-default pocket-web-backend)

# Wait a moment for startup
sleep 2

echo "Checking constants.mjs content..."
$CONTAINER_RUNTIME exec $CONTAINER_ID cat /var/www/statics/js/constants.mjs | grep "BACKEND_URL"

echo "Stopping container..."
$CONTAINER_RUNTIME stop $CONTAINER_ID > /dev/null 2>&1
$CONTAINER_RUNTIME rm $CONTAINER_ID > /dev/null 2>&1

# Test 2: Custom configuration
echo ""
echo "📋 Test 2: Custom configuration (192.168.1.100:9090)"
CONTAINER_ID=$($CONTAINER_RUNTIME run -d \
    -e POCKET_ADDRESS=192.168.1.100 \
    -e POCKET_PORT=9090 \
    --name pocket-test-custom \
    pocket-web-backend)

# Wait a moment for startup
sleep 2

echo "Checking constants.mjs content..."
$CONTAINER_RUNTIME exec $CONTAINER_ID cat /var/www/statics/js/constants.mjs | grep "BACKEND_URL"

echo "Stopping container..."
$CONTAINER_RUNTIME stop $CONTAINER_ID > /dev/null 2>&1
$CONTAINER_RUNTIME rm $CONTAINER_ID > /dev/null 2>&1

# Test 3: Test with run-docker.sh script
echo ""
echo "📋 Test 3: Test with run-docker.sh script"
echo "This test will start the container interactively for 10 seconds..."
echo "Press Ctrl+C after verifying everything works correctly."

timeout 10s ./run-docker.sh --address 127.0.0.1 --port 3000 || true

echo ""
echo "✅ Tests completed!"
echo ""
echo "💡 To verify manually:"
echo "   1. Start: ./run-docker.sh --address YOUR_IP --port YOUR_PORT"
echo "   2. In another terminal: $CONTAINER_RUNTIME exec -it pocket-web-backend-instance cat /var/www/statics/js/constants.mjs"
echo "   3. Verify that BACKEND_URL contains your IP and port"