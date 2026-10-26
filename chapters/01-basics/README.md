# 1. Hello, world and basic syntax

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch01-basics --example 01-hello
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch01-basics --example 01-hello
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-hello`](examples/01-hello.rs) | The first program: `fn main` and the `println!` macro | `cargo run -p ch01-basics --example 01-hello` |
| [`02-mut`](examples/02-mut.rs) | `let mut` — a variable that can change | `cargo run -p ch01-basics --example 02-mut` |
| [`03-warnings`](examples/03-warnings.rs) | Compiler warnings: an unused variable and the `_` prefix | `cargo run -p ch01-basics --example 03-warnings` |
| [`04-const`](examples/04-const.rs) | `const` — a real constant, the type is mandatory | `cargo run -p ch01-basics --example 04-const` |
| [`05-types`](examples/05-types.rs) | Type inference and explicit annotations | `cargo run -p ch01-basics --example 05-types` |
| [`06-as-cast`](examples/06-as-cast.rs) | `as` — explicit numeric conversion | `cargo run -p ch01-basics --example 06-as-cast` |
| [`07-overflow`](examples/07-overflow.rs) | Overflow: `checked_*`, `saturating_*`, `wrapping_*` and the silent `as` | `cargo run -p ch01-basics --example 07-overflow` |
| [`08-strings`](examples/08-strings.rs) | `&str` and `String`: a literal, an owned string, `format!` | `cargo run -p ch01-basics --example 08-strings` |
| [`09-utf8-len`](examples/09-utf8-len.rs) | String length: bytes (`len`) vs characters (`chars().count()`) | `cargo run -p ch01-basics --example 09-utf8-len` |
| [`10-format`](examples/10-format.rs) | Formatting: positional, named, `{:?}`, `{:#?}`, alignment, precision | `cargo run -p ch01-basics --example 10-format` |
| [`11-dbg`](examples/11-dbg.rs) | `println!` (stdout), `eprintln!` (stderr) and `dbg!` — the real `dd()` | `cargo run -p ch01-basics --example 11-dbg` |
| [`12-block-expression`](examples/12-block-expression.rs) | A block `{ ... }` is an expression: the value of the last line without `;` | `cargo run -p ch01-basics --example 12-block-expression` |
| [`13-return`](examples/13-return.rs) | A function's value is its last expression; `return` works but isn't idiomatic | `cargo run -p ch01-basics --example 13-return` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: price (f64) × quantity (u32) — needs an explicit conversion | `cargo run -p ch01-basics --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: two decimal places with `{:.2}` | `cargo run -p ch01-basics --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: without `mut`, reassignment doesn't compile | `cargo run -p ch01-basics --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: `dbg!` in the middle of an expression, and the output order | `cargo run -p ch01-basics --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `checked_add` and `saturating_add` for u8 | `cargo run -p ch01-basics --example exercise-5` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 01
rustc --edition 2024 chapters/01-basics/broken/02-immutable.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`02-immutable.rs`](broken/02-immutable.rs) | Variables are immutable by default: reassignment does not compile | error[E0384]: cannot assign twice to immutable variable `count` |
| [`06-no-implicit-conversion.rs`](broken/06-no-implicit-conversion.rs) | No implicit conversions: i32 + f64 does not compile | error[E0277]: cannot add `f64` to `i32` |
| [`13-extra-semicolon.rs`](broken/13-extra-semicolon.rs) | An extra `;` after the last expression: the function returns `()` instead of i32 | error[E0308]: mismatched types — expected `i32`, found `()` |
| [`exercise-3.rs`](broken/exercise-3.rs) | Exercise 3 (broken version): assigning to an immutable variable | error[E0384]: cannot assign twice to immutable variable `price` |
| [`exercise-5.rs`](broken/exercise-5.rs) | Exercise 5 (broken version): an overflow known at compile time | error: this arithmetic operation will overflow |
