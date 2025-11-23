# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src
COPY proto ./proto

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (curl for Walrus HTTP calls)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 cryptool

# Create working directory for encrypted files
WORKDIR /data
RUN chown cryptool:cryptool /data

# Copy binary from builder
COPY --from=builder /app/target/release/mothrbox_rs /usr/local/bin/cryptool

# Switch to non-root user
USER cryptool

# Set entrypoint
ENTRYPOINT ["cryptool"]
CMD ["help"]