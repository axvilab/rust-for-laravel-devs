# 13. Generics and smart pointers

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch13-generics --example 01-generic-function
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch13-generics --example 01-generic-function
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-generic-function`](examples/01-generic-function.rs) | A generic function `first<T>` — one for any type, without losing the type | `cargo run -p ch13-generics --example 01-generic-function` |
| [`02-largest`](examples/02-largest.rs) | The bound `T: PartialOrd` — "any type that can be compared" | `cargo run -p ch13-generics --example 02-largest` |
| [`03-where`](examples/03-where.rs) | Several bounds with `+` and a `where` block | `cargo run -p ch13-generics --example 03-where` |
| [`04-generic-struct`](examples/04-generic-struct.rs) | A generic struct `Paginated<T>` and methods only for `T: Clone` | `cargo run -p ch13-generics --example 04-generic-struct` |
| [`05-associated-type`](examples/05-associated-type.rs) | An associated type `type Item`: implement `next` — get all the iterator methods | `cargo run -p ch13-generics --example 05-associated-type` |
| [`06-impl-trait-return`](examples/06-impl-trait-return.rs) | `-> impl Iterator<Item = u32>`: a type you can't write by hand | `cargo run -p ch13-generics --example 06-impl-trait-return` |
| [`07-box-recursive`](examples/07-box-recursive.rs) | `Box` for recursive types; a `Vec` inside — no Box needed | `cargo run -p ch13-generics --example 07-box-recursive` |
| [`08-rc`](examples/08-rc.rs) | `Rc` — several owners in one thread; `Rc::clone` is just +1 to the counter | `cargo run -p ch13-generics --example 08-rc` |
| [`09-arc-clone`](examples/09-arc-clone.rs) | `Arc::clone(&x)` instead of `x.clone()`: it's visibly a cheap "pass", not a copy | `cargo run -p ch13-generics --example 09-arc-clone` |
| [`10-refcell`](examples/10-refcell.rs) | `Rc<RefCell<T>>`: borrow checks at runtime; a violation = a panic | `cargo run -p ch13-generics --example 10-refcell` |
| [`11-shared-state-table`](examples/11-shared-state-table.rs) | What goes with what: Rc / Arc / Rc<RefCell> / Arc<Mutex> / Arc<RwLock> / Box | `cargo run -p ch13-generics --example 11-shared-state-table` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: `count_where` with a generic `T` and a predicate closure | `cargo run -p ch13-generics --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: `Cache<K, V>` — HashMap requires `Eq + Hash` from the key | `cargo run -p ch13-generics --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: a comment tree with `Vec` (no Box) and a chain with `Option<Box<_>>` | `cargo run -p ch13-generics --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: `Rc<String>`, three clones, `strong_count` before and after `drop` | `cargo run -p ch13-generics --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: an `Iterator` for Fibonacci; the first 10 with `take(10)` | `cargo run -p ch13-generics --example exercise-5` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 13
rustc --edition 2024 chapters/13-generics/broken/02-largest-without-bound.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`02-largest-without-bound.rs`](broken/02-largest-without-bound.rs) | A generic without a bound: the compiler doesn't know whether `T` can be compared | error[E0369]: binary operation `>` cannot be applied to type `&T` |
| [`07-recursive-without-box.rs`](broken/07-recursive-without-box.rs) | A recursive type without `Box` has infinite size | error[E0072]: recursive type `Expr` has infinite size |
| [`09-arc-clone-on-plain-value.rs`](broken/09-arc-clone-on-plain-value.rs) | `Arc::clone(&x)` for a value that isn't in an `Arc` doesn't compile — and that's a plus | error[E0308]: mismatched types (expected `&Arc<_>`, found `&Config`) |
| [`exercise-3.rs`](broken/exercise-3.rs) | Exercise 3 (broken version): `Option<Reply>` inside `Reply` | error[E0072]: recursive type `Reply` has infinite size |
