# Multi-stage Docker build for Pocket Backend with Spring Security
# Stage 1: Build the application
FROM debian:trixie as build

USER root

# Install system dependencies
RUN DEBIAN_FRONTEND=noninteractive apt update && apt-get upgrade -y
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y \
    pkg-config libssl-dev libsqlite3-dev libc6-dev \
    curl git bash \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Setup working directory and user
WORKDIR /var/www
RUN useradd -m pocket
RUN chown pocket:pocket /var/www
USER pocket

# Copy application files
RUN mkdir -p /var/www
COPY --chown=pocket:pocket ./bridge /var/www
COPY --chown=pocket:pocket ./src /var/www
COPY --chown=pocket:pocket ./statics /var/www
COPY --chown=pocket:pocket ./build.rs /var/www
COPY --chown=pocket:pocket ./Cargo.* /var/www
WORKDIR /var/www
# Install Rust and build the application
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/pocket/.cargo/bin:$PATH"
RUN rustup default stable
RUN rustup update
RUN cargo build --release
RUN cp /var/www/target/release/pocket-web-backend /var/www
RUN rustup self uninstall -y


# Stage 2: Runtime image
FROM debian:trixie

# Create user and setup directories
RUN adduser -D -s /bin/bash pocket
RUN mkdir -p /var/www/scripts /var/log/pocket
RUN chown -R pocket:pocket /var/www /var/log/pocket

# Copy application files from build stage
COPY --from=build --chown=pocket:pocket /var/www/target/release/pocket-web-backend /var/www/pocket-web-backend
COPY --from=build --chown=pocket:pocket /var/www/statics /var/www/statics

# Make CLI tools executable
RUN chmod +x /var/www/pocket-web-backend /var/www/pocket-web-backend

# Switch to non-root user
USER pocket

# Set working directory
WORKDIR /var/www

# Set default environment variables
ENV POCKET_ADDRESS=0.0.0.0
ENV POCKET_PORT=8080
ENV POCKET_MAX_THREADS=2
ENV POCKET_SESSION_EXPIRATION=300

# Expose application port (can be overridden)
EXPOSE ${POCKET_PORT}

# Health check (using environment variable for port)
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:${POCKET_PORT} || exit 1

# Create startup script that updates constants.mjs and starts the application
RUN echo '#!/bin/bash' > /var/www/start.sh && \
    echo 'set -e' >> /var/www/start.sh && \
    echo '' >> /var/www/start.sh && \
    echo '# Update BACKEND_URL in constants.mjs with current ADDRESS and PORT' >> /var/www/start.sh && \
    echo 'BACKEND_URL="http://${POCKET_ADDRESS}:${POCKET_PORT}"' >> /var/www/start.sh && \
    echo 'echo "Updating constants.mjs with BACKEND_URL: $BACKEND_URL"' >> /var/www/start.sh && \
    echo 'sed -i "s|const BACKEND_URL = '\''[^'\'']*'\'';|const BACKEND_URL = '\''$BACKEND_URL'\'';|g" /var/www/statics/js/constants.mjs' >> /var/www/start.sh && \
    echo '' >> /var/www/start.sh && \
    echo '# Start the application' >> /var/www/start.sh && \
    echo 'exec /var/www/pocket-web-backend \' >> /var/www/start.sh && \
    echo '    --address ${POCKET_ADDRESS} \' >> /var/www/start.sh && \
    echo '    --port ${POCKET_PORT} \' >> /var/www/start.sh && \
    echo '    --max-threads ${POCKET_MAX_THREADS} \' >> /var/www/start.sh && \
    echo '    --session-expiration-time ${POCKET_SESSION_EXPIRATION}' >> /var/www/start.sh && \
    chmod +x /var/www/start.sh

# Start application using the startup script
CMD ["/var/www/start.sh"] 



