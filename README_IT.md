# Pocket Web Backend

Un backend web sicuro e ad alte prestazioni costruito con Rust e Actix Web, dotato di rate limiting avanzato e gestione sicura delle sessioni crittograficamente protette.

## 🚀 Caratteristiche

- **Sistema di Autenticazione Sicuro** con session ID crittograficamente sicuri
- **Rate Limiting Avanzato** con tracciamento basato su IP e sessioni
- **API RESTful** con copertura completa degli endpoint
- **Alte Prestazioni** costruito con Rust e Actix Web
- **Architettura Thread-Safe** con meccanismi di pulizia automatica
- **Sicurezza Completa** contro brute force, DoS e altri attacchi
- **Servizio File Statici** per integrazione frontend web
- **Supporto Cross-Platform** (Linux, macOS, Windows)

## 🛡️ Caratteristiche di Sicurezza

### Sistema di Rate Limiting

L'applicazione implementa un sofisticato rate limiting per proteggere contro vari attacchi:

#### Protezione Endpoint Critici
- **Login** (`/v5/pocket/login`): 5 tentativi ogni 5 minuti
- **Registrazione** (`/v5/pocket/registration`): 3 registrazioni all'ora
- **Cambio Password** (`/v5/pocket/change_passwd`): 3 cambi all'ora
- **Heartbeat** (`/v5/pocket/heartbeat`): 6 richieste al minuto

#### Protezione API Generale
- **Limite Predefinito**: 1000 richieste all'ora per tutti gli altri endpoint `/v5/pocket/`

#### Sistema di Tracciamento Doppio
Il rate limiter traccia le richieste utilizzando entrambi:
- **Indirizzo IP**: Previene attacchi da singola fonte
- **Session ID**: Previene abusi basati su sessioni

#### Architettura
```rust
pub struct RateLimiter {
    ip_requests: Arc<Mutex<HashMap<IpAddr, HashMap<String, RequestEntry>>>>,
    session_requests: Arc<Mutex<HashMap<String, HashMap<String, RequestEntry>>>>,
    endpoint_limits: HashMap<String, RateLimit>,
}
```

### Generazione Sicura dei Session ID

L'applicazione utilizza un sistema di generazione session ID crittograficamente sicuro:

#### Implementazione Precedente vs Attuale
| Aspetto | Precedente (ULID) | Attuale (SHA256 Sicuro) |
|---------|-------------------|--------------------------|
| Lunghezza | 26 caratteri | 64 caratteri |
| Sicurezza | Media | Alta |
| Predicibilità | Bassa | Nulla |
| Tempo Generazione | ~100ns | ~5μs |

#### Fonti di Entropia
Il generatore sicuro combina multiple fonti di entropia:
1. **Timestamp ad alta risoluzione**: `SystemTime::now().as_nanos()`
2. **Process ID**: Identificatore del processo corrente
3. **Entropia di sistema**: 32 byte dal SO (`getrandom`)
4. **Contatore sequenziale**: Previene collisioni in generazioni simultanee
5. **Byte casuali**: Ulteriori 32 byte di casualità

#### Caratteristiche di Sicurezza
- **Resistenza ad attacchi di predizione**: Impossibile predire session ID futuri
- **Protezione brute force**: 2^256 combinazioni possibili
- **Resistenza alle collisioni**: Forza crittografica SHA256
- **Thread safety**: Generatore globale protetto da mutex

## 📋 Requisiti

- **Rust**: 1.70 o successivo (edizione 2024)
- **CMake**: Per compilare componenti nativi
- **Git**: Per controllo versione

## 🛠️ Installazione

### Dipendenze di Sistema (Debian 12)

Prima di compilare il progetto, installa i pacchetti di sistema richiesti:

```bash
# Aggiorna l'elenco dei pacchetti
sudo apt update

# Installa gli strumenti di build essenziali
sudo apt install -y build-essential git

# Installa la toolchain Rust (se non già installata)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Installa CMake e pkg-config (richiesti per la compilazione del bridge C++)
sudo apt install -y cmake pkg-config

# Installa le librerie di sviluppo OpenSSL
sudo apt install -y libssl-dev

# Installa le librerie di sviluppo SQLite3
sudo apt install -y libsqlite3-dev

# Installa librerie di sviluppo aggiuntive (opzionali ma consigliate)
sudo apt install -y libc6-dev
```

#### Dettagli Pacchetti

| Pacchetto | Scopo | Richiesto da |
|-----------|-------|--------------|
| `build-essential` | Compilatore GCC/G++, make e strumenti di build base | Compilazione bridge C++ |
| `cmake` | Generatore di sistema di build | Build CMake di pocket-lib |
| `pkg-config` | Strumento di configurazione pacchetti | Rilevamento librerie in CMake |
| `libssl-dev` | Header e librerie di sviluppo OpenSSL | Operazioni crittografiche |
| `libsqlite3-dev` | Header e librerie di sviluppo SQLite3 | Operazioni database |
| `libc6-dev` | File di sviluppo libreria C standard | Compilazione generale C/C++ |
| `git` | Sistema di controllo versione | Gestione codice sorgente |

