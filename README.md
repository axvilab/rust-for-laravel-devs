# Rust для Laravel-розробника — код до книги

Companion code for the book **"Rust для Laravel-розробника / Rust for Laravel Developers"** by Vitaly Ax.

Тут лежить код з книги, який компілюється і проходить тести. Якщо код у тексті й тут розходяться, правильна версія та, що в репозиторії: її перевіряє CI на кожен коміт.

Код поширюється за ліцензією MIT (див. [LICENSE](LICENSE)).

## Що всередині

```
Cargo.toml            Cargo workspace: orders-api і крейти розділів
chapters/NN-*/        крейт розділу: examples/ (запускаються), broken/ (навмисно не компілюються)
orders-api/           наскрізний проєкт: сервіс замовлень (розділи 16b–16f)
scripts/              check-broken.sh — перевірка broken/; gen-chapter-readmes.py — README розділів
.github/workflows/    CI: fmt, clippy -D warnings, тести з Postgres, broken/, release-збірка, audit, docker
```

### Приклади до розділів

Кожен розділ — окремий крейт `chNN-назва`, а кожен приклад з тексту — окрема програма:

```bash
cargo run -p ch01-basics --example 01-hello
cargo run -p ch08-ownership --example 11-let-underscore
cargo run -p ch05-enums --example exercise-4      # розв'язок вправи «Спробуй» №4

# те саме без Rust на машині
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch01-basics --example 01-hello
```

Код, який у книзі наведено як помилку компіляції, лежить у `broken/` (Cargo його не збирає). У шапці кожного такого файлу записано очікувану помилку компілятора і посилання на виправлену версію в `examples/`. Скрипт `./scripts/check-broken.sh` перевіряє, що кожен файл справді не компілюється і що помилка саме та, яку обіцяє шапка. Таблиця прикладів є в README кожного розділу.

### `orders-api`

Мікросервіс замовлень на Axum 0.8 + sqlx + Postgres:

- **домен**: `Order`, `OrderItem`, стейт-машина `OrderStatus` з приватним `status`, `DomainError` через `thiserror`, newtype `Email` з `TryFrom` і `#[serde(try_from)]`. Модуль не залежить ні від БД, ні від HTTP, і має юніт-тести;
- **помилки**: один `AppError` з `#[from]` та `IntoResponse`, тож у хендлерах залишається тільки `?`;
- **репозиторій**: `sqlx::query_as`, транзакції через `&mut *tx`, позиції замовлень одним запитом `ANY($1)`, без N+1; фільтри та сортування через `QueryBuilder::push_bind`;
- **HTTP**: `TraceLayer` з request-id у span, свій middleware `x-request-id`, timeout, CORS, compression, JSON-fallback 404;
- **автентифікація**: JWT-екстрактор `AuthUser` (`FromRequestParts`), `Option<AuthUser>`, `AdminUser`, argon2 у `spawn_blocking`;
- **валідація**: `validator` і свій екстрактор `ValidatedJson<T>` (422 зі списком полів), пагінація `Page<T>` з `meta`;
- **експлуатація**: конфіг через `envy`/`dotenvy`, `tracing` (pretty або JSON), graceful shutdown, `/health` і `/ready`, міграції вбудовані в бінарник;
- **деплой**: multi-stage `Dockerfile` (cargo-chef для кешу залежностей → `distroless/cc`), `docker-compose.yml` з Postgres.

## Запуск через Docker (на хост Rust ставити не треба)

Усі команди виконуються з `orders-api/`:

```bash
cd orders-api

# API + Postgres → http://localhost:3000
docker compose up --build

# тести всередині rust:1-bookworm, разом з інтеграційними на Postgres
docker compose run --rm dev
docker compose run --rm dev cargo clippy --all-targets -- -D warnings
```

Кеш реєстру й `target/` зберігається в іменованих томах, тому другий запуск проходить швидко.

Одноразова команда без compose (з кореня репозиторію):

```bash
docker run --rm -v "$PWD":/work -w /work \
  -v cargo-registry:/usr/local/cargo/registry \
  rust:1-bookworm cargo test --lib     # юніт-тести, база не потрібна
```

## Запуск з локальним Rust

```bash
cd orders-api
cp .env.example .env
docker compose up -d db          # тільки Postgres
cargo run                        # міграції накочуються на старті
cargo test                       # юніт- та інтеграційні тести (#[sqlx::test] бере DATABASE_URL з .env)
cargo test --lib                 # тільки юніт-тести, база не потрібна
```

Спробувати руками:

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

### Маршрути

| Метод і шлях | Доступ | Що робить |
|---|---|---|
| `GET /health` | всі | процес живий |
| `GET /ready` | всі | база доступна |
| `POST /api/v1/auth/register` | всі | реєстрація: 201, або 409 якщо email уже є |
| `POST /api/v1/auth/login` | всі | JWT |
| `GET /api/v1/me` | `Option<AuthUser>` | гість або поточний користувач |
| `POST /api/v1/orders` | `AuthUser` | створити замовлення, позиції можна передати одразу |
| `GET /api/v1/orders` | `AuthUser` | свої замовлення: `page`, `per_page`, `status`, `customer_email`, `created_after`, `sort` |
| `GET /api/v1/orders/{id}` | `AuthUser` | одне замовлення; чуже повертає 404 |
| `POST /api/v1/orders/{id}/items` | `AuthUser` | додати позицію |
| `POST /api/v1/orders/{id}/pay` | `AuthUser` | оплатити |
| `POST /api/v1/orders/{id}/cancel` | `AuthUser` | скасувати |
| `GET /api/v1/admin/orders` | `AdminUser` | усі замовлення |

