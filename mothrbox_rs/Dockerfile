# ========= 1) Build stage =========
FROM rust:slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ========= 2) Runtime stage =========
FROM rust:slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/mothrbox_rs /usr/local/bin/mothrbox

ENTRYPOINT ["mothrbox"]