#### Verifica

Puoi verificare le installazioni con:

```bash
# Controlla versioni compilatori
gcc --version
g++ --version
cmake --version

# Controlla librerie
pkg-config --modversion openssl
pkg-config --modversion sqlite3

# Controlla installazione Rust
rustc --version
cargo --version
```

### Clona il Repository
```bash
git clone https://github.com/passy1977/pocket-web-backend.git
cd pocket-web-backend
```

### Compila il Progetto
```bash
# Compilazione in modalità debug
cargo build

# Compilazione in modalità release (consigliata per produzione)
cargo build --release
```

### Esegui i Test
```bash
# Esegui tutti i test
cargo test

# Esegui moduli di test specifici
cargo test rate_limiter
cargo test secure_session
```

## 🎯 Utilizzo

### Opzioni Linea di Comando
```bash
# Mostra l'aiuto
cargo run -- --help

# Esegui con impostazioni predefinite
cargo run

# Esegui in modalità release
cargo run --release
```

### Configurazione
L'applicazione può essere configurata tramite:
- Argomenti della linea di comando
- Variabili d'ambiente
- File di configurazione (se implementati)

### Avvio del Server
```bash
# Modalità sviluppo
cargo run

# Modalità produzione
cargo run --release
```

Il server si avvierà sull'indirizzo e porta configurati (predefinito: `127.0.0.1:8080`).

## 📡 Endpoint API

### Endpoint di Autenticazione

#### Login
```http
POST /v5/pocket/login
Content-Type: application/json

{
    "session_id": "string",
    "username": "string",
    "password": "string"
}
```

#### Registrazione
```http
POST /v5/pocket/registration
Content-Type: application/json

{
    "session_id": "string",
    "username": "string", 
    "password": "string",
    "email": "string"
}
```

#### Logout
```http
POST /v5/pocket/logout
Content-Type: application/json

{
    "session_id": "string"
}
```

#### Cambio Password
```http
POST /v5/pocket/change_passwd
Content-Type: application/json

{
    "session_id": "string",
    "old_password": "string",
    "new_password": "string"
}
```

### Endpoint di Sistema

#### Heartbeat
```http
GET /v5/pocket/heartbeat/{session_id}
```

#### Hello
```http
GET /v5/pocket/hello/{session_id}
```

### Endpoint Gestione Dati

#### Importa Dati
```http
POST /v5/pocket/import_data
Content-Type: application/json

{
    "session_id": "string",
    "data": "object"
}
```

#### Ottieni Dati
```http
GET /v5/pocket/data/{session_id}
```

### Gestione Campi

#### Dettagli Campo
```http
GET /v5/pocket/field_detail/{session_id}/{field_id}
```

### Gestione Gruppi

#### Dettagli Gruppo
```http
GET /v5/pocket/group_detail/{session_id}/{group_id}
```

### Risposte di Errore

#### Rate Limit Superato
```json
{
    "error": "Rate limit exceeded. Too many requests from your IP.",
    "retry_after": 60
}
```

```json
{
    "error": "Rate limit exceeded. Too many requests for this session.", 
    "retry_after": 60
}
```

## 🏗️ Architettura

### Struttura del Progetto
```
src/
├── main.rs              # Punto di ingresso dell'applicazione
├── models/              # Modelli di dati
│   ├── user.rs
│   ├── field.rs
│   ├── group.rs
│   └── data_transport.rs
├── rest/                # Controller REST
│   ├── rest_controller.rs
│   ├── rest_controller_login.rs
│   ├── rest_controller_registration.rs
│   └── ...
├── services/            # Servizi logica di business
│   ├── http_server.rs
│   ├── rate_limiter.rs
│   ├── secure_session.rs
│   └── session.rs
```

### Componenti Chiave

#### Rate Limiter
- **Thread-safe**: Utilizza `Arc<Mutex<>>` per accesso concorrente
- **Pulizia automatica**: Task in background rimuove entry scadute
- **Limiti configurabili**: Regole di rate limiting per endpoint
- **Efficiente in memoria**: Storage basato su HashMap con scadenza automatica

#### Session Manager
- **Generazione sicura**: ID crittograficamente sicuri basati su SHA256
- **Singleton globale**: `LazyLock` per inizializzazione thread-safe
- **Alta entropia**: Multiple fonti di casualità combinate

#### HTTP Server
- **Framework Actix Web**: Server web asincrono ad alte prestazioni
- **Supporto CORS**: Cross-origin resource sharing abilitato
- **Servizio file statici**: Consegna asset frontend
- **Gestione errori**: Sistema di risposta errori completo

## 🧪 Testing

### Copertura Test
Il progetto include test completi per:

