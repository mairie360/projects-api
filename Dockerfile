# Define build arguments
ARG RUST_VERSION=1.85.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/src/projects-api

# Install dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends \
    binutils libpq-dev curl pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Fetch dependencies
RUN cargo fetch --locked

# Build the application
RUN cargo build --release --locked && \
    strip target/release/projects-api

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates wget libpq5 curl libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group
RUN groupadd --system projects-api && useradd --no-log-init --system -g projects-api projects-api

# Copy the compiled binary
COPY --from=builder --chown=projects-api:projects-api /usr/src/projects-api/target/release/projects-api /usr/local/bin/projects-api

# Set permissions
USER projects-api

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/projects-api"]
CMD []
