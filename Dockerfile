FROM rust:1-slim AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/rust-skeleton /app/rust-skeleton

EXPOSE 3000

CMD ["/app/rust-skeleton"]
