# 7. Traits — interfaces that can do more

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch07-traits --example 01-trait-as-interface
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch07-traits --example 01-trait-as-interface
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-trait-as-interface`](examples/01-trait-as-interface.rs) | A trait is an interface; the implementation is a separate `impl Trait for Type` block | `cargo run -p ch07-traits --example 01-trait-as-interface` |
| [`02-default-methods`](examples/02-default-methods.rs) | A default implementation: a type overrides only what it wants | `cargo run -p ch07-traits --example 02-default-methods` |
| [`03-impl-generic-dyn`](examples/03-impl-generic-dyn.rs) | Three parameter forms: `&impl Trait`, `<T: Trait>`, `&dyn Trait` | `cargo run -p ch07-traits --example 03-impl-generic-dyn` |
| [`04-vec-box-dyn`](examples/04-vec-box-dyn.rs) | A list of objects of different types — `Vec<Box<dyn Trait>>` | `cargo run -p ch07-traits --example 04-vec-box-dyn` |
| [`05-return-box-dyn`](examples/05-return-box-dyn.rs) | "Either this or that" from a function — `Box<dyn Trait>` | `cargo run -p ch07-traits --example 05-return-box-dyn` |
| [`06-display`](examples/06-display.rs) | `Display` is written by hand — a human-readable format for `{}` | `cargo run -p ch07-traits --example 06-display` |
| [`07-display-gives-to-string`](examples/07-display-gives-to-string.rs) | Implement `Display` — get `.to_string()` and `format!` for free | `cargo run -p ch07-traits --example 07-display-gives-to-string` |
| [`08-from-into`](examples/08-from-into.rs) | `From` — "my type can be made from this one"; `Into` comes free the other way round | `cargo run -p ch07-traits --example 08-from-into` |
| [`09-orphan-rule`](examples/09-orphan-rule.rs) | Working around the orphan rule: your own wrapper type around `Vec<String>` | `cargo run -p ch07-traits --example 09-orphan-rule` |
| [`10-trait-in-scope`](examples/10-trait-in-scope.rs) | `use std::fmt::Write` — without importing the trait, `write!` on `String` isn't found | `cargo run -p ch07-traits --example 10-trait-in-scope` |
| [`11-operator-add`](examples/11-operator-add.rs) | Operators are traits too: `impl Add for Money` | `cargo run -p ch07-traits --example 11-operator-add` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: a `Priced` trait with a default `price_with_vat` | `cargo run -p ch07-traits --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: `Priced` for a product and a service (price = rate × hours) | `cargo run -p ch07-traits --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `total` over a `Box<dyn Priced>` list of mixed products and services | `cargo run -p ch07-traits --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: `Display` for `Product`: "Laptop — 45000.00 UAH" | `cargo run -p ch07-traits --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `#[derive(Debug, Clone, PartialEq)]` and `product == product.clone()` | `cargo run -p ch07-traits --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: `to_string()` without any new code | `cargo run -p ch07-traits --example exercise-6` |
| [`exercise-7`](examples/exercise-7.rs) | Exercise 7: `From<f64> for Product`; `.into()` needs a type annotation | `cargo run -p ch07-traits --example exercise-7` |

## The crate itself

```bash
cargo test -p ch07-traits                     # unit, integration and doc tests
```

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 07
rustc --edition 2024 chapters/07-traits/broken/05-impl-trait-two-types.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`05-impl-trait-two-types.rs`](broken/05-impl-trait-two-types.rs) | `-> impl Trait` is ONE hidden type, not "any of several" | error[E0308]: `if` and `else` have incompatible types |
| [`08-into-needs-type.rs`](broken/08-into-needs-type.rs) | `.into()` without an annotation: the compiler doesn't know what to convert into | error[E0283]: type annotations needed |
| [`09-orphan-rule.rs`](broken/09-orphan-rule.rs) | The orphan rule: you can't implement a foreign trait for a foreign type | error[E0117]: only traits defined in the current crate can be |
| [`10-trait-not-in-scope.rs`](broken/10-trait-not-in-scope.rs) | A trait method isn't visible without `use` | error[E0599]: cannot write into `String` |
| [`11-use-after-add.rs`](broken/11-use-after-add.rs) | `+` takes its operands (`self`, not `&self`): you can't use them after the addition | error[E0382]: borrow of moved value: `price` |
| [`exercise-7.rs`](broken/exercise-7.rs) | Exercise 7 (broken version): `.into()` without a type annotation | error[E0283]: type annotations needed |
