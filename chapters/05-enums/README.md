# 5. Enum, Option і прощання з null

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch05-enums --example 01-enum
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch05-enums --example 01-enum
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-enum`](examples/01-enum.rs) | Простий enum — як PHP 8.1 enum без значень | `cargo run -p ch05-enums --example 01-enum` |
| [`02-enum-with-data`](examples/02-enum-with-data.rs) | Кожен варіант enum несе свої дані | `cargo run -p ch05-enums --example 02-enum-with-data` |
| [`03-match-enum`](examples/03-match-enum.rs) | `match` по enum: дані витягуються прямо в гілці, `..` — «решта полів» | `cargo run -p ch05-enums --example 03-match-enum` |
| [`04-matches-macro`](examples/04-matches-macro.rs) | Методи enum і макрос `matches!` | `cargo run -p ch05-enums --example 04-matches-macro` |
| [`05-enum-eq`](examples/05-enum-eq.rs) | `==` (потрібен `PartialEq`, порівнює й дані) проти `matches!` (лише форма) | `cargo run -p ch05-enums --example 05-enum-eq` |
| [`06-enum-string-roundtrip`](examples/06-enum-string-roundtrip.rs) | enum у рядок і назад — аналог `->value` і `::tryFrom()` | `cargo run -p ch05-enums --example 06-enum-string-roundtrip` |
| [`07-option`](examples/07-option.rs) | `Option` замість null: `match` і `if let` | `cargo run -p ch05-enums --example 07-option` |
| [`08-option-methods`](examples/08-option-methods.rs) | Щоденні методи Option: is_some, map, unwrap_or, unwrap, expect, and_then | `cargo run -p ch05-enums --example 08-option-methods` |
| [`09-unwrap-or-lazy`](examples/09-unwrap-or-lazy.rs) | `unwrap_or` рахує дефолт завжди, `unwrap_or_else` — лише при None | `cargo run -p ch05-enums --example 09-unwrap-or-lazy` |
| [`10-option-in-struct`](examples/10-option-in-struct.rs) | `Option` у полях — аналог nullable-колонок | `cargo run -p ch05-enums --example 10-option-in-struct` |
| [`11-option-size`](examples/11-option-size.rs) | Розмір Option: для посилань і Box обгортка безкоштовна | `cargo run -p ch05-enums --example 11-option-size` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: enum `PaymentMethod` з даними у варіантах | `cargo run -p ch05-enums --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: комісія за способом оплати через `match` | `cargo run -p ch05-enums --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `find_discount` повертає `Option<f64>` | `cargo run -p ch05-enums --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: вивід знижки через `match`, потім через `.map()` + `.unwrap_or()` | `cargo run -p ch05-enums --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `is_cashless` через `matches!` | `cargo run -p ch05-enums --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: `code()` і новий варіант `Invoice` | `cargo run -p ch05-enums --example exercise-6` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 05
rustc --edition 2024 chapters/05-enums/broken/03-new-variant.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`03-new-variant.rs`](broken/03-new-variant.rs) | Новий варіант enum — і кожен `match` без нього перестає компілюватися | error[E0004]: non-exhaustive patterns: `&OrderStatus::Refunded` not covered |
| [`05-enum-eq-without-partialeq.rs`](broken/05-enum-eq-without-partialeq.rs) | `==` для enum без `PartialEq` | error[E0369]: binary operation `==` cannot be applied to type `Status` |
| [`07-option-field.rs`](broken/07-option-field.rs) | До поля всередині `Option` не дістатися без перевірки | error[E0609]: no field `name` on type `Option<User>` |
| [`exercise-5.rs`](broken/exercise-5.rs) | Вправа 5 (зламана версія): `!=` для enum без `PartialEq` | error[E0369]: binary operation `!=` cannot be applied to type `PaymentMethod` |
| [`exercise-6.rs`](broken/exercise-6.rs) | Вправа 6 (зламана версія): додали `Invoice`, а `match` у `code()` — ні | error[E0004]: non-exhaustive patterns: `&PaymentMethod::Invoice { .. }` not covered |
