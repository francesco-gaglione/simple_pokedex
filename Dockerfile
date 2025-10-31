# ---- Backend build stage ----
FROM rust:1.86-bullseye AS backend-builder

WORKDIR /app

# Cache dependencies by creating a dummy project
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf target/release/.fingerprint/simple_pokedex-* \
    && rm -rf target/release/deps/simple_pokedex-* \
    && rm -rf target/release/simple_pokedex*

# Copy actual source code
COPY src ./src

# Build the actual binary
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bullseye-slim AS runtime

# Install runtime dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates

WORKDIR /app

# Copy the compiled binary from builder
COPY --from=backend-builder /app/target/release/simple_pokedex /usr/local/bin/simple_pokedex

# Set environment variables
ENV RUST_LOG=info \
    SERVER_PORT=8080 \
    ENABLE_SWAGGER=false

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD [ "wget", "--quiet", "--tries=1", "--spider", "http://localhost:8080/health" ] || exit 1

# Run the application
CMD ["/usr/local/bin/simple_pokedex"]