#### Test Rate Limiter
- ✅ Creazione e configurazione rate limit
- ✅ Limitazione basata su IP
- ✅ Limitazione basata su sessioni
- ✅ Separazione endpoint
- ✅ Scadenza entry richieste
- ✅ Funzionalità rate limiter globale

#### Test Sessioni Sicure
- ✅ Verifica univocità (1000+ ID)
- ✅ Validazione formato (esadecimale 64-char)
- ✅ Test entropia
- ✅ Test generatore globale
- ✅ Verifica forza sicurezza

### Esecuzione Test
```bash
# Tutti i test
cargo test

# Moduli specifici
cargo test rate_limiter
cargo test secure_session

# Con output
cargo test -- --nocapture

# Test modalità release
cargo test --release
```

## 🚀 Prestazioni

### Benchmark
- **Rate Limiter**: <1ms overhead per richiesta
- **Generazione Sessione**: ~5μs per generazione ID
- **Uso Memoria**: ~100 byte per IP/sessione tracciata
- **Pulizia**: Automatica ogni 5 minuti

### Caratteristiche di Ottimizzazione
- **Strutture dati efficienti**: Lookup basati su HashMap (O(1))
- **Allocazioni minime**: Riuso strutture dati dove possibile
- **Pulizia in background**: Gestione automatica memoria
- **Design thread-safe**: Operazioni lock-free dove possibile

## 🔧 Configurazione

### Configurazione Rate Limiting
```rust
// Modifica limiti in RateLimiter::new()
endpoint_limits.insert("/v5/pocket/login".to_string(), RateLimit::new(5, 300));
endpoint_limits.insert("/v5/pocket/registration".to_string(), RateLimit::new(3, 3600));
```

### Aggiunta Nuovi Endpoint
```rust
endpoint_limits.insert("/v5/pocket/new_endpoint".to_string(), RateLimit::new(10, 600));
```

### Intervallo Pulizia
```rust
// Modifica intervallo pulizia (predefinito: 5 minuti)
let mut cleanup_interval = interval(TokioDuration::from_secs(300));
```

## 🔒 Best Practice di Sicurezza

### Protezioni Implementate
- ✅ **Protezione Brute Force**: Limitazione login, registrazione, cambio password
- ✅ **Mitigazione DoS/DDoS**: Limitazione richieste generale
- ✅ **Prevenzione Esaurimento Risorse**: Limitazione heartbeat
- ✅ **Protezione Spam**: Limitazione registrazione e API
- ✅ **Sicurezza Sessioni**: Session ID crittograficamente sicuri
- ✅ **Validazione Input**: Validazione schema JSON
- ✅ **Gestione Errori**: Risposte errore sicure

### Resistenza Contro
- **IP Spoofing**: Protetto da infrastruttura
- **Attacchi Rotazione Sessioni**: Tracciamento basato su sessioni
- **Attacchi Distribuiti**: Efficace contro attacchi da singola fonte
- **Attacchi Livello Applicazione**: Protezione L7
- **Attacchi di Predizione**: Casualità crittograficamente sicura

## 📈 Monitoraggio e Metriche

### Monitoraggio Suggerito
- Numero richieste bloccate per endpoint
- Top IP con violazioni rate limit
- Pattern di accesso temporali
- Ratio efficacia rate limiting
- Prestazioni generazione sessioni
- Trend uso memoria

### Implementazione Logging Futura
```rust
log::warn!("Rate limit exceeded for IP {} on endpoint {}", ip, endpoint);
log::info!("Rate limiter stats: {} active IPs, {} active sessions", ip_count, session_count);
```

## 🚧 Miglioramenti Futuri

### Caratteristiche Pianificate
- [ ] Supporto token JWT
- [ ] Sistema logging completo

### Miglioramenti Scalabilità
- [ ] Supporto scaling orizzontale
- [ ] Integrazione load balancer
- [ ] Deployment container
- [ ] Architettura microservizi

## 🤝 Contribuire

1. Fork del repository
2. Crea un branch per la feature
3. Apporta le tue modifiche
4. Aggiungi test per nuove funzionalità
5. Assicurati che tutti i test passino
6. Invia una pull request

### Linee Guida Sviluppo
- Segui gli standard di codifica Rust
- Scrivi test comprensivi
- Aggiorna la documentazione
- Usa messaggi di commit significativi
- Assicura le best practice di sicurezza

## 📄 Licenza

Questo progetto è licenziato secondo i termini specificati nel file LICENSE.

## 👥 Autori

- Antonio Salsi (@passy1977)

## 🆘 Supporto

Per supporto, per favore:
1. Controlla la documentazione esistente
2. Cerca tra le issue
3. Crea una nuova issue con informazioni dettagliate
4. Includi log e configurazione rilevanti

## 🔗 Progetti Correlati

- [pocket-lib](https://github.com/passy1977/pocket-lib) - Core C++ library

---

**Nota**: Questo backend è progettato per funzionare con l'ecosistema applicativo Pocket e fornisce servizi API sicuri e scalabili con meccanismi di protezione completi.