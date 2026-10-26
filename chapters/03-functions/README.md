# 3. Functions and closures

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch03-functions --example 01-add
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch03-functions --example 01-add
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-add`](examples/01-add.rs) | A plain function: parameter types are mandatory, `->` for the result | `cargo run -p ch03-functions --example 01-add` |
| [`02-option-default`](examples/02-option-default.rs) | No default arguments: an optional value is passed as an `Option` | `cargo run -p ch03-functions --example 02-option-default` |
| [`03-tuple-min-max`](examples/03-tuple-min-max.rs) | Several values from a function — a tuple; a `&[i32]` slice as the parameter | `cargo run -p ch03-functions --example 03-tuple-min-max` |
| [`04-closures`](examples/04-closures.rs) | Closures: `\|x\| x * 2` is `fn($x) => $x * 2` | `cargo run -p ch03-functions --example 04-closures` |
| [`05-fn-as-argument`](examples/05-fn-as-argument.rs) | Instead of a closure you can pass the name of a function or method | `cargo run -p ch03-functions --example 05-fn-as-argument` |
| [`06-capture`](examples/06-capture.rs) | A closure sees outer variables by itself, no `use` needed | `cargo run -p ch03-functions --example 06-capture` |
| [`07-move`](examples/07-move.rs) | `move` — the closure takes ownership of what it captures | `cargo run -p ch03-functions --example 07-move` |
| [`08-fnmut-counter`](examples/08-fnmut-counter.rs) | The closure changes `count` → the closure itself must be `mut` | `cargo run -p ch03-functions --example 08-fnmut-counter` |
| [`09-move-copy`](examples/09-move-copy.rs) | `move` with numbers "takes" nothing: Copy types are copied | `cargo run -p ch03-functions --example 09-move-copy` |
| [`10-iterators-with-closures`](examples/10-iterators-with-closures.rs) | `.map()`, `.filter()`, `.sum()` — like Laravel collection methods | `cargo run -p ch03-functions --example 10-iterators-with-closures` |
| [`11-impl-fn`](examples/11-impl-fn.rs) | Your own function that takes a closure: `impl Fn(i32) -> i32` | `cargo run -p ch03-functions --example 11-impl-fn` |
| [`12-methods`](examples/12-methods.rs) | Methods in `impl`: `&self` reads, `&mut self` changes | `cargo run -p ch03-functions --example 12-methods` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: `stats` returns (min, max, avg) as a tuple | `cargo run -p ch03-functions --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: a discount closure applied with `.map()` | `cargo run -p ch03-functions --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `move` with a number breaks nothing, with a `String` it does | `cargo run -p ch03-functions --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: a call counter inside a closure (FnMut) | `cargo run -p ch03-functions --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `greet` with an optional greeting via `Option<&str>` | `cargo run -p ch03-functions --example exercise-5` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 03
rustc --edition 2024 chapters/03-functions/broken/07-use-after-move.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`07-use-after-move.rs`](broken/07-use-after-move.rs) | `move` takes the `String` into the closure: the variable is gone afterwards | error[E0382]: borrow of moved value: `name` |
| [`08-closure-not-mut.rs`](broken/08-closure-not-mut.rs) | A closure that changes an outer variable must itself be `let mut` | error[E0596]: cannot borrow `inc` as mutable, as it is not declared as mutable |
| [`exercise-3.rs`](broken/exercise-3.rs) | Exercise 3 (broken version): a `String` after a `move` closure | error[E0382]: borrow of moved value: `label` |
| [`exercise-4.rs`](broken/exercise-4.rs) | Exercise 4 (broken version): an FnMut closure without `mut` | error[E0596]: cannot borrow `apply` as mutable, as it is not declared as mutable |
