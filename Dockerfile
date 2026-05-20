# Stage 1: Build the binary
FROM rust:1.75-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/monatm
COPY . .

RUN cargo build --release

# Stage 2: Minimal runtime environment
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/monatm/target/release/monatm .
COPY --from=builder /usr/src/monatm/Config.toml .

ENTRYPOINT ["./monatm"]