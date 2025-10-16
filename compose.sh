#!/bin/bash

# Script wrapper per Docker Compose / Podman Compose
# Rileva automaticamente quale strumento utilizzare

# Funzione per rilevare il runtime compose disponibile
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

# Rileva il runtime compose
COMPOSE_RUNTIME=$(detect_compose_runtime)

if [ "$COMPOSE_RUNTIME" = "none" ]; then
    echo "❌ Errore: Né Docker Compose né Podman Compose sono disponibili"
    echo "Installa uno dei seguenti:"
    echo "  - Docker Compose: https://docs.docker.com/compose/install/"
    echo "  - Podman Compose: pip install podman-compose"
    exit 1
fi

# Funzione di aiuto
show_help() {
    echo "🐳 Container Compose Wrapper"
    echo "Runtime rilevato: $COMPOSE_RUNTIME"
    echo ""
    echo "Uso: $0 <comando> [opzioni]"
    echo ""
    echo "Comandi principali:"
    echo "  up              - Avvia i servizi"
    echo "  down            - Ferma i servizi"
    echo "  build           - Costruisci le immagini"
    echo "  ps              - Lista i servizi"
    echo "  logs            - Mostra i log"
    echo "  --profile dev   - Usa il profilo development"
    echo ""
    echo "Esempi:"
    echo "  $0 up                    # Avvia il servizio principale"
    echo "  $0 --profile dev up      # Avvia anche il servizio dev"
    echo "  $0 down                  # Ferma tutti i servizi"
    echo "  $0 logs -f               # Segui i log in tempo reale"
    echo ""
    echo "Variabili d'ambiente supportate:"
    echo "  POCKET_ADDRESS, POCKET_PORT, POCKET_MAX_THREADS, POCKET_SESSION_EXPIRATION"
}

# Se non ci sono argomenti o è richiesto help
if [ $# -eq 0 ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
    exit 0
fi

# Passa tutti gli argomenti al runtime compose
echo "🐳 Eseguendo: $COMPOSE_RUNTIME $*"
exec $COMPOSE_RUNTIME "$@"