# 2. Керування потоком

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch02-control --example 01-if-expression
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch02-control --example 01-if-expression
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-if-expression`](examples/01-if-expression.rs) | `if` — вираз, тому тернарний оператор не потрібен | `cargo run -p ch02-control --example 01-if-expression` |
| [`02-if-bool`](examples/02-if-bool.rs) | Умова — явне порівняння, яке дає `bool` | `cargo run -p ch02-control --example 02-if-bool` |
| [`03-loop`](examples/03-loop.rs) | `loop` — нескінченний цикл до `break` (у PHP — `while (true)`) | `cargo run -p ch02-control --example 03-loop` |
| [`04-loop-break-value`](examples/04-loop-break-value.rs) | `break значення` — результат усього `loop` | `cargo run -p ch02-control --example 04-loop-break-value` |
| [`05-while`](examples/05-while.rs) | `while` — як у PHP, тільки без дужок навколо умови | `cargo run -p ch02-control --example 05-while` |
| [`06-for`](examples/06-for.rs) | `for` по діапазонах і колекціях, `&users`, `.iter().enumerate()` | `cargo run -p ch02-control --example 06-for` |
| [`07-clippy-loops`](examples/07-clippy-loops.rs) | Що скаже `cargo clippy` про цикли «в стилі PHP» — і як переписати | `cargo run -p ch02-control --example 07-clippy-loops` |
| [`08-labels`](examples/08-labels.rs) | Мітки циклів: `break 'outer` — це `break 2` у PHP | `cargo run -p ch02-control --example 08-labels` |
| [`09-match`](examples/09-match.rs) | `match` — `switch`, який подорослішав (і повертає значення) | `cargo run -p ch02-control --example 09-match` |
| [`10-match-ranges`](examples/10-match-ranges.rs) | `match` з діапазонами і guard-умовою | `cargo run -p ch02-control --example 10-match-ranges` |
| [`11-exhaustive`](examples/11-exhaustive.rs) | Вичерпний `match` по bool: обидві гілки, `_` не потрібен | `cargo run -p ch02-control --example 11-exhaustive` |
| [`12-unreachable-pattern`](examples/12-unreachable-pattern.rs) | `_` угорі ковтає все: гілки нижче недосяжні | `cargo run -p ch02-control --example 12-unreachable-pattern` |
| [`13-match-tuple`](examples/13-match-tuple.rs) | Деструктуризація кортежу прямо в гілці `match` | `cargo run -p ch02-control --example 13-match-tuple` |
| [`14-if-let`](examples/14-if-let.rs) | `if let` — коли цікавить одна гілка | `cargo run -p ch02-control --example 14-if-let` |
| [`15-let-else`](examples/15-let-else.rs) | `let else` — ранній вихід, якщо шаблон не збігся | `cargo run -p ch02-control --example 15-let-else` |
| [`16-while-let`](examples/16-while-let.rs) | `while let` — цикл, поки шаблон збігається | `cargo run -p ch02-control --example 16-while-let` |
| [`17-let-chains`](examples/17-let-chains.rs) | `if let ... && умова` — let chains (edition 2024, Rust 1.88+) | `cargo run -p ch02-control --example 17-let-chains` |
| [`18-shadowing`](examples/18-shadowing.rs) | Затінення: нова змінна з тим самим іменем (і, можливо, іншим типом) | `cargo run -p ch02-control --example 18-shadowing` |
| [`19-block-shadowing`](examples/19-block-shadowing.rs) | Пастка: затінення всередині блоку не змінює зовнішню змінну | `cargo run -p ch02-control --example 19-block-shadowing` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: `http_class` через `match` з діапазонами | `cargo run -p ch02-control --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: вичерпність без `_` — перелічуємо діапазони явно | `cargo run -p ch02-control --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: числа 0..=100, що діляться на 3, але не на 5 (`continue`) | `cargo run -p ch02-control --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `_` на початку `match` — функція завжди повертає "invalid" | `cargo run -p ch02-control --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: скільки подвоєнь, щоб 1 перевищило 1000 (`break значення`) | `cargo run -p ch02-control --example exercise-5` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 02
rustc --edition 2024 chapters/02-control/broken/02-if-not-bool.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`02-if-not-bool.rs`](broken/02-if-not-bool.rs) | Умова `if` має бути саме `bool`: «істинності» чисел немає | error[E0308]: mismatched types — expected `bool`, found integer |
| [`11-non-exhaustive.rs`](broken/11-non-exhaustive.rs) | `match` має бути вичерпним: забута гілка — помилка компіляції | error[E0004]: non-exhaustive patterns: `false` not covered |
| [`exercise-2.rs`](broken/exercise-2.rs) | Вправа 2: без гілки `_` match по u16 не вичерпний | error[E0004]: non-exhaustive patterns: |
