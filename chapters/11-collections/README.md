# 11. Колекції та ітератори — замість PHP-масивів

Код до розділу книги. Кожен приклад — окрема програма в `examples/`.

```bash
cargo run -p ch11-collections --example 01-vec
# без Rust на машині:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch11-collections --example 01-vec
```

Команди запускаються з кореня репозиторію.

## Приклади з тексту

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`01-vec`](examples/01-vec.rs) | `Vec` — список: push, len, [i], get, first/last, pop, insert, remove, contains | `cargo run -p ch11-collections --example 01-vec` |
| [`02-index-vs-get`](examples/02-index-vs-get.rs) | `[i]` панікує за межами, `.get(i)` повертає `Option<&T>`; `.copied()` знімає `&` | `cargo run -p ch11-collections --example 02-index-vs-get` |
| [`03-slices`](examples/03-slices.rs) | Зріз `&[T]` — погляд на Vec, його частину або масив | `cargo run -p ch11-collections --example 03-slices` |
| [`04-hashmap`](examples/04-hashmap.rs) | `HashMap`: insert, get → Option, entry().or_insert(), ітерація, remove | `cargo run -p ch11-collections --example 04-hashmap` |
| [`05-entry-patterns`](examples/05-entry-patterns.rs) | Усі прийоми `entry()`: підрахунок, групування, and_modify, or_insert_with | `cargo run -p ch11-collections --example 05-entry-patterns` |
| [`06-iterator-chain`](examples/06-iterator-chain.rs) | Ланцюжок `.iter().filter().map().sum()` — як Laravel Collection | `cargo run -p ch11-collections --example 06-iterator-chain` |
| [`07-collect`](examples/07-collect.rs) | `collect()` у Vec, HashMap, HashSet, String; турбориба `::<Vec<_>>` | `cargo run -p ch11-collections --example 07-collect` |
| [`08-three-ways-to-iterate`](examples/08-three-ways-to-iterate.rs) | `for s in &v` (позичити), `&mut v` (змінювати), `v` (забрати) | `cargo run -p ch11-collections --example 08-three-ways-to-iterate` |
| [`09-into-iter-on-ref`](examples/09-into-iter-on-ref.rs) | `into_iter()` на `&Vec` позичає, на `Vec` — забирає | `cargo run -p ch11-collections --example 09-into-iter-on-ref` |
| [`10-ref-to-ref`](examples/10-ref-to-ref.rs) | `&&f64` у `filter`: дві зірочки, деструктуризація або `.copied()` | `cargo run -p ch11-collections --example 10-ref-to-ref` |
| [`11-sorting`](examples/11-sorting.rs) | Сортування `Vec`: sort, sort_by, sort_by_key, partial_cmp для f64 | `cargo run -p ch11-collections --example 11-sorting` |
| [`12-total-cmp`](examples/12-total-cmp.rs) | `total_cmp` — сортування f64 без паніки на NaN | `cargo run -p ch11-collections --example 12-total-cmp` |
| [`13-sort-by-name`](examples/13-sort-by-name.rs) | Сортування за рядковим полем: `sort_by` (без копій) або `sort_by_key` з `clone()` | `cargo run -p ch11-collections --example 13-sort-by-name` |

## Розв'язки вправ «Спробуй»

| Приклад | Що показує | Команда запуску |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Вправа 1: 10 замовлень — сума оплачених, id неоплачених, середній чек | `cargo run -p ch11-collections --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Вправа 2: групування за `paid` у `HashMap<bool, Vec<&Order>>` | `cargo run -p ch11-collections --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Вправа 3: `top_n` — n найбільших замовлень за `total`, без клонування самих замовлень | `cargo run -p ch11-collections --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Вправа 4: `let v2 = v;` проти `let v2 = v.clone();` | `cargo run -p ch11-collections --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Вправа 5: частота слів — `HashMap` (порядок випадковий) проти `BTreeMap` (стабільний) | `cargo run -p ch11-collections --example exercise-5` |

## Навмисно зламані приклади

У `broken/` лежить код, який книга показує як помилку компіляції. Cargo їх не збирає.
У шапці кожного файлу записано очікувану помилку і виправлення.
Перевірити, що вони справді не компілюються і з тією самою помилкою:

```bash
./scripts/check-broken.sh 11
rustc --edition 2024 chapters/11-collections/broken/06-sum-needs-type.rs   # побачити повідомлення компілятора
```

| Файл | Що показує | Очікувана помилка |
|---|---|---|
| [`06-sum-needs-type.rs`](broken/06-sum-needs-type.rs) | `.sum()` без підказки типу | error[E0283]: type annotations needed |
| [`09-for-moves-vec.rs`](broken/09-for-moves-vec.rs) | `for s in v` без `&` забирає вектор | error[E0382]: borrow of moved value: `v` |
| [`13-sort-by-key-ref.rs`](broken/13-sort-by-key-ref.rs) | `sort_by_key` не приймає ключ-посилання | error: lifetime may not live long enough |
| [`exercise-4.rs`](broken/exercise-4.rs) | Вправа 4 (зламана версія): використання `v` після `let v2 = v;` | error[E0382]: borrow of moved value: `v` |
