#!/bin/bash

# Script wrapper universale per gestire Docker/Podman
# Questo script può essere utilizzato come alias per docker/podman commands

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

# Funzione di aiuto
show_help() {
    echo "🐳 Container Runtime Wrapper"
    echo "Runtime rilevato: $CONTAINER_RUNTIME"
    echo ""
    echo "Uso: $0 <comando> [opzioni]"
    echo ""
    echo "Comandi disponibili:"
    echo "  build <args>     - Costruisci un'immagine container"
    echo "  run <args>       - Esegui un container"
    echo "  ps <args>        - Lista i container in esecuzione"
    echo "  images <args>    - Lista le immagini disponibili"
    echo "  stop <args>      - Ferma un container"
    echo "  rm <args>        - Rimuovi un container"
    echo "  rmi <args>       - Rimuovi un'immagine"
    echo "  exec <args>      - Esegui un comando in un container"
    echo "  logs <args>      - Mostra i log di un container"
    echo "  help             - Mostra questo messaggio"
    echo ""
    echo "Esempi:"
    echo "  $0 build -t myapp ."
    echo "  $0 run -p 8080:8080 myapp"
    echo "  $0 ps"
    echo ""
    echo "Nota: Tutti i comandi vengono passati direttamente a $CONTAINER_RUNTIME"
}

# Se non ci sono argomenti o è richiesto help
if [ $# -eq 0 ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    show_help
    exit 0
fi

# Passa tutti gli argomenti al runtime container
echo "🐳 Eseguendo: $CONTAINER_RUNTIME $*"
exec $CONTAINER_RUNTIME "$@"