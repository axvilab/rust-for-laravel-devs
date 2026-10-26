# 13. Generics і розумні вказівники

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch13-generics --example 01-generic-function
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch13-generics --example 01-generic-function
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-generic-function`](examples/01-generic-function.rs) | Generic-функція `first<T>` — одна для будь-якого типу, без втрати типу | `cargo run -p ch13-generics --example 01-generic-function` |
| [`02-largest`](examples/02-largest.rs) | Обмеження `T: PartialOrd` — «будь-який тип, який можна порівнювати» | `cargo run -p ch13-generics --example 02-largest` |
| [`03-where`](examples/03-where.rs) | Кілька обмежень через `+` і блок `where` | `cargo run -p ch13-generics --example 03-where` |
| [`04-generic-struct`](examples/04-generic-struct.rs) | Generic-структура `Paginated<T>` і методи лише для `T: Clone` | `cargo run -p ch13-generics --example 04-generic-struct` |
| [`05-associated-type`](examples/05-associated-type.rs) | Асоційований тип `type Item`: реалізуй `next` — отримай усі методи ітераторів | `cargo run -p ch13-generics --example 05-associated-type` |
| [`06-impl-trait-return`](examples/06-impl-trait-return.rs) | `-> impl Iterator<Item = u32>`: тип, який неможливо записати руками | `cargo run -p ch13-generics --example 06-impl-trait-return` |
| [`07-box-recursive`](examples/07-box-recursive.rs) | `Box` для рекурсивних типів; `Vec` усередині — без Box | `cargo run -p ch13-generics --example 07-box-recursive` |
| [`08-rc`](examples/08-rc.rs) | `Rc` — кілька власників в одному потоці; `Rc::clone` лише +1 до лічильника | `cargo run -p ch13-generics --example 08-rc` |
| [`09-arc-clone`](examples/09-arc-clone.rs) | `Arc::clone(&x)` замість `x.clone()`: видно, що це дешевий «пропуск», а не копія | `cargo run -p ch13-generics --example 09-arc-clone` |
| [`10-refcell`](examples/10-refcell.rs) | `Rc<RefCell<T>>`: перевірка позичань у рантаймі; порушення = паніка | `cargo run -p ch13-generics --example 10-refcell` |
| [`11-shared-state-table`](examples/11-shared-state-table.rs) | Що з чим поєднувати: Rc / Arc / Rc<RefCell> / Arc<Mutex> / Arc<RwLock> / Box | `cargo run -p ch13-generics --example 11-shared-state-table` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: `count_where` з generic `T` і замиканням-предикатом | `cargo run -p ch13-generics --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: `Cache<K, V>` — HashMap вимагає від ключа `Eq + Hash` | `cargo run -p ch13-generics --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: дерево коментарів через `Vec` (без Box) і ланцюжок через `Option<Box<_>>` | `cargo run -p ch13-generics --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `Rc<String>`, три клони, `strong_count` до і після `drop` | `cargo run -p ch13-generics --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: `Iterator` для Фібоначчі; перші 10 через `take(10)` | `cargo run -p ch13-generics --example exercise-5` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 13
rustc --edition 2024 chapters/13-generics/broken/02-largest-without-bound.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`02-largest-without-bound.rs`](broken/02-largest-without-bound.rs) | Generic без обмеження: компілятор не знає, чи можна порівнювати `T` | error[E0369]: binary operation `>` cannot be applied to type `&T` |
| [`07-recursive-without-box.rs`](broken/07-recursive-without-box.rs) | Рекурсивний тип без `Box` має нескінченний розмір | error[E0072]: recursive type `Expr` has infinite size |
| [`09-arc-clone-on-plain-value.rs`](broken/09-arc-clone-on-plain-value.rs) | `Arc::clone(&x)` для значення, яке не в `Arc`, не компілюється — і це плюс | error[E0308]: mismatched types (expected `&Arc<_>`, found `&Config`) |
| [`exercise-3.rs`](broken/exercise-3.rs) | Вправа 3 (зламана версія): `Option<Reply>` усередині `Reply` | error[E0072]: recursive type `Reply` has infinite size |
