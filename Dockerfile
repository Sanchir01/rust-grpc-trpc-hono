FROM rust:1.85-slim as builder

WORKDIR /app


RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    build-essential \
    autoconf \
    libtool \
    && rustup component add rustfmt


COPY Cargo.toml Cargo.lock ./
COPY apps apps/
COPY packages packages/


RUN cargo build --release

FROM debian:bullseye-slim as ingester

RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app


COPY --from=builder /app/target/release/ingester .
COPY apps/ingester/config/prod.toml ./config/prod.toml

ENV RUST_LOG=info

CMD ["./ingester"]

FROM debian:bullseye-slim as generator

RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/generator .
COPY apps/generator/config/prod.toml ./config/prod.toml

ENV RUST_LOG=info

# Запускаем сервис
CMD ["./generator"]