# CLAUDE.md

Guia para sessões do Claude Code (ou qualquer desenvolvedor) neste projeto.

## O que é este projeto

`rust-skeleton` é o port em Rust (Axum + Tokio) do framework [lurickardo/skeleton](https://github.com/lurickardo/skeleton) (Node.js + Fastify). Serve como base reutilizável para serviços HTTP, preservando a mesma organização modular e semântica do original.

## Comandos essenciais

```bash
cargo run                                  # subir servidor (usa .env)
cargo test                                 # rodar todos os testes unitários
cargo test -- --nocapture                  # ver stdout dos logs durante os testes
cargo clippy --all-targets -- -D warnings  # lint estrito
cargo fmt                                  # formatar
cargo build --release                      # build produção
```

## Layout dos módulos

```
src/
├── main.rs                    bootstrap (tracing, env, listener)
├── lib.rs                     re-exporta módulos
├── app.rs                     build_router(env) → agrega rotas + plugins
├── config/
│   ├── env.rs                 Env + load_env() (dotenvy)
│   ├── error.rs               AppError + IntoResponse (substitui errorHandler)
│   └── logger.rs              log_method helper (substitui @Log)
├── plugins/
│   ├── cors.rs                CorsLayer
│   ├── healthcheck.rs         GET /healthcheck
│   ├── validation.rs          ValidatedJson<T> extractor (serde + validator)
│   └── swagger.rs             utoipa OpenApi doc + Swagger UI
└── v1/modules/user/
    ├── dto.rs                 CreateUserDto, UpdateUserDto + transforms
    ├── schema.rs              UserResponse, DeleteMessage (ToSchema)
    ├── service.rs             UserServiceTrait + UserService impl (+ mock)
    ├── middleware.rs          find_by_id middleware
    ├── controller.rs          handlers axum com #[utoipa::path]
    └── routes.rs              monta Router do módulo
```

## Convenções

- **Imports**: `cargo fmt` organiza automaticamente.
- **Testes**: ficam inline em `#[cfg(test)] mod tests` no mesmo arquivo do código. Teste de integração com axum usa `tower::ServiceExt::oneshot`.
- **Mocks**: tudo que precise ser mockado é um `trait` com `#[cfg_attr(test, mockall::automock)]`.
- **Erros**: sempre retornar `Result<T, AppError>`. Nunca usar `panic!` em caminho quente.
- **Variáveis de ambiente**: quaisquer testes que mexem em `std::env::set_var` precisam do `ENV_LOCK` (Mutex estático) em `src/config/env.rs`.
- **Commits**: autor único — nunca adicionar `Co-Authored-By`.

## Como adicionar um novo módulo (ex: `product`)

1. Criar `src/v1/modules/product/` com: `mod.rs`, `dto.rs`, `schema.rs`, `service.rs`, `middleware.rs`, `controller.rs`, `routes.rs`.
2. Em cada controller handler, anotar `#[utoipa::path(...)]` e registrar em `src/plugins/swagger.rs` (struct `ApiDoc`).
3. Em `src/v1/modules/mod.rs` declare `pub mod product;`.
4. Em `src/app.rs::build_router`, instancie o service e faça `Router::merge` no `api_router` ou `nest` adicional.
5. Escrever testes inline (service puros + controller via `ServiceExt::oneshot`).
6. Rodar `cargo test && cargo clippy --all-targets -- -D warnings` antes de commitar.

## Variáveis de ambiente

| Nome               | Default  | Descrição                                     |
|--------------------|----------|-----------------------------------------------|
| `PORT`             | 3000     | porta HTTP                                    |
| `APP_ENVIRONMENT`  | DEV      | `DEV` expõe Swagger; `PRD` desabilita         |
| `USE_ROUTE_PREFIX` | false    | `true` faz Swagger usar `/api/<app>/` base    |
| `DB_NAME`          | (vazio)  | placeholder para futura integração DB         |
| `DB_URL`           | (vazio)  | placeholder para futura integração DB         |

## Equivalências Node → Rust (para quem veio do skeleton original)

| Skeleton (Node/TS)        | rust-skeleton                                 |
|---------------------------|-----------------------------------------------|
| Fastify                   | axum                                          |
| `cluster.fork()`          | `#[tokio::main]` (multi-thread default)       |
| Zod DTO                   | `serde + validator`                           |
| AJV schemaCompiler        | `ValidatedJson<T>` extractor                  |
| `@Log()` decorator        | `log_method(...)` helper                      |
| errorHandler (Fastify)    | `AppError: IntoResponse`                      |
| `@fastify/cors`           | `tower_http::cors::CorsLayer`                 |
| `@fastify/under-pressure` | handler `/healthcheck` simples                |
| `@fastify/swagger`        | `utoipa + utoipa-swagger-ui`                  |
| Jest                      | `cargo test` + `mockall` + `tower::ServiceExt`|
| Biome                     | `rustfmt + clippy`                            |
