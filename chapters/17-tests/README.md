# 17. Тести, макроси і що далі

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch17-tests --example 01-macro-rules
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch17-tests --example 01-macro-rules
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-macro-rules`](examples/01-macro-rules.rs) | Декларативний макрос `hashmap!`: `$k:expr`, повтор `$( ... ),*`, кома в кінці `$(,)?` | `cargo run -p ch17-tests --example 01-macro-rules` |
| [`02-serde`](examples/02-serde.rs) | Serde: `from_str` / `to_string` і атрибути default, rename, skip_serializing | `cargo run -p ch17-tests --example 02-serde` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: тест, що навмисно падає, з поясненням у `assert_eq!` | `cargo run -p ch17-tests --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: `println!` у тесті, що проходить — видно лише з `--nocapture` | `cargo run -p ch17-tests --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `#[sqlx::test]` для `OrderRepo` — зберегти, оплатити, зберегти, прочитати | `cargo run -p ch17-tests --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: свій `macro_rules! hashset` за зразком `hashmap!` | `cargo run -p ch17-tests --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: у що розгортається `#[derive(Debug, Clone)]` (як показав би `cargo expand`) | `cargo run -p ch17-tests --example exercise-5` |

## Сам крейт

```bash
cargo test -p ch17-tests                     # юніт-, інтеграційні та doc-тести
```

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 17
rustc --edition 2024 chapters/17-tests/broken/01-async-test-without-runtime.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`01-async-test-without-runtime.rs`](broken/01-async-test-without-runtime.rs) | `#[test]` над `async fn` — тест-раннер не вміє чекати на future | error: async functions cannot be used for tests |
