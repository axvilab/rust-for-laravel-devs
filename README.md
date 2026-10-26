# Rust for Laravel Developers — companion code

Code for the book **"Rust for Laravel Developers"** (Ukrainian edition: **"Rust для Laravel-розробника"**) by Vitaly Ax.

Every code sample from the book is here, and all of it compiles and passes its tests. If the text and the repository ever disagree, trust the repository: CI checks it on every commit.

The code is MIT-licensed (see [LICENSE](LICENSE)).

## What's inside

```
Cargo.toml            Cargo workspace: orders-api and one crate per chapter
chapters/NN-*/        a chapter crate: examples/ (runnable), broken/ (intentionally does not compile)
orders-api/           the end-to-end project: an orders microservice (chapters 16b–16f)
scripts/              check-broken.sh — verifies broken/; gen-chapter-readmes.py — chapter READMEs
.github/workflows/    CI: fmt, clippy -D warnings, tests with Postgres, broken/, release build, audit, docker
```

### Chapter examples

Each chapter is a separate crate `chNN-name`, and each sample from the text is a separate program:

```bash
cargo run -p ch01-basics --example 01-hello
cargo run -p ch08-ownership --example 11-let-underscore
cargo run -p ch05-enums --example exercise-4      # solution to "Try it" exercise 4

# the same without Rust installed
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch01-basics --example 01-hello
```

Code that the book shows as a compile error lives in `broken/`, which Cargo does not build. The header of each such file states the expected compiler error and points to the fixed version in `examples/`. `./scripts/check-broken.sh` verifies that every file really fails, and with exactly the error its header promises. Each chapter's README has a table of its examples.

### `orders-api`

An orders microservice on Axum 0.8 + sqlx + Postgres:

- **domain**: `Order`, `OrderItem`, an `OrderStatus` state machine with a private `status`, `DomainError` via `thiserror`, an `Email` newtype with `TryFrom` and `#[serde(try_from)]`. The module depends on neither the database nor HTTP, and has unit tests;
- **errors**: one `AppError` with `#[from]` and `IntoResponse`, so handlers only need `?`;
- **repository**: `sqlx::query_as`, transactions via `&mut *tx`, order items loaded with a single `ANY($1)` query (no N+1); filters and sorting via `QueryBuilder::push_bind`;
- **HTTP**: `TraceLayer` with the request id in the span, a custom `x-request-id` middleware, timeout, CORS, compression, a JSON 404 fallback;
- **authentication**: a JWT extractor `AuthUser` (`FromRequestParts`), `Option<AuthUser>`, `AdminUser`, argon2 in `spawn_blocking`;
- **validation**: `validator` plus a custom `ValidatedJson<T>` extractor (422 with a list of fields), pagination `Page<T>` with `meta`;
- **operations**: config via `envy`/`dotenvy`, `tracing` (pretty or JSON), graceful shutdown, `/health` and `/ready`, migrations embedded in the binary;
- **deployment**: a multi-stage `Dockerfile` (cargo-chef for dependency caching → `distroless/cc`), `docker-compose.yml` with Postgres.

## Running with Docker (no Rust on the host)

Run everything from `orders-api/`:

```bash
cd orders-api

# API + Postgres → http://localhost:3000
docker compose up --build

# tests inside rust:1-bookworm, including the Postgres integration tests
docker compose run --rm dev
docker compose run --rm dev cargo clippy --all-targets -- -D warnings
```

The registry and `target/` caches live in named volumes, so the second run is fast.

A one-off command without compose (from the repository root):

```bash
docker run --rm -v "$PWD":/work -w /work \
  -v cargo-registry:/usr/local/cargo/registry \
  rust:1-bookworm cargo test --lib     # unit tests, no database needed
```

## Running with a local Rust toolchain

```bash
cd orders-api
cp .env.example .env
docker compose up -d db          # Postgres only
cargo run                        # migrations run at startup
cargo test                       # unit and integration tests (#[sqlx::test] reads DATABASE_URL from .env)
cargo test --lib                 # unit tests only, no database needed
```

Try it by hand:

