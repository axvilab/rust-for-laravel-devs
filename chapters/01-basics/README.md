# 1. Hello, world і базовий синтаксис.

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch01-basics --example 01-hello
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch01-basics --example 01-hello
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-hello`](examples/01-hello.rs) | Перша програма: `fn main` і макрос `println!` | `cargo run -p ch01-basics --example 01-hello` |
| [`02-mut`](examples/02-mut.rs) | `let mut` — змінна, яку можна змінювати | `cargo run -p ch01-basics --example 02-mut` |
| [`03-warnings`](examples/03-warnings.rs) | Попередження компілятора: невикористана змінна і `_`-префікс | `cargo run -p ch01-basics --example 03-warnings` |
| [`04-const`](examples/04-const.rs) | `const` — справжня константа, тип обов'язковий | `cargo run -p ch01-basics --example 04-const` |
| [`05-types`](examples/05-types.rs) | Виведення типів і явна анотація | `cargo run -p ch01-basics --example 05-types` |
| [`06-as-cast`](examples/06-as-cast.rs) | `as` — явне перетворення числових типів | `cargo run -p ch01-basics --example 06-as-cast` |
| [`07-overflow`](examples/07-overflow.rs) | Переповнення: `checked_*`, `saturating_*`, `wrapping_*` і мовчазний `as` | `cargo run -p ch01-basics --example 07-overflow` |
| [`08-strings`](examples/08-strings.rs) | `&str` і `String`: літерал, власний рядок, `format!` | `cargo run -p ch01-basics --example 08-strings` |
| [`09-utf8-len`](examples/09-utf8-len.rs) | Довжина рядка: байти (`len`) проти символів (`chars().count()`) | `cargo run -p ch01-basics --example 09-utf8-len` |
| [`10-format`](examples/10-format.rs) | Форматування: позиційно, іменовано, `{:?}`, `{:#?}`, вирівнювання, точність | `cargo run -p ch01-basics --example 10-format` |
| [`11-dbg`](examples/11-dbg.rs) | `println!` (stdout), `eprintln!` (stderr) і `dbg!` — справжній `dd()` | `cargo run -p ch01-basics --example 11-dbg` |
| [`12-block-expression`](examples/12-block-expression.rs) | Блок `{ ... }` — вираз: значення останнього рядка без `;` | `cargo run -p ch01-basics --example 12-block-expression` |
| [`13-return`](examples/13-return.rs) | Значення функції — останній вираз; `return` можливий, але не ідіоматичний | `cargo run -p ch01-basics --example 13-return` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: ціна (f64) × кількість (u32) — потрібне явне перетворення | `cargo run -p ch01-basics --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: два знаки після коми через `{:.2}` | `cargo run -p ch01-basics --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: без `mut` повторне присвоєння не компілюється | `cargo run -p ch01-basics --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `dbg!` посеред виразу і порядок виводу | `cargo run -p ch01-basics --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `checked_add` і `saturating_add` для u8 | `cargo run -p ch01-basics --example exercise-5` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 01
rustc --edition 2024 chapters/01-basics/broken/02-immutable.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`02-immutable.rs`](broken/02-immutable.rs) | Змінні незмінні за замовчуванням: повторне присвоєння не компілюється | error[E0384]: cannot assign twice to immutable variable `count` |
| [`06-no-implicit-conversion.rs`](broken/06-no-implicit-conversion.rs) | Немає неявних перетворень: i32 + f64 не компілюється | error[E0277]: cannot add `f64` to `i32` |
| [`13-extra-semicolon.rs`](broken/13-extra-semicolon.rs) | Зайва `;` після останнього виразу: функція повертає `()` замість i32 | error[E0308]: mismatched types — expected `i32`, found `()` |
| [`exercise-3.rs`](broken/exercise-3.rs) | Вправа 3 (зламана версія): присвоєння незмінній змінній | error[E0384]: cannot assign twice to immutable variable `price` |
| [`exercise-5.rs`](broken/exercise-5.rs) | Вправа 5 (зламана версія): переповнення, відоме під час компіляції | error: this arithmetic operation will overflow |
