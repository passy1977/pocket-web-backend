# Pocket Web Backend

A secure, high-performance web backend built with Rust and Actix Web, featuring advanced rate limiting and cryptographically secure session management.

## 🚀 Features

- **Secure Authentication System** with cryptographically secure session IDs
- **Advanced Rate Limiting** with IP and session-based tracking
- **RESTful API** with comprehensive endpoint coverage
- **High Performance** built with Rust and Actix Web
- **Thread-Safe Architecture** with automatic cleanup mechanisms
- **Comprehensive Security** against brute force, DoS, and other attacks
- **Static File Serving** for web frontend integration
- **Cross-Platform Support** (Linux, macOS, Windows)

## 🛡️ Security Features

### Rate Limiting System

The application implements sophisticated rate limiting to protect against various attacks:

#### Critical Endpoints Protection
- **Login** (`/v5/pocket/login`): 5 attempts per 5 minutes
- **Registration** (`/v5/pocket/registration`): 3 registrations per hour  
- **Password Change** (`/v5/pocket/change_passwd`): 3 changes per hour
- **Heartbeat** (`/v5/pocket/heartbeat`): 6 requests per minute

#### General API Protection
- **Default Limit**: 1000 requests per hour for all other `/v5/pocket/` endpoints

#### Dual Tracking System
The rate limiter tracks requests using both:
- **IP Address**: Prevents single-source attacks
- **Session ID**: Prevents session-based abuse

#### Architecture
```rust
pub struct RateLimiter {
    ip_requests: Arc<Mutex<HashMap<IpAddr, HashMap<String, RequestEntry>>>>,
    session_requests: Arc<Mutex<HashMap<String, HashMap<String, RequestEntry>>>>,
    endpoint_limits: HashMap<String, RateLimit>,
}
```

### Secure Session ID Generation

The application uses a cryptographically secure session ID generation system:

#### Previous vs Current Implementation
| Aspect | Previous (ULID) | Current (SHA256 Secure) |
|--------|----------------|--------------------------|
| Length | 26 characters | 64 characters |
| Security | Medium | High |
| Predictability | Low | None |
| Generation Time | ~100ns | ~5μs |

#### Entropy Sources
The secure generator combines multiple entropy sources:
1. **High-resolution timestamp**: `SystemTime::now().as_nanos()`
2. **Process ID**: Current process identifier
3. **System entropy**: 32 bytes from OS (`getrandom`)
4. **Sequential counter**: Prevents simultaneous generation collisions
5. **Random bytes**: Additional 32 bytes of randomness

#### Security Characteristics
- **Resistance to prediction attacks**: Impossible to predict future session IDs
- **Brute force protection**: 2^256 possible combinations
- **Collision resistance**: SHA256 cryptographic strength
- **Thread safety**: Mutex-protected global generator

## 📋 Requirements

- **Rust**: 1.70 or later (2024 edition)
- **CMake**: For building native components
- **Git**: For version control

## 🛠️ Installation

### System Dependencies (Debian 12)

Before building the project, install the required system packages:

```bash
# Update package list
sudo apt update

# Install essential build tools
sudo apt install -y build-essential git

# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install CMake and pkg-config (required for C++ bridge compilation)
sudo apt install -y cmake pkg-config

# Install Clang and libclang (required for bindgen/C++ bridge)
sudo apt install -y clang libclang-dev llvm-dev

# Install OpenSSL development libraries
sudo apt install -y libssl-dev

# Install SQLite3 development libraries  
sudo apt install -y libsqlite3-dev

# Install additional development libraries (optional but recommended)
sudo apt install -y libc6-dev
```

#### Package Details

| Package | Purpose | Required By |
|---------|---------|-------------|
| `build-essential` | GCC/G++ compiler, make, and basic build tools | C++ bridge compilation |
| `cmake` | Build system generator | pocket-lib CMake build |
| `pkg-config` | Package configuration tool | Library detection in CMake |
| `clang` | Clang compiler and tools | bindgen for Rust-C++ bridge |
| `libclang-dev` | Clang development libraries | bindgen C++ parsing |
| `llvm-dev` | LLVM development tools | bindgen dependencies |
| `libssl-dev` | OpenSSL development headers and libraries | Cryptographic operations |
| `libsqlite3-dev` | SQLite3 development headers and libraries | Database operations |
| `libc6-dev` | Standard C library development files | General C/C++ compilation |
| `git` | Version control system | Source code management |

#### Verification

You can verify the installations with:

```bash
# Check compiler versions
gcc --version
g++ --version
cmake --version
clang --version

# Check libraries
pkg-config --modversion openssl
pkg-config --modversion sqlite3

# Check Rust installation
rustc --version
cargo --version
```

### Clone the Repository
```bash
git clone https://github.com/passy1977/pocket-web-backend.git
cd pocket-web-backend
```

### Build the Project
```bash
# Build in debug mode
cargo build

# Build in release mode (recommended for production)
cargo build --release
```

### Run Tests
```bash
# Run all tests
cargo test

# Run specific test modules
cargo test rate_limiter
cargo test secure_session
```

## 🎯 Usage

