# 4. Structs — classes without classes

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch04-structs --example 01-struct-literal
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch04-structs --example 01-struct-literal
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-struct-literal`](examples/01-struct-literal.rs) | Declaring a struct and building it with a literal with named fields | `cargo run -p ch04-structs --example 01-struct-literal` |
| [`02-field-init-shorthand`](examples/02-field-init-shorthand.rs) | The shorthand `name` instead of `name: name` | `cargo run -p ch04-structs --example 02-field-init-shorthand` |
| [`03-string-conversions`](examples/03-string-conversions.rs) | Four ways to make a `String` from a `&str` | `cargo run -p ch04-structs --example 03-string-conversions` |
| [`05-impl-methods`](examples/05-impl-methods.rs) | An `impl` block: a `new` constructor, methods with `&self` and `&mut self` | `cargo run -p ch04-structs --example 05-impl-methods` |
| [`06-self-and-self-type`](examples/06-self-and-self-type.rs) | `self` is the instance (`$this`), `Self` is the type (`self::`/`static::` in PHP) | `cargo run -p ch04-structs --example 06-self-and-self-type` |
| [`07-attributes-cfg`](examples/07-attributes-cfg.rs) | Attributes: `#![allow]` for the whole file and `#[cfg]` — conditional compilation | `cargo run -p ch04-structs --example 07-attributes-cfg` |
| [`08-derive-debug`](examples/08-derive-debug.rs) | `#[derive(Debug)]` — the compiler writes the `Debug` implementation for you | `cargo run -p ch04-structs --example 08-derive-debug` |
| [`09-derive-list`](examples/09-derive-list.rs) | `#[derive(Debug, Clone, PartialEq, Default)]` in action | `cargo run -p ch04-structs --example 09-derive-list` |
| [`10-partial-eq`](examples/10-partial-eq.rs) | Why there's `PartialEq` but no `Eq` for `f64`: NaN isn't equal to itself | `cargo run -p ch04-structs --example 10-partial-eq` |
| [`11-derive-default`](examples/11-derive-default.rs) | `derive(Default)` and `..Default::default()` / `..another_instance` | `cargo run -p ch04-structs --example 11-derive-default` |
| [`12-manual-default`](examples/12-manual-default.rs) | A hand-written `impl Default` when "zero" values don't fit | `cargo run -p ch04-structs --example 12-manual-default` |
| [`13-struct-update-moves`](examples/13-struct-update-moves.rs) | `..dev` moves fields: `u16` is copied, `String` moves | `cargo run -p ch04-structs --example 13-struct-update-moves` |
| [`14-visibility`](examples/14-visibility.rs) | `pub` and private fields: from outside the module — only through the constructor | `cargo run -p ch04-structs --example 14-visibility` |
| [`15-tuple-and-unit-structs`](examples/15-tuple-and-unit-structs.rs) | Tuple structs as wrappers and unit structs as markers | `cargo run -p ch04-structs --example 15-tuple-and-unit-structs` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: a `Product` struct with `#[derive(Debug)]` | `cargo run -p ch04-structs --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: `new`, `is_available`, `sell` with `saturating_sub` | `cargo run -p ch04-structs --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `sell` requires `let mut product` | `cargo run -p ch04-structs --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: printing a product with `{:#?}` | `cargo run -p ch04-structs --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: what the compiler says without `Debug` — and a hand-written `impl Debug` | `cargo run -p ch04-structs --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: `PartialEq` works, `Eq` doesn't (the `price: f64` field) | `cargo run -p ch04-structs --example exercise-6` |
| [`exercise-7`](examples/exercise-7.rs) | Exercise 7: a hand-written `impl Default` and `..Default::default()` | `cargo run -p ch04-structs --example exercise-7` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 04
rustc --edition 2024 chapters/04-structs/broken/03-str-field.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`03-str-field.rs`](broken/03-str-field.rs) | A `&str` field in a struct without a lifetime does not compile | error[E0106]: missing lifetime specifier |
| [`04-into-without-type.rs`](broken/04-into-without-type.rs) | `.into()` without a known target type | error[E0283]: type annotations needed |
| [`08-no-debug.rs`](broken/08-no-debug.rs) | `{:?}` for a struct without `#[derive(Debug)]` | error[E0277]: `User` doesn't implement `Debug` |
| [`10-eq-on-f64.rs`](broken/10-eq-on-f64.rs) | `#[derive(Eq)]` on a struct with an `f64` | error[E0277]: the trait bound `f64: Eq` is not satisfied |
| [`13-use-moved-field.rs`](broken/13-use-moved-field.rs) | The `String` field moved out via `..dev` — `dev.host` can't be used any more | error[E0382]: borrow of moved value: `dev.host` |
| [`14-private-field.rs`](broken/14-private-field.rs) | A struct with a private field can't be built with a literal outside its module | error[E0451]: field `password_hash` of struct `User` is private |
| [`exercise-3.rs`](broken/exercise-3.rs) | Exercise 3 (broken version): a `&mut self` method on an immutable variable | error[E0596]: cannot borrow `product` as mutable, as it is not declared as mutable |
| [`exercise-5.rs`](broken/exercise-5.rs) | Exercise 5: without `Debug` in `derive`, printing with `{:?}` doesn't compile | error[E0277]: `Product` doesn't implement `Debug` |
| [`exercise-6.rs`](broken/exercise-6.rs) | Exercise 6 (broken version): `Eq` on a struct with an `f64` | error[E0277]: the trait bound `f64: Eq` is not satisfied |
