# 4. Структури — класи без класів

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch04-structs --example 01-struct-literal
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch04-structs --example 01-struct-literal
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-struct-literal`](examples/01-struct-literal.rs) | Оголошення структури і створення літералом з іменованими полями | `cargo run -p ch04-structs --example 01-struct-literal` |
| [`02-field-init-shorthand`](examples/02-field-init-shorthand.rs) | Скорочення `name` замість `name: name` | `cargo run -p ch04-structs --example 02-field-init-shorthand` |
| [`03-string-conversions`](examples/03-string-conversions.rs) | Чотири способи зробити `String` з `&str` | `cargo run -p ch04-structs --example 03-string-conversions` |
| [`05-impl-methods`](examples/05-impl-methods.rs) | Блок `impl`: конструктор `new`, методи з `&self` і `&mut self` | `cargo run -p ch04-structs --example 05-impl-methods` |
| [`06-self-and-self-type`](examples/06-self-and-self-type.rs) | `self` — екземпляр (`$this`), `Self` — тип (`self::`/`static::` у PHP) | `cargo run -p ch04-structs --example 06-self-and-self-type` |
| [`07-attributes-cfg`](examples/07-attributes-cfg.rs) | Атрибути: `#![allow]` на весь файл і `#[cfg]` — умовна компіляція | `cargo run -p ch04-structs --example 07-attributes-cfg` |
| [`08-derive-debug`](examples/08-derive-debug.rs) | `#[derive(Debug)]` — компілятор пише реалізацію `Debug` за тебе | `cargo run -p ch04-structs --example 08-derive-debug` |
| [`09-derive-list`](examples/09-derive-list.rs) | `#[derive(Debug, Clone, PartialEq, Default)]` у дії | `cargo run -p ch04-structs --example 09-derive-list` |
| [`10-partial-eq`](examples/10-partial-eq.rs) | Чому `PartialEq` є, а `Eq` для `f64` — ні: NaN не дорівнює собі | `cargo run -p ch04-structs --example 10-partial-eq` |
| [`11-derive-default`](examples/11-derive-default.rs) | `derive(Default)` і `..Default::default()` / `..інший_екземпляр` | `cargo run -p ch04-structs --example 11-derive-default` |
| [`12-manual-default`](examples/12-manual-default.rs) | Ручний `impl Default`, коли «нульові» значення не підходять | `cargo run -p ch04-structs --example 12-manual-default` |
| [`13-struct-update-moves`](examples/13-struct-update-moves.rs) | `..dev` переносить поля: `u16` копіюється, `String` переїжджає | `cargo run -p ch04-structs --example 13-struct-update-moves` |
| [`14-visibility`](examples/14-visibility.rs) | `pub` і приватні поля: ззовні модуля — тільки через конструктор | `cargo run -p ch04-structs --example 14-visibility` |
| [`15-tuple-and-unit-structs`](examples/15-tuple-and-unit-structs.rs) | Кортежні структури-обгортки і одиничні структури-маркери | `cargo run -p ch04-structs --example 15-tuple-and-unit-structs` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: структура `Product` з `#[derive(Debug)]` | `cargo run -p ch04-structs --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: `new`, `is_available`, `sell` з `saturating_sub` | `cargo run -p ch04-structs --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `sell` вимагає `let mut product` | `cargo run -p ch04-structs --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: вивід товару через `{:#?}` | `cargo run -p ch04-structs --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: що каже компілятор без `Debug` — і власний `impl Debug` замість derive | `cargo run -p ch04-structs --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: `PartialEq` працює, `Eq` — ні (поле `price: f64`) | `cargo run -p ch04-structs --example exercise-6` |
| [`exercise-7`](examples/exercise-7.rs) | Вправа 7: ручний `impl Default` і `..Default::default()` | `cargo run -p ch04-structs --example exercise-7` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 04
rustc --edition 2024 chapters/04-structs/broken/03-str-field.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`03-str-field.rs`](broken/03-str-field.rs) | Поле `&str` у структурі без лайфтайму не компілюється | error[E0106]: missing lifetime specifier |
| [`04-into-without-type.rs`](broken/04-into-without-type.rs) | `.into()` без відомого цільового типу | error[E0283]: type annotations needed |
| [`08-no-debug.rs`](broken/08-no-debug.rs) | `{:?}` для структури без `#[derive(Debug)]` | error[E0277]: `User` doesn't implement `Debug` |
| [`10-eq-on-f64.rs`](broken/10-eq-on-f64.rs) | `#[derive(Eq)]` на структурі з `f64` | error[E0277]: the trait bound `f64: Eq` is not satisfied |
| [`13-use-moved-field.rs`](broken/13-use-moved-field.rs) | Поле `String` переїхало через `..dev` — `dev.host` більше не використати | error[E0382]: borrow of moved value: `dev.host` |
| [`14-private-field.rs`](broken/14-private-field.rs) | Структуру з приватним полем не створити літералом ззовні модуля | error[E0451]: field `password_hash` of struct `User` is private |
| [`exercise-3.rs`](broken/exercise-3.rs) | Вправа 3 (зламана версія): метод з `&mut self` на незмінній змінній | error[E0596]: cannot borrow `product` as mutable, as it is not declared as mutable |
| [`exercise-5.rs`](broken/exercise-5.rs) | Вправа 5: без `Debug` у `derive` вивід через `{:?}` не компілюється | error[E0277]: `Product` doesn't implement `Debug` |
| [`exercise-6.rs`](broken/exercise-6.rs) | Вправа 6 (зламана версія): `Eq` на структурі з `f64` | error[E0277]: the trait bound `f64: Eq` is not satisfied |
