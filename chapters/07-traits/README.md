# 7. Трейти — інтерфейси, які вміють більше

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch07-traits --example 01-trait-as-interface
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch07-traits --example 01-trait-as-interface
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-trait-as-interface`](examples/01-trait-as-interface.rs) | Трейт = інтерфейс; реалізація — окремий блок `impl Trait for Type` | `cargo run -p ch07-traits --example 01-trait-as-interface` |
| [`02-default-methods`](examples/02-default-methods.rs) | Реалізація за замовчуванням: тип перевизначає лише те, що хоче | `cargo run -p ch07-traits --example 02-default-methods` |
| [`03-impl-generic-dyn`](examples/03-impl-generic-dyn.rs) | Три форми параметра: `&impl Trait`, `<T: Trait>`, `&dyn Trait` | `cargo run -p ch07-traits --example 03-impl-generic-dyn` |
| [`04-vec-box-dyn`](examples/04-vec-box-dyn.rs) | Список різнотипних об'єктів — `Vec<Box<dyn Trait>>` | `cargo run -p ch07-traits --example 04-vec-box-dyn` |
| [`05-return-box-dyn`](examples/05-return-box-dyn.rs) | «Або те, або інше» з функції — `Box<dyn Trait>` | `cargo run -p ch07-traits --example 05-return-box-dyn` |
| [`06-display`](examples/06-display.rs) | `Display` пишеться руками — людський формат для `{}` | `cargo run -p ch07-traits --example 06-display` |
| [`07-display-gives-to-string`](examples/07-display-gives-to-string.rs) | Реалізував `Display` — отримав `.to_string()` і `format!` безкоштовно | `cargo run -p ch07-traits --example 07-display-gives-to-string` |
| [`08-from-into`](examples/08-from-into.rs) | `From` — «з цього типу можна зробити мій»; `Into` — безкоштовно навпаки | `cargo run -p ch07-traits --example 08-from-into` |
| [`09-orphan-rule`](examples/09-orphan-rule.rs) | Обхід правила сироти: свій тип-обгортка навколо `Vec<String>` | `cargo run -p ch07-traits --example 09-orphan-rule` |
| [`10-trait-in-scope`](examples/10-trait-in-scope.rs) | `use std::fmt::Write` — без імпорту трейту `write!` на `String` не знайдеться | `cargo run -p ch07-traits --example 10-trait-in-scope` |
| [`11-operator-add`](examples/11-operator-add.rs) | Оператори — теж трейти: `impl Add for Money` | `cargo run -p ch07-traits --example 11-operator-add` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: трейт `Priced` з дефолтним `price_with_vat` | `cargo run -p ch07-traits --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: `Priced` для товару і для послуги (ціна = ставка × години) | `cargo run -p ch07-traits --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `total` по списку `Box<dyn Priced>` з товарів і послуг упереміш | `cargo run -p ch07-traits --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `Display` для `Product`: "Ноутбук — 45000.00 грн" | `cargo run -p ch07-traits --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `#[derive(Debug, Clone, PartialEq)]` і `product == product.clone()` | `cargo run -p ch07-traits --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Вправа 6: `to_string()` без жодного нового коду | `cargo run -p ch07-traits --example exercise-6` |
| [`exercise-7`](examples/exercise-7.rs) | Вправа 7: `From<f64> for Product`; `.into()` потребує анотації типу | `cargo run -p ch07-traits --example exercise-7` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 07
rustc --edition 2024 chapters/07-traits/broken/05-impl-trait-two-types.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`05-impl-trait-two-types.rs`](broken/05-impl-trait-two-types.rs) | `-> impl Trait` — це ОДИН прихований тип, а не «будь-який з кількох» | error[E0308]: `if` and `else` have incompatible types |
| [`08-into-needs-type.rs`](broken/08-into-needs-type.rs) | `.into()` без анотації: компілятор не знає, у що перетворювати | error[E0283]: type annotations needed |
| [`09-orphan-rule.rs`](broken/09-orphan-rule.rs) | Правило сироти: чужий трейт для чужого типу реалізувати не можна | error[E0117]: only traits defined in the current crate can be |
| [`10-trait-not-in-scope.rs`](broken/10-trait-not-in-scope.rs) | Метод трейту не видно без `use` | error[E0599]: cannot write into `String` |
| [`11-use-after-add.rs`](broken/11-use-after-add.rs) | `+` забирає доданки (`self`, не `&self`): після додавання їх не використати | error[E0382]: borrow of moved value: `price` |
| [`exercise-7.rs`](broken/exercise-7.rs) | Вправа 7 (зламана версія): `.into()` без анотації типу | error[E0283]: type annotations needed |
