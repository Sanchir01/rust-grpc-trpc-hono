FROM rust:1.85 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    bash \
    git \
    make \
    gcc \
    gettext \
    musl-dev \
    libssl-dev ca-certificates

COPY . .

RUN cargo build --release --workspace

CMD ["make", "run-prod"]