```bash
curl -s -XPOST localhost:3000/api/v1/auth/register -H 'content-type: application/json' \
  -d '{"email":"v@example.com","password":"password123"}'
TOKEN=$(curl -s -XPOST localhost:3000/api/v1/auth/login -H 'content-type: application/json' \
  -d '{"email":"v@example.com","password":"password123"}' | jq -r .token)

curl -s -XPOST localhost:3000/api/v1/orders -H "authorization: Bearer $TOKEN" \
  -H 'content-type: application/json' \
  -d '{"customer_email":"v@example.com","items":[{"sku":"SKU-1","qty":2,"unit_price":"100"}]}'
# → 201 {"id":"…","status":"draft","total":"200.00",…}

curl -s -XPOST localhost:3000/api/v1/orders/<id>/pay -H "authorization: Bearer $TOKEN"
curl -s "localhost:3000/api/v1/orders?status=paid&sort=total_desc&per_page=10" -H "authorization: Bearer $TOKEN"
```

### Routes

| Method and path | Access | What it does |
|---|---|---|
| `GET /health` | everyone | the process is alive |
| `GET /ready` | everyone | the database is reachable |
| `POST /api/v1/auth/register` | everyone | sign up: 201, or 409 if the email is taken |
| `POST /api/v1/auth/login` | everyone | a JWT |
| `GET /api/v1/me` | `Option<AuthUser>` | a guest or the current user |
| `POST /api/v1/orders` | `AuthUser` | create an order; items can be passed right away |
| `GET /api/v1/orders` | `AuthUser` | your orders: `page`, `per_page`, `status`, `customer_email`, `created_after`, `sort` |
| `GET /api/v1/orders/{id}` | `AuthUser` | one order; someone else's returns 404 |
| `POST /api/v1/orders/{id}/items` | `AuthUser` | add an item |
| `POST /api/v1/orders/{id}/pay` | `AuthUser` | pay |
| `POST /api/v1/orders/{id}/cancel` | `AuthUser` | cancel |
| `GET /api/v1/admin/orders` | `AdminUser` | all orders |

An admin is made directly in the database: `UPDATE users SET role = 'admin' WHERE email = '…'`. Log in again afterwards, because the role is stored in the token.

### `query_as` or `query_as!`

The repository uses `sqlx::query_as::<_, T>(…)`: SQL is checked at runtime, so the project builds without a database. In production people usually use the `query!`/`query_as!` macros, which check SQL against a live database during `cargo build`. How to switch is described in chapter 16d:

```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
cargo sqlx prepare            # creates .sqlx/ — commit it to the repository
SQLX_OFFLINE=true cargo build # CI and Docker build without a database
```

After switching, add `ENV SQLX_OFFLINE=true` before `cargo build` in the `Dockerfile`, and make sure `.sqlx/` is part of the build context.

## Chapters → directories

