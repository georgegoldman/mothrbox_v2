# ============================
# 1. Build Rust binary
# ============================
FROM rust:1.83 as rust-builder

WORKDIR /app

# Install nightly toolchain (needed for edition2024 crates)
RUN rustup toolchain install nightly && rustup default nightly

# Copy Rust project files
COPY mothrbox_rs/Cargo.toml mothrbox_rs/Cargo.lock ./mothrbox_rs/
COPY mothrbox_rs/src ./mothrbox_rs/src

WORKDIR /app/mothrbox_rs

# Build release binary
RUN cargo build --release

# ============================
# 2. Final image: Rust CLI + Deno Walrus CLI
# ============================
FROM debian:bookworm-slim

# Install runtime deps and tools for Deno
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Install Deno
RUN curl -fsSL https://deno.land/install.sh | sh

# Add Deno to PATH
ENV DENO_INSTALL="/root/.deno"
ENV PATH="${DENO_INSTALL}/bin:${PATH}"

WORKDIR /app

# Copy Rust binary from builder
# Adjust binary name if your package is named differently
COPY --from=rust-builder /app/mothrbox_rs/target/release/mothrbox_rs /usr/local/bin/mothrbox_rs

# Copy Deno Walrus project
COPY mothrbox_ts/ ./mothrbox_ts/

# Pre-cache Deno dependencies
RUN deno cache mothrbox_ts/src/walrus-cli.ts

# Folder for data (bind-mount from host)
RUN mkdir -p /app/data

# Entry script - exposes only the Rust CLI
COPY mothrbox_entrypoint.sh /usr/local/bin/mothrbox_entrypoint.sh
RUN chmod +x /usr/local/bin/mothrbox_entrypoint.sh

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/mothrbox_entrypoint.sh"]
CMD ["--help"]