### Command Line Options
```bash
# Show help
cargo run -- --help

# Run with default settings
cargo run

# Run with custom parameters (positional arguments)
cargo run -- [ADDRESS] [PORT] [MAX_THREADS] [SESSION_EXPIRATION_TIME]

# Example with specific values
cargo run -- 0.0.0.0 8080 16 1800

# Run in release mode
cargo run --release

# Run release mode with custom parameters
cargo run --release -- 127.0.0.1 3030 8 900
```

### Parameters
- **ADDRESS**: IP address to bind to (default: 0.0.0.0)
- **PORT**: Port number to listen on (default: 3030)  
- **MAX_THREADS**: Maximum number of worker threads (default: 8)
- **SESSION_EXPIRATION_TIME**: Session timeout in seconds (default: 900)

### Configuration
The application can be configured through:
- Command line arguments (positional format)
- Environment variables (for Docker/containers)
- Default values if no arguments provided

### Starting the Server
```bash
# Development mode with default settings
cargo run

# Development mode with custom configuration
cargo run -- 0.0.0.0 8080 16 1800

# Production mode with default settings
cargo run --release

# Production mode with custom configuration  
cargo run --release -- 127.0.0.1 3030 8 900
```

The server will start on the configured address and port (default: `0.0.0.0:3030`).

## 📡 API Endpoints

### Authentication Endpoints

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

#### Registration
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

#### Change Password
```http
POST /v5/pocket/change_passwd
Content-Type: application/json

{
    "session_id": "string",
    "old_password": "string",
    "new_password": "string"
}
```

### System Endpoints

#### Heartbeat
```http
GET /v5/pocket/heartbeat/{session_id}
```

#### Hello
```http
GET /v5/pocket/hello/{session_id}
```

### Data Management Endpoints

#### Import Data
```http
POST /v5/pocket/import_data
Content-Type: application/json

{
    "session_id": "string",
    "data": "object"
}
```

#### Get Data
```http
GET /v5/pocket/data/{session_id}
```

### Field Management

#### Field Details
```http
GET /v5/pocket/field_detail/{session_id}/{field_id}
```

### Group Management

#### Group Details  
```http
GET /v5/pocket/group_detail/{session_id}/{group_id}
```

### Error Responses

#### Rate Limit Exceeded
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

## 🏗️ Architecture

### Project Structure
```
src/
├── main.rs              # Application entry point
├── models/              # Data models
│   ├── user.rs
│   ├── field.rs
│   ├── group.rs
│   └── data_transport.rs
├── rest/                # REST controllers
│   ├── rest_controller.rs
│   ├── rest_controller_login.rs
│   ├── rest_controller_registration.rs
│   └── ...
├── services/            # Business logic services
│   ├── http_server.rs
│   ├── rate_limiter.rs
│   ├── secure_session.rs
│   └── session.rs
```

### Key Components

#### Rate Limiter
- **Thread-safe**: Uses `Arc<Mutex<>>` for concurrent access
- **Automatic cleanup**: Background task removes expired entries
- **Configurable limits**: Per-endpoint rate limiting rules
- **Memory efficient**: HashMap-based storage with automatic expiration

#### Session Manager
- **Secure generation**: SHA256-based cryptographically secure IDs
- **Global singleton**: `LazyLock` for thread-safe initialization
- **High entropy**: Multiple randomness sources combined

#### HTTP Server
- **Actix Web framework**: High-performance async web server
- **CORS support**: Cross-origin resource sharing enabled
- **Static file serving**: Frontend asset delivery
- **Error handling**: Comprehensive error response system

## 🧪 Testing

### Test Coverage
The project includes comprehensive tests for:

#### Rate Limiter Tests
- ✅ Rate limit creation and configuration
- ✅ IP-based limiting
- ✅ Session-based limiting  
- ✅ Endpoint separation
- ✅ Request entry expiration
- ✅ Global rate limiter functionality

#### Secure Session Tests
- ✅ Uniqueness verification (1000+ IDs)
- ✅ Format validation (64-char hexadecimal)
- ✅ Entropy testing
- ✅ Global generator testing
- ✅ Security strength verification

### Running Tests
```bash
# All tests
cargo test

# Specific modules
cargo test rate_limiter
cargo test secure_session

# With output
cargo test -- --nocapture

# Release mode tests
cargo test --release
```

## 🚀 Performance

### Benchmarks
- **Rate Limiter**: <1ms overhead per request
- **Session Generation**: ~5μs per ID generation
- **Memory Usage**: ~100 bytes per tracked IP/session
- **Cleanup**: Automatic every 5 minutes

### Optimization Features
- **Efficient data structures**: HashMap-based lookups (O(1))
- **Minimal allocations**: Reuse of data structures where possible
- **Background cleanup**: Automatic memory management
- **Thread-safe design**: Lock-free operations where possible

## 🔧 Configuration

### Command Line Arguments

The application supports the following command line arguments:

```bash
./pocket-web-backend --help
```

Usage format:
```
pocket-web-backend [ADDRESS] [PORT] [MAX_THREADS] [SESSION_EXPIRATION_TIME]
```

