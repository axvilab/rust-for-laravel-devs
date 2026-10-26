# 12. Обробка помилок — Result замість винятків

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch12-errors --example 01-result
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch12-errors --example 01-result
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-result`](examples/01-result.rs) | `Result<T, E>`: функція, яка може не впоратися, і `match` по результату | `cargo run -p ch12-errors --example 01-result` |
| [`02-question-mark`](examples/02-question-mark.rs) | Оператор `?`: якщо `Err` — повернути його з функції, якщо `Ok` — йти далі | `cargo run -p ch12-errors --example 02-question-mark` |
| [`03-question-mark-desugared`](examples/03-question-mark-desugared.rs) | Що `?` робить за тебе: той самий код через `match` | `cargo run -p ch12-errors --example 03-question-mark-desugared` |
| [`04-main-result`](examples/04-main-result.rs) | `main` повертає `Result`: помилка друкується через `{:?}`, код виходу 1 | `cargo run -p ch12-errors --example 04-main-result` |
| [`05-box-dyn-error`](examples/05-box-dyn-error.rs) | `Result<(), Box<dyn Error>>` — «будь-яка помилка в коробці» для main | `cargo run -p ch12-errors --example 05-box-dyn-error` |
| [`06-error-enum`](examples/06-error-enum.rs) | Власний enum помилок: хендлер робить `match` і вибирає HTTP-код | `cargo run -p ch12-errors --example 06-error-enum` |
| [`07-from-for-question`](examples/07-from-for-question.rs) | `impl From<io::Error> for AppError` — і `?` сам конвертує помилки | `cargo run -p ch12-errors --example 07-from-for-question` |
| [`08-thiserror`](examples/08-thiserror.rs) | thiserror: `#[error]` генерує Display, `#[from]` — impl From | `cargo run -p ch12-errors --example 08-thiserror` |
| [`09-anyhow-context`](examples/09-anyhow-context.rs) | anyhow + `.context()`: людський ланцюжок причин у `main` | `cargo run -p ch12-errors --example 09-anyhow-context` |
| [`10-panics`](examples/10-panics.rs) | Паніка — для багів: expect, індекс за межами, unwrap, assert!, unreachable!, todo! | `cargo run -p ch12-errors --example 10-panics` |
| [`11-result-methods`](examples/11-result-methods.rs) | Методи Result: unwrap_or, unwrap_or_else, map, map_err, ok, and_then, ok_or | `cargo run -p ch12-errors --example 11-result-methods` |
| [`12-unwrap-or-is-eager`](examples/12-unwrap-or-is-eager.rs) | `unwrap_or` рахує дефолт завжди, `unwrap_or_else` — лише при `Err` | `cargo run -p ch12-errors --example 12-unwrap-or-is-eager` |
| [`13-collect-result`](examples/13-collect-result.rs) | `collect()` у `Result<Vec<_>, _>` зупиняється на першій помилці; `partition` — зібрати все | `cargo run -p ch12-errors --example 13-collect-result` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: `parse_age` — не число або > 150 → `Err` | `cargo run -p ch12-errors --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: `parse_user` прокидає помилку `parse_age` нагору через `?` | `cargo run -p ch12-errors --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `ValidationError` + `impl From<String>`, щоб `?` з `parse_age` працював | `cargo run -p ch12-errors --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: той самий `ValidationError` на thiserror; вивід через `{}` | `cargo run -p ch12-errors --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `anyhow` + `.context()` навколо всієї обробки в `main` | `cargo run -p ch12-errors --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: `parse_ids` — `split`, `map`, `collect()` у `Result<Vec<_>, _>` | `cargo run -p ch12-errors --example exercise-6` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 12
rustc --edition 2024 chapters/12-errors/broken/04-question-in-unit-main.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`04-question-in-unit-main.rs`](broken/04-question-in-unit-main.rs) | `?` у звичайному `main`, який нічого не повертає | error[E0277]: the `?` operator can only be used in a function |