Адміна призначають прямо в базі: `UPDATE users SET role = 'admin' WHERE email = '…'`. Після цього треба залогінитися заново, бо роль записана в токені.

### `query_as` чи `query_as!`

Репозиторій використовує `sqlx::query_as::<_, T>(…)`: SQL перевіряється під час виконання, тому проєкт збирається без бази. У продакшні зазвичай беруть макроси `query!`/`query_as!`, які перевіряють SQL проти живої бази під час `cargo build`. Як перейти на них, описано в розділі 16d:

```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
cargo sqlx prepare            # створює .sqlx/ — комітиться в репозиторій
SQLX_OFFLINE=true cargo build # CI і Docker збирають без бази
```

Після переходу в `Dockerfile` перед `cargo build` треба додати `ENV SQLX_OFFLINE=true`, а `.sqlx/` має потрапити в контекст збірки.

## Розділи → директорії

| Розділ книги | Файл | Код |
|---|---|---|
| 1. Hello, world і базовий синтаксис | `01-basics` | [`chapters/01-basics`](chapters/01-basics) |
| 2. Керування потоком | `02-control` | [`chapters/02-control`](chapters/02-control) |
| 3. Функції та замикання | `03-functions` | [`chapters/03-functions`](chapters/03-functions) |
| 4. Структури — класи без класів | `04-structs` | [`chapters/04-structs`](chapters/04-structs) |
| 5. Enum, Option і прощання з null | `05-enums` | [`chapters/05-enums`](chapters/05-enums) |
| 6. Як влаштована мова: модель у голові | `06-concept` | [`chapters/06-concept`](chapters/06-concept) |
| 7. Трейти — інтерфейси, які вміють більше | `07-traits` | [`chapters/07-traits`](chapters/07-traits) |
| 8. Ownership — один власник на кожну річ | `08-ownership` | [`chapters/08-ownership`](chapters/08-ownership) |
| 9. Borrowing — позичити, не забираючи | `09-borrowing` | [`chapters/09-borrowing`](chapters/09-borrowing) |
| 10. Лайфтайми — коротко і без паніки | `10-lifetimes` | [`chapters/10-lifetimes`](chapters/10-lifetimes) |
| 11. Колекції та ітератори | `11-collections` | [`chapters/11-collections`](chapters/11-collections) |
| 12. Обробка помилок — Result замість винятків | `12-errors` | [`chapters/12-errors`](chapters/12-errors) |
| 13. Generics і розумні вказівники | `13-generics` | [`chapters/13-generics`](chapters/13-generics) |
| 14. Модулі, крейти і Cargo | `14-modules` | [`chapters/14-modules`](chapters/14-modules) |
| 15. Потоки | `15-threads` | [`chapters/15-threads`](chapters/15-threads) |
| 16. Async — на чому працюють веб-сервери | `16-async` | [`chapters/16-async`](chapters/16-async) |
| Проєкт: сервіс замовлень від `struct` до HTTP-API | `16b-project` | [`orders-api/src/domain.rs`](orders-api/src/domain.rs), [`error.rs`](orders-api/src/error.rs), [`repo.rs`](orders-api/src/repo.rs), [`handlers/orders.rs`](orders-api/src/handlers/orders.rs), [`main.rs`](orders-api/src/main.rs) |
| Middleware, автентифікація і стан застосунку | `16c-middleware` | [`middleware.rs`](orders-api/src/middleware.rs), [`auth.rs`](orders-api/src/auth.rs), [`state.rs`](orders-api/src/state.rs), [`handlers/auth.rs`](orders-api/src/handlers/auth.rs), [`lib.rs`](orders-api/src/lib.rs) |
| База даних глибше: sqlx без Eloquent | `16d-database` | [`repo.rs`](orders-api/src/repo.rs), [`migrations/`](orders-api/migrations) |
| Практики API: валідація, пагінація, версії | `16e-api` | [`validation.rs`](orders-api/src/validation.rs), [`pagination.rs`](orders-api/src/pagination.rs), [`filter.rs`](orders-api/src/filter.rs), [`error.rs`](orders-api/src/error.rs) |
| Деплой: один бінарник замість Forge | `16f-deploy` | [`config.rs`](orders-api/src/config.rs), [`Dockerfile`](orders-api/Dockerfile), [`docker-compose.yml`](orders-api/docker-compose.yml), [`ci.yml`](.github/workflows/ci.yml) |
| 17. Тести, макроси і що далі | `17-tests` | [`chapters/17-tests`](chapters/17-tests), [`orders-api/tests/`](orders-api/tests) |

## Версії

Rust 1.97, edition 2024. Мінімальна версія Rust: 1.88 для розділів (let chains у розділі 2), 1.94 для `orders-api` (через sqlx 0.9). Розділи 12, 16, 17 використовують ті самі версії tokio, axum, serde, thiserror, anyhow (плюс toml 0.9), що й orders-api. Залежності orders-api: axum 0.8.9, tokio 1.54, sqlx 0.9.0, tower-http 0.7.1, validator 0.21, jsonwebtoken 11.1 (бекенд `aws_lc_rs`), argon2 0.6, thiserror 2, serde 1, tracing-subscriber 0.3. Точні версії зафіксовані в `Cargo.lock`.
