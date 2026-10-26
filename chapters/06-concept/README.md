# 6. Як влаштована мова: модель у голові

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch06-concept --example 01-data-behavior-contract
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch06-concept --example 01-data-behavior-contract
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-data-behavior-contract`](examples/01-data-behavior-contract.rs) | Дані (`struct`), поведінка (`impl`) і контракт (`trait`) — три окремі блоки | `cargo run -p ch06-concept --example 01-data-behavior-contract` |
| [`02-composition`](examples/02-composition.rs) | Композиція замість наслідування: спільна частина — окремою структурою | `cargo run -p ch06-concept --example 02-composition` |
| [`03-trait-default-method`](examples/03-trait-default-method.rs) | Трейт з реалізацією за замовчуванням — це PHP-трейт з перевіркою типів | `cargo run -p ch06-concept --example 03-trait-default-method` |
| [`04-generic-function`](examples/04-generic-function.rs) | Generic-функція `send<T: Mailable>` — як `function send(Mailable $item)` | `cargo run -p ch06-concept --example 04-generic-function` |
| [`05-no-constructors`](examples/05-no-constructors.rs) | Конструкторів немає: літерал з усіма полями, а `new` — звичайна функція | `cargo run -p ch06-concept --example 05-no-constructors` |
| [`06-extend-foreign-type`](examples/06-extend-foreign-type.rs) | Свій трейт для чужого типу: метод з'являється навіть у `String` (як `Collection::macro`) | `cargo run -p ch06-concept --example 06-extend-foreign-type` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 06
rustc --edition 2024 chapters/06-concept/broken/05-missing-field.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`05-missing-field.rs`](broken/05-missing-field.rs) | «Поле не ініціалізоване» неможливе: літерал має перелічити всі поля | error[E0063]: missing field `paid` in initializer of `Invoice` |
