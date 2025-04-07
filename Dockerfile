FROM rust:1.85-slim as builder

WORKDIR /app


RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    make \
    libssl-dev \
    protobuf-compiler \
    build-essential \
    autoconf \
    libtool \
    && rustup component add rustfmt


COPY Cargo.toml Cargo.lock .env ./
COPY apps apps/
COPY packages packages/


RUN cargo build --release

FROM debian:bookworm-slim as ingester

RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/.env .
COPY --from=builder /app/target/release/ingester .
COPY apps/ingester/config ./apps/ingester/config

ENV RUST_LOG=info
ARG CLICKHOUSE_PASSWORD
ENV CLICKHOUSE_USER=default
ENV CLICKHOUSE_PASSWORD=$CLICKHOUSE_PASSWORD

CMD ["./ingester"]

FROM debian:bookworm-slim as generator

RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/.env .
COPY --from=builder /app/target/release/generator .
COPY apps/generator/config ./apps/generator/config

ENV RUST_LOG=info

CMD ["./generator"]