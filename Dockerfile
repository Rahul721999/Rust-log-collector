# Prepare tool
FROM rust:latest AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Caching dependencies
FROM chef AS planner
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

# Build stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code and build the final binary
COPY . .
RUN cargo build --release --bin Rust-log-collector

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install required runtime dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
    openssl \
    libssl-dev \
    ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/Rust-log-collector /app/binary

# Set default command
CMD [ "/app/binary" ]
