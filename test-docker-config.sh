#!/bin/bash

# Script di test per verificare la configurazione automatica del BACKEND_URL
# Supporta automaticamente Docker e Podman

# Funzione per rilevare il runtime container disponibile
detect_container_runtime() {
    if command -v podman >/dev/null 2>&1; then
        echo "podman"
    elif command -v docker >/dev/null 2>&1; then
        echo "docker"
    else
        echo "none"
    fi
}

# Rileva il runtime container
CONTAINER_RUNTIME=$(detect_container_runtime)

if [ "$CONTAINER_RUNTIME" = "none" ]; then
    echo "❌ Errore: Né Docker né Podman sono installati sul sistema"
    echo "Installa uno dei due per continuare:"
    echo "  - Docker: https://docs.docker.com/get-docker/"
    echo "  - Podman: https://podman.io/getting-started/installation"
    exit 1
fi

echo "🧪 Test della configurazione automatica BACKEND_URL"
echo "Runtime container: $CONTAINER_RUNTIME"
echo "================================================"

# Test 1: Configurazione di default
echo ""
echo "📋 Test 1: Configurazione di default (localhost:8080)"
echo "Building image..."
$CONTAINER_RUNTIME build -t pocket-web-backend . > /dev/null 2>&1

echo "Starting container in background..."
CONTAINER_ID=$($CONTAINER_RUNTIME run -d --name pocket-test-default pocket-web-backend)

# Aspetta un momento per l'avvio
sleep 2

echo "Checking constants.mjs content..."
$CONTAINER_RUNTIME exec $CONTAINER_ID cat /var/www/statics/js/constants.mjs | grep "BACKEND_URL"

echo "Stopping container..."
$CONTAINER_RUNTIME stop $CONTAINER_ID > /dev/null 2>&1
$CONTAINER_RUNTIME rm $CONTAINER_ID > /dev/null 2>&1

# Test 2: Configurazione personalizzata
echo ""
echo "📋 Test 2: Configurazione personalizzata (192.168.1.100:9090)"
CONTAINER_ID=$($CONTAINER_RUNTIME run -d \
    -e POCKET_ADDRESS=192.168.1.100 \
    -e POCKET_PORT=9090 \
    --name pocket-test-custom \
    pocket-web-backend)

# Aspetta un momento per l'avvio
sleep 2

echo "Checking constants.mjs content..."
$CONTAINER_RUNTIME exec $CONTAINER_ID cat /var/www/statics/js/constants.mjs | grep "BACKEND_URL"

echo "Stopping container..."
$CONTAINER_RUNTIME stop $CONTAINER_ID > /dev/null 2>&1
$CONTAINER_RUNTIME rm $CONTAINER_ID > /dev/null 2>&1

# Test 3: Test con lo script run-docker.sh
echo ""
echo "📋 Test 3: Test con script run-docker.sh"
echo "Questo test avvierà il container interattivamente per 10 secondi..."
echo "Premi Ctrl+C dopo aver verificato che tutto funzioni correttamente."

timeout 10s ./run-docker.sh --address 127.0.0.1 --port 3000 || true

echo ""
echo "✅ Test completati!"
echo ""
echo "💡 Per verificare manualmente:"
echo "   1. Avvia: ./run-docker.sh --address TUO_IP --port TUA_PORTA"
echo "   2. In un altro terminale: $CONTAINER_RUNTIME exec -it pocket-web-backend-instance cat /var/www/statics/js/constants.mjs"
echo "   3. Verifica che BACKEND_URL contenga il tuo IP e porta"