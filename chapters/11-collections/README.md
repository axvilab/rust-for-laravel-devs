# 11. Collections and iterators — instead of PHP arrays

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch11-collections --example 01-vec
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch11-collections --example 01-vec
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-vec`](examples/01-vec.rs) | `Vec` — a list: push, len, [i], get, first/last, pop, insert, remove, contains | `cargo run -p ch11-collections --example 01-vec` |
| [`02-index-vs-get`](examples/02-index-vs-get.rs) | `[i]` panics out of bounds, `.get(i)` returns `Option<&T>`; `.copied()` removes the `&` | `cargo run -p ch11-collections --example 02-index-vs-get` |
| [`03-slices`](examples/03-slices.rs) | A `&[T]` slice — a view of a Vec, a part of it or an array | `cargo run -p ch11-collections --example 03-slices` |
| [`04-hashmap`](examples/04-hashmap.rs) | `HashMap`: insert, get → Option, entry().or_insert(), iteration, remove | `cargo run -p ch11-collections --example 04-hashmap` |
| [`05-entry-patterns`](examples/05-entry-patterns.rs) | All the `entry()` tricks: counting, grouping, and_modify, or_insert_with | `cargo run -p ch11-collections --example 05-entry-patterns` |
| [`06-iterator-chain`](examples/06-iterator-chain.rs) | A `.iter().filter().map().sum()` chain — like a Laravel Collection | `cargo run -p ch11-collections --example 06-iterator-chain` |
| [`07-collect`](examples/07-collect.rs) | `collect()` into a Vec, HashMap, HashSet, String; the turbofish `::<Vec<_>>` | `cargo run -p ch11-collections --example 07-collect` |
| [`08-three-ways-to-iterate`](examples/08-three-ways-to-iterate.rs) | `for s in &v` (borrow), `&mut v` (change), `v` (take) | `cargo run -p ch11-collections --example 08-three-ways-to-iterate` |
| [`09-into-iter-on-ref`](examples/09-into-iter-on-ref.rs) | `into_iter()` on `&Vec` borrows, on `Vec` it takes | `cargo run -p ch11-collections --example 09-into-iter-on-ref` |
| [`10-ref-to-ref`](examples/10-ref-to-ref.rs) | `&&f64` in `filter`: two asterisks, destructuring or `.copied()` | `cargo run -p ch11-collections --example 10-ref-to-ref` |
| [`11-sorting`](examples/11-sorting.rs) | Sorting a `Vec`: sort, sort_by, sort_by_key, partial_cmp for f64 | `cargo run -p ch11-collections --example 11-sorting` |
| [`12-total-cmp`](examples/12-total-cmp.rs) | `total_cmp` — sorting f64 without panicking on NaN | `cargo run -p ch11-collections --example 12-total-cmp` |
| [`13-sort-by-name`](examples/13-sort-by-name.rs) | Sorting by a string field: `sort_by` (no copies) or `sort_by_key` with `clone()` | `cargo run -p ch11-collections --example 13-sort-by-name` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: 10 orders — the sum of paid ones, ids of unpaid ones, the average order | `cargo run -p ch11-collections --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: grouping by `paid` into a `HashMap<bool, Vec<&Order>>` | `cargo run -p ch11-collections --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `top_n` — the n largest orders by `total`, without cloning the orders | `cargo run -p ch11-collections --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: `let v2 = v;` vs `let v2 = v.clone();` | `cargo run -p ch11-collections --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: word frequency — `HashMap` (random order) vs `BTreeMap` (stable) | `cargo run -p ch11-collections --example exercise-5` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 11
rustc --edition 2024 chapters/11-collections/broken/06-sum-needs-type.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`06-sum-needs-type.rs`](broken/06-sum-needs-type.rs) | `.sum()` without a type hint | error[E0283]: type annotations needed |
| [`09-for-moves-vec.rs`](broken/09-for-moves-vec.rs) | `for s in v` without `&` takes the vector | error[E0382]: borrow of moved value: `v` |
| [`13-sort-by-key-ref.rs`](broken/13-sort-by-key-ref.rs) | `sort_by_key` doesn't accept a reference key | error: lifetime may not live long enough |
| [`exercise-4.rs`](broken/exercise-4.rs) | Exercise 4 (broken version): using `v` after `let v2 = v;` | error[E0382]: borrow of moved value: `v` |
