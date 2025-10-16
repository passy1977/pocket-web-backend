#!/bin/bash

# Script di esempio per eseguire il container con configurazione personalizzata
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

echo "🐳 Usando runtime container: $CONTAINER_RUNTIME"

# Valori di default
ADDRESS="0.0.0.0"
PORT="8080"
MAX_THREADS="2"
SESSION_EXPIRATION="300"

# Parsing degli argomenti
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
            echo "Uso: $0 [opzioni]"
            echo "Opzioni:"
            echo "  --address ADDRESS          Indirizzo di bind (default: 0.0.0.0)"
            echo "  --port PORT               Porta di ascolto (default: 8080)"
            echo "  --max-threads THREADS     Numero massimo di thread (default: 2)"
            echo "  --session-expiration SEC  Tempo scadenza sessione in secondi (default: 300)"
            echo "  --help                    Mostra questo messaggio"
            echo ""
            echo "Runtime container rilevato: $CONTAINER_RUNTIME"
            echo ""
            echo "Esempi:"
            echo "  $0 --address 192.168.1.100 --port 9090"
            echo "  $0 --port 3000 --max-threads 4"
            exit 0
            ;;
        *)
            echo "Opzione sconosciuta: $1"
            echo "Usa --help per vedere le opzioni disponibili"
            exit 1
            ;;
    esac
done

echo "Avvio del container pocket-web-backend con $CONTAINER_RUNTIME:"
echo "  Address: $ADDRESS"
echo "  Port: $PORT"
echo "  Max Threads: $MAX_THREADS"
echo "  Session Expiration: $SESSION_EXPIRATION secondi"
echo "  Backend URL (constants.mjs): http://$ADDRESS:$PORT"
echo ""

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