| Book chapter | File | Code |
|---|---|---|
| 1. Hello, world and basic syntax | `01-basics` | [`chapters/01-basics`](chapters/01-basics) |
| 2. Control flow | `02-control` | [`chapters/02-control`](chapters/02-control) |
| 3. Functions and closures | `03-functions` | [`chapters/03-functions`](chapters/03-functions) |
| 4. Structs — classes without classes | `04-structs` | [`chapters/04-structs`](chapters/04-structs) |
| 5. Enum, Option and saying goodbye to null | `05-enums` | [`chapters/05-enums`](chapters/05-enums) |
| 6. How the language is built: the mental model | `06-concept` | [`chapters/06-concept`](chapters/06-concept) |
| 7. Traits — interfaces that can do more | `07-traits` | [`chapters/07-traits`](chapters/07-traits) |
| 8. Ownership — one owner for every thing | `08-ownership` | [`chapters/08-ownership`](chapters/08-ownership) |
| 9. Borrowing — lend it without giving it away | `09-borrowing` | [`chapters/09-borrowing`](chapters/09-borrowing) |
| 10. Lifetimes — briefly and without panic | `10-lifetimes` | [`chapters/10-lifetimes`](chapters/10-lifetimes) |
| 11. Collections and iterators — instead of PHP arrays | `11-collections` | [`chapters/11-collections`](chapters/11-collections) |
| 12. Error handling — Result instead of exceptions | `12-errors` | [`chapters/12-errors`](chapters/12-errors) |
| 13. Generics and smart pointers | `13-generics` | [`chapters/13-generics`](chapters/13-generics) |
| 14. Modules, crates and Cargo — how to organize a project | `14-modules` | [`chapters/14-modules`](chapters/14-modules) |
| 15. Threads — the parallelism PHP never had | `15-threads` | [`chapters/15-threads`](chapters/15-threads) |
| 16. Async — what web servers run on | `16-async` | [`chapters/16-async`](chapters/16-async) |
| Project: an orders service from `struct` to HTTP API | `16b-project` | [`orders-api/src/domain.rs`](orders-api/src/domain.rs), [`error.rs`](orders-api/src/error.rs), [`repo.rs`](orders-api/src/repo.rs), [`handlers/orders.rs`](orders-api/src/handlers/orders.rs), [`main.rs`](orders-api/src/main.rs) |
| Middleware, authentication and application state | `16c-middleware` | [`middleware.rs`](orders-api/src/middleware.rs), [`auth.rs`](orders-api/src/auth.rs), [`state.rs`](orders-api/src/state.rs), [`handlers/auth.rs`](orders-api/src/handlers/auth.rs), [`lib.rs`](orders-api/src/lib.rs) |
| Database, deeper: sqlx without Eloquent | `16d-database` | [`repo.rs`](orders-api/src/repo.rs), [`migrations/`](orders-api/migrations) |
| API practices: validation, pagination, versioning, documentation | `16e-api` | [`validation.rs`](orders-api/src/validation.rs), [`pagination.rs`](orders-api/src/pagination.rs), [`filter.rs`](orders-api/src/filter.rs), [`error.rs`](orders-api/src/error.rs) |
| Deployment: a single binary instead of Forge | `16f-deploy` | [`config.rs`](orders-api/src/config.rs), [`Dockerfile`](orders-api/Dockerfile), [`docker-compose.yml`](orders-api/docker-compose.yml), [`ci.yml`](.github/workflows/ci.yml) |
| 17. Tests, macros, and what's next | `17-tests` | [`chapters/17-tests`](chapters/17-tests), [`orders-api/tests/`](orders-api/tests) |

## Versions

Rust 1.97, edition 2024. Minimum Rust version: 1.88 for the chapters (let chains in chapter 2), 1.94 for `orders-api` (because of sqlx 0.9). Chapters 12, 16 and 17 use the same versions of tokio, axum, serde, thiserror and anyhow (plus toml 0.9) as orders-api. orders-api dependencies: axum 0.8.9, tokio 1.54, sqlx 0.9.0, tower-http 0.7.1, validator 0.21, jsonwebtoken 11.1 (`aws_lc_rs` backend), argon2 0.6, thiserror 2, serde 1, tracing-subscriber 0.3. Exact versions are pinned in `Cargo.lock`.

## Українською

Це код до книги **«Rust для Laravel-розробника»** (Vitaly Ax): усі приклади з тексту, розв'язки вправ «Спробуй» і наскрізний проєкт `orders-api` (сервіс замовлень на Axum + sqlx + Postgres). Усе компілюється й проходить тести в CI. Коментарі в коді англійською, бо репозиторій спільний для обох видань книги. Ліцензія — MIT.

Запуск через Docker, без Rust на машині (з кореня репозиторію):

```bash
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch01-basics --example 01-hello

cd orders-api
docker compose up --build        # API + Postgres → http://localhost:3000
docker compose run --rm dev      # тести всередині rust:1-bookworm
```

У `chapters/NN-*/broken/` лежить код, який книга показує як помилку компіляції. Перевірка: `./scripts/check-broken.sh`.

| Розділ книги | Файл | Код |
|---|---|---|
| 1–17 | `NN-назва` | `chapters/NN-назва` (таблиця вище) |
| Проєкт, 16b–16f | `16b-project` … `16f-deploy` | `orders-api/` |
