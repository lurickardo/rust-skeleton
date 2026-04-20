# <p align="center">Rust Skeleton</p>

<p align="center">Generic software architecture framework in <a href="https://www.rust-lang.org" target="_blank">Rust</a> using Axum and Tokio. Rust port of <a href="https://github.com/lurickardo/skeleton">lurickardo/skeleton</a>.</p>

<p align="center">
  <a><img src="https://img.shields.io/badge/license-MIT-green" alt="Package License" /></a>
  <a href="https://www.rust-lang.org" target="_blank"><img src="https://img.shields.io/badge/rust-edition%202021-orange?logo=rust" alt="Rust Edition" /></a>
  <a href="https://github.com/tokio-rs/axum" target="_blank"><img src="https://img.shields.io/badge/axum-v0.7-green" alt="Axum Version" /></a>
  <a href="https://tokio.rs" target="_blank"><img src="https://img.shields.io/badge/tokio-v1-green" alt="Tokio Version" /></a>
  <a href="https://github.com/juhaku/utoipa" target="_blank"><img src="https://img.shields.io/badge/utoipa-v4-green" alt="utoipa Version" /></a>
  <a href="https://github.com/Keats/validator" target="_blank"><img src="https://img.shields.io/badge/validator-v0.18-green" alt="Validator Version" /></a>
</p>

## Description

Rust Skeleton is a generic software architecture framework in Rust using Axum, designed to provide a flexible and reusable foundation for application development. It is a direct port of [lurickardo/skeleton](https://github.com/lurickardo/skeleton) (Node.js + Fastify) into idiomatic Rust.

## Philosophy

The idea of the system is to be a base architecture for you to develop your own small or large-scale services, without the need to recreate existing solutions. At the same time, not forcing you to use a complete and heavy structure. Each module was designed to be a "Lego piece" that you can take only what is necessary for your version of the system and extend it the way you prefer.

## Libraries

- [Rust](https://www.rust-lang.org) (edition 2021)
- [Tokio](https://tokio.rs) — async runtime (multi-thread by default, replaces Node's `cluster`)
- [Axum](https://github.com/tokio-rs/axum) — web framework (replaces Fastify)
- [tower-http](https://docs.rs/tower-http) — CORS & tracing
- [serde](https://serde.rs) + [validator](https://docs.rs/validator) — DTO validation (replaces Zod/AJV)
- [utoipa](https://docs.rs/utoipa) + [utoipa-swagger-ui](https://docs.rs/utoipa-swagger-ui) — OpenAPI + Swagger UI
- [thiserror](https://docs.rs/thiserror) — error enums
- [tracing](https://docs.rs/tracing) — structured logs
- [dotenvy](https://docs.rs/dotenvy) — `.env` loader
- Built-in `cargo test` + [mockall](https://docs.rs/mockall) (replaces Jest)

## Dependencies

- Rust toolchain (`rustup` recommended)

## Installation

```bash
cargo build
```

## Running the app

```bash
$ cargo run

# OR development (hot reload with cargo-watch, optional)
$ cargo watch -x run

# production
$ cargo build --release
$ ./target/release/rust-skeleton
```

Set the environment variables (see `.env.example`) before running.

## Testing

```bash
# run unit + integration tests
$ cargo test

# with verbose output
$ cargo test -- --nocapture

# lint / format
$ cargo clippy --all-targets -- -D warnings
$ cargo fmt
```

## Endpoints

Default prefix: `/api/rustskeleton`

| Method | Path                                 | Description            |
|--------|--------------------------------------|------------------------|
| GET    | `/api/rustskeleton/healthcheck`      | Liveness probe         |
| GET    | `/api/rustskeleton/v1/user`          | List all users         |
| GET    | `/api/rustskeleton/v1/user/:id`      | Find user by id        |
| POST   | `/api/rustskeleton/v1/user`          | Create user            |
| PUT    | `/api/rustskeleton/v1/user/:id`      | Update user            |
| DELETE | `/api/rustskeleton/v1/user/:id`      | Remove user            |
| GET    | `/api/rustskeleton/docs`             | Swagger UI (dev only)  |

## Swagger

`http://localhost:3000/api/rustskeleton/docs`

## License

MIT
