FROM rust:1-bookworm AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src/bin ./src/bin
RUN cargo build --release --bin upload_pipeline

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/upload_pipeline /usr/local/bin/upload_pipeline

ENV DATA_DIR=/data
ENTRYPOINT ["upload_pipeline"]