# 14. Модулі, крейти і Cargo — як організувати проєкт

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch14-modules --example 01-modules
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch14-modules --example 01-modules
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-modules`](examples/01-modules/) | Модулі як у книзі: `mod` у main.rs, файли models.rs, services.rs, services/*.rs | `cargo run -p ch14-modules --example 01-modules` |
| [`02-paths-and-visibility`](examples/02-paths-and-visibility.rs) | Шляхи `crate::`, `super::`, `self::` і рівні видимості `pub`, `pub(crate)`, `pub(super)` | `cargo run -p ch14-modules --example 02-paths-and-visibility` |
| [`03-reexport`](examples/03-reexport.rs) | `pub use` — реекспорт, щоб сховати внутрішню структуру модулів | `cargo run -p ch14-modules --example 03-reexport` |
| [`04-use-library-crate`](examples/04-use-library-crate.rs) | Приклад підключає бібліотеку цього ж пакета ззовні: дефіс у назві → підкреслення | `cargo run -p ch14-modules --example 04-use-library-crate` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: «shop» по модулях — models, pricing, errors (див. src/ цього крейта) | `cargo run -p ch14-modules --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: без `pub` функція приватна для свого модуля | `cargo run -p ch14-modules --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `cargo add thiserror anyhow`, `cargo tree` — хто тягне `syn` | `cargo run -p ch14-modules --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: що `cargo clippy` пропонує найчастіше (уже виправлено) і `cargo fmt` | `cargo run -p ch14-modules --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `calc_fee` у lib.rs з doc-прикладом — і чому без `use` він падає | `cargo run -p ch14-modules --example exercise-5` |

## Сам крейт

```bash
cargo run -p ch14-modules                      # src/main.rs
cargo run -p ch14-modules --bin report         # src/bin/report.rs
cargo test -p ch14-modules                     # юніт-, інтеграційні та doc-тести
```

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 14
rustc --edition 2024 chapters/14-modules/broken/02-private-function.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`02-private-function.rs`](broken/02-private-function.rs) | Виклик приватної функції з іншого модуля | error[E0603]: function `format_money` is private |
| [`03-mod-for-dependency.rs`](broken/03-mod-for-dependency.rs) | `mod serde;` для залежності з Cargo.toml — так не роблять | error[E0583]: file not found for module `serde` |
