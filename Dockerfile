# Multi-stage Docker build for Pocket Backend with Spring Security
# Stage 1: Build the application
FROM debian:trixie AS build

USER root

# Install system dependencies
RUN DEBIAN_FRONTEND=noninteractive apt update && apt-get upgrade -y
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential gcc g++ cmake \
    pkg-config libssl-dev libsqlite3-dev libc6-dev \
    curl libcurl4-gnutls-dev git bash \
    clang libclang-dev llvm-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Setup working directory and user
WORKDIR /var/www
RUN useradd -m pocket
RUN chown pocket:pocket /var/www
USER pocket

# Copy application files with correct structure
COPY --chown=pocket:pocket ./bridge ./bridge
COPY --chown=pocket:pocket ./src ./src
COPY --chown=pocket:pocket ./statics ./statics
COPY --chown=pocket:pocket ./build.rs ./
COPY --chown=pocket:pocket ./Cargo.* ./
WORKDIR /var/www
# Install Rust and build the application
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/pocket/.cargo/bin:$PATH"
# Source the cargo environment and run rustup commands
RUN /bin/bash -c "source $HOME/.cargo/env && rustup default stable && rustup update"
RUN /bin/bash -c "source $HOME/.cargo/env && cargo build --release"
RUN cp /var/www/target/release/pocket-web-backend /var/www
RUN /bin/bash -c "source $HOME/.cargo/env && rustup self uninstall -y"


# Stage 2: Runtime image
FROM debian:trixie

# Install runtime dependencies
RUN DEBIAN_FRONTEND=noninteractive apt update && \
    apt-get install -y \
    curl \
    libcurl4-gnutls-dev \
    libssl3 \
    libsqlite3-0 \
    libc6 \
    libstdc++6 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Create user and setup directories
RUN useradd -m -s /bin/bash pocket
RUN mkdir -p /var/www/scripts /var/log/pocket
RUN chown -R pocket:pocket /var/www /var/log/pocket

# Copy application files from build stage
COPY --from=build --chown=pocket:pocket /var/www/target/release/pocket-web-backend /var/www/pocket-web-backend
COPY --from=build --chown=pocket:pocket /var/www/statics /var/www/statics

# Make CLI tools executable
RUN chmod +x /var/www/pocket-web-backend
RUN echo ciao

# Create startup script that updates constants.mjs and starts the application
RUN echo '#!/bin/bash' > /var/www/start.sh && \
    echo 'set -e' >> /var/www/start.sh && \
    echo '' >> /var/www/start.sh && \
    echo '# Build full address from POCKET_HOST and POCKET_PORT' >> /var/www/start.sh && \
    echo 'FULL_ADDRESS="http://${POCKET_HOST}:${POCKET_PORT}"' >> /var/www/start.sh && \
    echo 'echo "Server will run at: ${FULL_ADDRESS}"' >> /var/www/start.sh && \
    echo '' >> /var/www/start.sh && \
    echo '# Use BACKEND_URL if set, otherwise use FULL_ADDRESS' >> /var/www/start.sh && \
    echo 'EFFECTIVE_BACKEND_URL="${BACKEND_URL:-$FULL_ADDRESS}"' >> /var/www/start.sh && \
    echo 'echo "Frontend will connect to: ${EFFECTIVE_BACKEND_URL}"' >> /var/www/start.sh && \
    echo '' >> /var/www/start.sh && \
    echo '# Update BACKEND_URL in constants.mjs' >> /var/www/start.sh && \
    echo 'sed -i "s|const BACKEND_URL = '\''[^'\'']*'\'';|const BACKEND_URL = '\''${EFFECTIVE_BACKEND_URL}'\'';|g" /var/www/statics/js/constants.mjs' >> /var/www/start.sh && \
    echo '' >> /var/www/start.sh && \
    echo '# Start the application' >> /var/www/start.sh && \
    echo 'exec /var/www/pocket-web-backend \' >> /var/www/start.sh && \
    echo '    ${FULL_ADDRESS} \' >> /var/www/start.sh && \
    echo '    ${POCKET_MAX_THREADS} \' >> /var/www/start.sh && \
    echo '    ${POCKET_SESSION_EXPIRATION}' >> /var/www/start.sh && \
    chmod +x /var/www/start.sh

# Switch to non-root user
USER pocket

# Set working directory
WORKDIR /var/www

# Set default environment variables
ENV POCKET_HOST=0.0.0.0
ENV POCKET_PORT=8080
ENV POCKET_MAX_THREADS=2
ENV POCKET_SESSION_EXPIRATION=300

# Expose application port (can be overridden)
EXPOSE ${POCKET_PORT}

# Health check
# Note: HEALTHCHECK is not supported in OCI format (Podman default)
# Uncomment the following line if using Docker format
# HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
#     CMD curl -f http://localhost:${POCKET_PORT}/v5/pocket/heartbeat || exit 1

# Start application using the startup script
CMD ["/var/www/start.sh"] 



