# 5. Enum, Option and saying goodbye to null

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch05-enums --example 01-enum
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch05-enums --example 01-enum
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-enum`](examples/01-enum.rs) | A simple enum — like a PHP 8.1 enum without values | `cargo run -p ch05-enums --example 01-enum` |
| [`02-enum-with-data`](examples/02-enum-with-data.rs) | Every enum variant carries its own data | `cargo run -p ch05-enums --example 02-enum-with-data` |
| [`03-match-enum`](examples/03-match-enum.rs) | `match` on an enum: data is pulled out right in the arm, `..` means "the other fields" | `cargo run -p ch05-enums --example 03-match-enum` |
| [`04-matches-macro`](examples/04-matches-macro.rs) | Enum methods and the `matches!` macro | `cargo run -p ch05-enums --example 04-matches-macro` |
| [`05-enum-eq`](examples/05-enum-eq.rs) | `==` (needs `PartialEq`, compares the data too) vs `matches!` (shape only) | `cargo run -p ch05-enums --example 05-enum-eq` |
| [`06-enum-string-roundtrip`](examples/06-enum-string-roundtrip.rs) | An enum to a string and back — like `->value` and `::tryFrom()` | `cargo run -p ch05-enums --example 06-enum-string-roundtrip` |
| [`07-option`](examples/07-option.rs) | `Option` instead of null: `match` and `if let` | `cargo run -p ch05-enums --example 07-option` |
| [`08-option-methods`](examples/08-option-methods.rs) | Everyday Option methods: is_some, map, unwrap_or, unwrap, expect, and_then | `cargo run -p ch05-enums --example 08-option-methods` |
| [`09-unwrap-or-lazy`](examples/09-unwrap-or-lazy.rs) | `unwrap_or` always computes the default, `unwrap_or_else` only on None | `cargo run -p ch05-enums --example 09-unwrap-or-lazy` |
| [`10-option-in-struct`](examples/10-option-in-struct.rs) | `Option` in fields — like nullable columns | `cargo run -p ch05-enums --example 10-option-in-struct` |
| [`11-option-size`](examples/11-option-size.rs) | The size of Option: for references and Box the wrapper is free | `cargo run -p ch05-enums --example 11-option-size` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: a `PaymentMethod` enum with data in its variants | `cargo run -p ch05-enums --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: a fee per payment method with `match` | `cargo run -p ch05-enums --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `find_discount` returns an `Option<f64>` | `cargo run -p ch05-enums --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: print the discount with `match`, then with `.map()` + `.unwrap_or()` | `cargo run -p ch05-enums --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `is_cashless` with `matches!` | `cargo run -p ch05-enums --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: `code()` and the new `Invoice` variant | `cargo run -p ch05-enums --example exercise-6` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 05
rustc --edition 2024 chapters/05-enums/broken/03-new-variant.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`03-new-variant.rs`](broken/03-new-variant.rs) | A new enum variant — and every `match` without it stops compiling | error[E0004]: non-exhaustive patterns: `&OrderStatus::Refunded` not covered |
| [`05-enum-eq-without-partialeq.rs`](broken/05-enum-eq-without-partialeq.rs) | `==` for an enum without `PartialEq` | error[E0369]: binary operation `==` cannot be applied to type `Status` |
| [`07-option-field.rs`](broken/07-option-field.rs) | You can't reach a field inside an `Option` without checking | error[E0609]: no field `name` on type `Option<User>` |
| [`exercise-5.rs`](broken/exercise-5.rs) | Exercise 5 (broken version): `!=` for an enum without `PartialEq` | error[E0369]: binary operation `!=` cannot be applied to type `PaymentMethod` |
| [`exercise-6.rs`](broken/exercise-6.rs) | Exercise 6 (broken version): `Invoice` added, but not the `match` arm in `code()` | error[E0004]: non-exhaustive patterns: `&PaymentMethod::Invoice { .. }` not covered |
