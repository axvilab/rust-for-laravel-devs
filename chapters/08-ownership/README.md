# 8. Ownership — one owner for every thing

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch08-ownership --example 01-scope
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch08-ownership --example 01-scope
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-scope`](examples/01-scope.rs) | Scope = the life of a value: the string is freed at the end of the block | `cargo run -p ch08-ownership --example 01-scope` |
| [`02-move`](examples/02-move.rs) | Move: after `let b = a` the owner of the string is `b` | `cargo run -p ch08-ownership --example 02-move` |
| [`03-move-into-function`](examples/03-move-into-function.rs) | A function with a `String` parameter becomes the owner and destroys the string at the end | `cargo run -p ch08-ownership --example 03-move-into-function` |
| [`04-move-in-loop-fixed`](examples/04-move-in-loop-fixed.rs) | Two fixes for E0382 in a loop: borrow (better) or clone | `cargo run -p ch08-ownership --example 04-move-in-loop-fixed` |
| [`05-give-back`](examples/05-give-back.rs) | Option A: give ownership away and get it back (works, but nobody writes this) | `cargo run -p ch08-ownership --example 05-give-back` |
| [`06-copy-types`](examples/06-copy-types.rs) | Copy: numbers are copied, not moved | `cargo run -p ch08-ownership --example 06-copy-types` |
| [`07-copy-struct`](examples/07-copy-struct.rs) | `#[derive(Clone, Copy)]` for a small "numeric" struct | `cargo run -p ch08-ownership --example 07-copy-struct` |
| [`08-copy-is-a-copy`](examples/08-copy-is-a-copy.rs) | A Copy struct is a separate copy, not "the same object" as in PHP | `cargo run -p ch08-ownership --example 08-copy-is-a-copy` |
| [`09-clone`](examples/09-clone.rs) | Clone — an explicit (and deep) copy | `cargo run -p ch08-ownership --example 09-clone` |
| [`10-drop`](examples/10-drop.rs) | `impl Drop` — a deterministic `__destruct()`; `drop(x)` — destroy earlier | `cargo run -p ch08-ownership --example 10-drop` |
| [`11-let-underscore`](examples/11-let-underscore.rs) | `let _ =` destroys immediately, `let _b =` — at the end of the block | `cargo run -p ch08-ownership --example 11-let-underscore` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: `let t = s;` — and `s` is no longer available | `cargo run -p ch08-ownership --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: no error with a number — `i32` implements `Copy` | `cargo run -p ch08-ownership --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `shout(String)` takes the string — two ways to fix it | `cargo run -p ch08-ownership --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: drop order is the reverse of declaration order | `cargo run -p ch08-ownership --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: with a `String` inside — only `Clone`, the copy is explicit | `cargo run -p ch08-ownership --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: a string into a function in a loop — both fixes in turn | `cargo run -p ch08-ownership --example exercise-6` |
| [`exercise-7`](examples/exercise-7.rs) | Exercise 7: one variable became `_` — its value is destroyed immediately | `cargo run -p ch08-ownership --example exercise-7` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 08
rustc --edition 2024 chapters/08-ownership/broken/01-out-of-scope.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`01-out-of-scope.rs`](broken/01-out-of-scope.rs) | A variable doesn't exist outside its block | error[E0425]: cannot find value `name` in this scope |
| [`02-move.rs`](broken/02-move.rs) | `let b = a` for a String is a move: `a` can't be used any more | error[E0382]: borrow of moved value: `a` |
| [`03-move-into-function.rs`](broken/03-move-into-function.rs) | Passed a `String` to a function — gave it away | error[E0382]: borrow of moved value: `n` |
| [`04-move-in-loop.rs`](broken/04-move-in-loop.rs) | A move in a loop: "value moved here, in previous iteration of loop" | error[E0382]: use of moved value: `email` |
| [`exercise-1.rs`](broken/exercise-1.rs) | Exercise 1 (broken version): use after move | error[E0382]: borrow of moved value: `s` |
| [`exercise-3.rs`](broken/exercise-3.rs) | Exercise 3 (broken version): the string is used after being passed to `shout` | error[E0382]: borrow of moved value: `s` |
| [`exercise-5.rs`](broken/exercise-5.rs) | Exercise 5: `Copy` for a struct with a `String` field | error[E0204]: the trait `Copy` cannot be implemented for this type |
| [`exercise-6.rs`](broken/exercise-6.rs) | Exercise 6 (broken version): the same String three times into a function that takes String | error[E0382]: use of moved value: `email` |
