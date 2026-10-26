# 3. Функції та замикання

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch03-functions --example 01-add
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch03-functions --example 01-add
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-add`](examples/01-add.rs) | Звичайна функція: типи параметрів обов'язкові, `->` для результату | `cargo run -p ch03-functions --example 01-add` |
| [`02-option-default`](examples/02-option-default.rs) | Немає дефолтних аргументів: необов'язкове передається як `Option` | `cargo run -p ch03-functions --example 02-option-default` |
| [`03-tuple-min-max`](examples/03-tuple-min-max.rs) | Кілька значень з функції — кортеж; зріз `&[i32]` як параметр | `cargo run -p ch03-functions --example 03-tuple-min-max` |
| [`04-closures`](examples/04-closures.rs) | Замикання: `\|x\| x * 2` — це `fn($x) => $x * 2` | `cargo run -p ch03-functions --example 04-closures` |
| [`05-fn-as-argument`](examples/05-fn-as-argument.rs) | Замість замикання можна передати ім'я функції чи методу | `cargo run -p ch03-functions --example 05-fn-as-argument` |
| [`06-capture`](examples/06-capture.rs) | Замикання бачить зовнішні змінні саме, без `use` | `cargo run -p ch03-functions --example 06-capture` |
| [`07-move`](examples/07-move.rs) | `move` — замикання забирає захоплене у власність | `cargo run -p ch03-functions --example 07-move` |
| [`08-fnmut-counter`](examples/08-fnmut-counter.rs) | Замикання змінює `count` → саме замикання має бути `mut` | `cargo run -p ch03-functions --example 08-fnmut-counter` |
| [`09-move-copy`](examples/09-move-copy.rs) | `move` з числами нічого не «забирає»: Copy-типи копіюються | `cargo run -p ch03-functions --example 09-move-copy` |
| [`10-iterators-with-closures`](examples/10-iterators-with-closures.rs) | `.map()`, `.filter()`, `.sum()` — як методи Laravel-колекції | `cargo run -p ch03-functions --example 10-iterators-with-closures` |
| [`11-impl-fn`](examples/11-impl-fn.rs) | Своя функція, яка приймає замикання: `impl Fn(i32) -> i32` | `cargo run -p ch03-functions --example 11-impl-fn` |
| [`12-methods`](examples/12-methods.rs) | Методи в `impl`: `&self` читає, `&mut self` змінює | `cargo run -p ch03-functions --example 12-methods` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: `stats` повертає (min, max, avg) кортежем | `cargo run -p ch03-functions --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: замикання зі знижкою, застосоване через `.map()` | `cargo run -p ch03-functions --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `move` з числом не ламає нічого, а з `String` — ламає | `cargo run -p ch03-functions --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: лічильник викликів усередині замикання (FnMut) | `cargo run -p ch03-functions --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `greet` з необов'язковим привітанням через `Option<&str>` | `cargo run -p ch03-functions --example exercise-5` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 03
rustc --edition 2024 chapters/03-functions/broken/07-use-after-move.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`07-use-after-move.rs`](broken/07-use-after-move.rs) | `move` забирає `String` у замикання: після цього змінна недоступна | error[E0382]: borrow of moved value: `name` |
| [`08-closure-not-mut.rs`](broken/08-closure-not-mut.rs) | Замикання, яке змінює зовнішню змінну, саме має бути `let mut` | error[E0596]: cannot borrow `inc` as mutable, as it is not declared as mutable |
| [`exercise-3.rs`](broken/exercise-3.rs) | Вправа 3 (зламана версія): `String` після `move`-замикання | error[E0382]: borrow of moved value: `label` |
| [`exercise-4.rs`](broken/exercise-4.rs) | Вправа 4 (зламана версія): FnMut-замикання без `mut` | error[E0596]: cannot borrow `apply` as mutable, as it is not declared as mutable |