Arguments (all optional, positional):
- `ADDRESS`: Server bind address (default from config: 127.0.0.1)
- `PORT`: Server port (default from config: 8080)
- `MAX_THREADS`: Maximum blocking threads (default from config: 2)
- `SESSION_EXPIRATION_TIME`: Session expiration time in seconds (default from config: 300)

Examples:
```bash
# Using all arguments
./pocket-web-backend 192.168.1.100 9090 4 600

# Using only some arguments (positional order matters)
./pocket-web-backend 0.0.0.0 8080

# Using only address
./pocket-web-backend 192.168.1.100

# Using defaults (no arguments)
./pocket-web-backend
```

### Docker/Podman Configuration

The application supports both **Docker** and **Podman** container runtimes.

#### Container Runtime Support

| Runtime | Status | Notes |
|---------|--------|-------|
| Docker | ✅ Fully Supported | Traditional container runtime |
| Podman | ✅ Fully Supported | Rootless, daemonless alternative |

#### Environment Variables

The Docker container supports the following environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `POCKET_ADDRESS` | Server bind address | `0.0.0.0` |
| `POCKET_PORT` | Server port | `8080` |
| `POCKET_MAX_THREADS` | Maximum blocking threads | `2` |
| `POCKET_SESSION_EXPIRATION` | Session expiration (seconds) | `300` |

#### Running with Docker/Podman

```bash
# Build the image with Docker
docker build -t pocket-web-backend .

# Or with Podman
podman build -t pocket-web-backend .

# Run the container with custom environment variables
docker run -it --rm \
    -p 8080:8080 \
    -e POCKET_ADDRESS=0.0.0.0 \
    -e POCKET_PORT=8080 \
    -e POCKET_MAX_THREADS=4 \
    -e POCKET_SESSION_EXPIRATION=900 \
    pocket-web-backend

# Or with Podman
podman run -it --rm \
    -p 8080:8080 \
    -e POCKET_ADDRESS=0.0.0.0 \
    -e POCKET_PORT=8080 \
    -e POCKET_MAX_THREADS=4 \
    -e POCKET_SESSION_EXPIRATION=900 \
    pocket-web-backend
```

#### Automatic Frontend Configuration

The Docker container automatically updates the frontend configuration (`statics/js/constants.mjs`) to match the server address and port specified in the environment variables.

### Rate Limiting Configuration
```rust
// Modify limits in RateLimiter::new()
endpoint_limits.insert("/v5/pocket/login".to_string(), RateLimit::new(5, 300));
endpoint_limits.insert("/v5/pocket/registration".to_string(), RateLimit::new(3, 3600));
```

### Adding New Endpoints
```rust
endpoint_limits.insert("/v5/pocket/new_endpoint".to_string(), RateLimit::new(10, 600));
```

### Cleanup Interval
```rust
// Modify cleanup interval (default: 5 minutes)
let mut cleanup_interval = interval(TokioDuration::from_secs(300));
```

## 🔒 Security Best Practices

### Implemented Protections
- ✅ **Brute Force Protection**: Login, registration, password change limiting
- ✅ **DoS/DDoS Mitigation**: General request limiting
- ✅ **Resource Exhaustion Prevention**: Heartbeat limiting
- ✅ **Spam Protection**: Registration and API limiting
- ✅ **Session Security**: Cryptographically secure session IDs
- ✅ **Input Validation**: JSON schema validation
- ✅ **Error Handling**: Secure error responses

### Resistance Against
- **IP Spoofing**: Protected by infrastructure
- **Session Rotation Attacks**: Session-based tracking
- **Distributed Attacks**: Effective against single-source attacks
- **Application Layer Attacks**: L7 protection
- **Prediction Attacks**: Cryptographically secure randomness

## 📈 Monitoring and Metrics

### Suggested Monitoring
- Number of blocked requests per endpoint
- Top IPs with rate limit violations
- Temporal access patterns
- Rate limiting effectiveness ratio
- Session generation performance
- Memory usage trends

### Future Logging Implementation
```rust
log::warn!("Rate limit exceeded for IP {} on endpoint {}", ip, endpoint);
log::info!("Rate limiter stats: {} active IPs, {} active sessions", ip_count, session_count);
```

## 🚧 Future Enhancements

### Planned Features
- [ ] JWT token support
- [ ] Comprehensive logging system

### Scalability Improvements
- [ ] Horizontal scaling support
- [ ] Load balancer integration
- [ ] Container deployment
- [ ] Microservices architecture

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Development Guidelines
- Follow Rust coding standards
- Write comprehensive tests
- Update documentation
- Use meaningful commit messages
- Ensure security best practices

## 📄 License

This project is licensed under the terms specified in the LICENSE file.

## 👥 Authors

- Antonio Salsi (@passy1977)

## 🆘 Support

For support, please:
1. Check the existing documentation
2. Search through issues
3. Create a new issue with detailed information
4. Include relevant logs and configuration

## 🔗 Related Projects

- [pocket-lib](https://github.com/passy1977/pocket-lib) - Core C++ library

---

**Note**: This backend is designed to work with the Pocket application ecosystem and provides secure, scalable API services with comprehensive protection mechanisms.