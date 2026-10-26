# 10. Lifetimes — briefly and without panic

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch10-lifetimes --example 01-longer
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch10-lifetimes --example 01-longer
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-longer`](examples/01-longer.rs) | `'a` describes a relationship: the result lives no longer than the shorter argument | `cargo run -p ch10-lifetimes --example 01-longer` |
| [`02-elision`](examples/02-elision.rs) | Elision rules: one reference parameter or `&self` — no lifetimes needed | `cargo run -p ch10-lifetimes --example 02-elision` |
| [`03-return-owned`](examples/03-return-owned.rs) | Created a string in the function — return `String`, not `&str` | `cargo run -p ch10-lifetimes --example 03-return-owned` |
| [`04-struct-with-ref`](examples/04-struct-with-ref.rs) | A struct with a reference needs a lifetime and can't outlive its source | `cargo run -p ch10-lifetimes --example 04-struct-with-ref` |
| [`05-static`](examples/05-static.rs) | `'static` — a reference valid until the end of the program (string literals) | `cargo run -p ch10-lifetimes --example 05-static` |
| [`06-static-bound`](examples/06-static-bound.rs) | `T: 'static` is satisfied by owned types: String, Vec, numbers, structs with String | `cargo run -p ch10-lifetimes --example 06-static-bound` |
| [`07-anonymous-lifetime`](examples/07-anonymous-lifetime.rs) | `'_` — the anonymous lifetime: "something is borrowed here, details don't matter" | `cargo run -p ch10-lifetimes --example 07-anonymous-lifetime` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: `pick` with two literals, then with a literal and a `&String` | `cargo run -p ch10-lifetimes --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: what the compiler suggests if you remove the lifetimes | `cargo run -p ch10-lifetimes --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `Excerpt<'a>` works as long as the source lives longer | `cargo run -p ch10-lifetimes --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: `Excerpt` with a `String` — the lifetime disappears | `cargo run -p ch10-lifetimes --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `make_greeting` returns a `String` | `cargo run -p ch10-lifetimes --example exercise-5` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 10
rustc --edition 2024 chapters/10-lifetimes/broken/01-missing-lifetime.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`01-missing-lifetime.rs`](broken/01-missing-lifetime.rs) | Two references in, a reference out — the compiler asks for a lifetime | error[E0106]: missing lifetime specifier |
| [`03-return-ref-to-local.rs`](broken/03-return-ref-to-local.rs) | A lifetime won't save a reference to a local variable | error[E0515]: cannot return reference to local variable `s` |
| [`06-static-bound.rs`](broken/06-static-bound.rs) | `T: 'static` means "holds no borrowed references", not "lives forever" | error[E0597]: `local` does not live long enough |
| [`exercise-2.rs`](broken/exercise-2.rs) | Exercise 2: `pick` without lifetimes | error[E0106]: missing lifetime specifier |
| [`exercise-3.rs`](broken/exercise-3.rs) | Exercise 3: an `Excerpt` referring to a String destroyed at the end of a block | error[E0597]: `text` does not live long enough |
| [`exercise-5.rs`](broken/exercise-5.rs) | Exercise 5: returning `&format!(...)` from a function | error[E0515]: cannot return reference to temporary value |
