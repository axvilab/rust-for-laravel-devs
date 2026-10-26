# 16. Async — на чому працюють веб-сервери

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch16-async --example 01-async-await
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch16-async --example 01-async-await
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-async-await`](examples/01-async-await.rs) | `async fn` повертає Future; `.await` чекає; `.await?` — дочекатися і прокинути помилку | `cargo run -p ch16-async --example 01-async-await` |
| [`02-tokio-main`](examples/02-tokio-main.rs) | `#[tokio::main]` — створює рантайм і запускає async main | `cargo run -p ch16-async --example 02-tokio-main` |
| [`03-futures-are-lazy`](examples/03-futures-are-lazy.rs) | Future нічого не робить, поки його не `.await`-нули | `cargo run -p ch16-async --example 03-futures-are-lazy` |
| [`04-tokio-spawn`](examples/04-tokio-spawn.rs) | `tokio::spawn(async move { ... })` — дешева задача, аналог `thread::spawn` | `cargo run -p ch16-async --example 04-tokio-spawn` |
| [`05-spawn-static-recipes`](examples/05-spawn-static-recipes.rs) | Три рецепти для `'static` у `tokio::spawn`: move, clone, Arc | `cargo run -p ch16-async --example 05-spawn-static-recipes` |
| [`06-many-tasks`](examples/06-many-tasks.rs) | Тисяча паралельних задач; `h.await??` — два рівні помилок | `cargo run -p ch16-async --example 06-many-tasks` |
| [`07-joinset`](examples/07-joinset.rs) | `JoinSet` віддає результати в міру готовності, а не по порядку | `cargo run -p ch16-async --example 07-joinset` |
| [`08-join-and-select`](examples/08-join-and-select.rs) | `join!` — чекати обидва одночасно; `select!` — хто перший; `timeout` | `cargo run -p ch16-async --example 08-join-and-select` |
| [`09-mutex-and-await`](examples/09-mutex-and-await.rs) | `std::sync::Mutex` не тримають через `.await`: закрити блоком або `tokio::sync::Mutex` | `cargo run -p ch16-async --example 09-mutex-and-await` |
| [`11-spawn-blocking`](examples/11-spawn-blocking.rs) | Не блокуй рантайм: `tokio::time::sleep`, `tokio::fs`, `spawn_blocking` для CPU-роботи | `cargo run -p ch16-async --example 11-spawn-blocking` |
| [`12-async-trait`](examples/12-async-trait.rs) | `async fn` у трейтах (Rust 1.75+) — без крейта async_trait | `cargo run -p ch16-async --example 12-async-trait` |
| [`13-pin-box-future`](examples/13-pin-box-future.rs) | `Pin<Box<dyn Future<Output = T> + Send>>` — коли гілки повертають різні futures | `cargo run -p ch16-async --example 13-pin-box-future` |
| [`14-axum-server`](examples/14-axum-server.rs) | Мінімальний Axum: State, Path, Json, Result, роут `/users/{id}` | `cargo run -p ch16-async --example 14-axum-server` |
| [`15-debug-handler`](examples/15-debug-handler.rs) | `#[axum::debug_handler]` — людські помилки для неправильних сигнатур хендлерів | `cargo run -p ch16-async --example 15-debug-handler` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: три `slow` послідовно (~600 мс) проти `tokio::join!` (~200 мс) | `cargo run -p ch16-async --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: те саме через `tokio::spawn` і цикл по `handles` | `cargo run -p ch16-async --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `std::thread::sleep` у `join!` — знову послідовно | `cargo run -p ch16-async --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `select!` з таймаутом 100 мс для функції, що спить 500 мс | `cargo run -p ch16-async --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: перший HTTP-сервер — `/hello/{name}` | `cargo run -p ch16-async --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: 5 задач, задача i спить (6 - i) * 100 мс — порядок у JoinSet і у Vec | `cargo run -p ch16-async --example exercise-6` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 16
rustc --edition 2024 chapters/16-async/broken/02-async-main-without-runtime.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`02-async-main-without-runtime.rs`](broken/02-async-main-without-runtime.rs) | `async fn main` без `#[tokio::main]` | error[E0752]: `main` function is not allowed to be `async` |
| [`10-drop-guard-does-not-help.rs`](broken/10-drop-guard-does-not-help.rs) | Пастка: `drop(guard)` перед `.await` НЕ допомагає (на Rust 1.97 — так само) | error: future cannot be sent between threads safely |
| [`10-std-mutex-across-await.rs`](broken/10-std-mutex-across-await.rs) | `std::sync::MutexGuard` живий під час `.await` — future перестає бути `Send` | error: future cannot be sent between threads safely |
