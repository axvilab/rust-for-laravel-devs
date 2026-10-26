# 6. How the language is built: the mental model

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch06-concept --example 01-data-behavior-contract
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch06-concept --example 01-data-behavior-contract
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-data-behavior-contract`](examples/01-data-behavior-contract.rs) | Data (`struct`), behavior (`impl`) and contract (`trait`) — three separate blocks | `cargo run -p ch06-concept --example 01-data-behavior-contract` |
| [`02-composition`](examples/02-composition.rs) | Composition instead of inheritance: the shared part is a separate struct | `cargo run -p ch06-concept --example 02-composition` |
| [`03-trait-default-method`](examples/03-trait-default-method.rs) | A trait with a default implementation — a PHP trait with type checking | `cargo run -p ch06-concept --example 03-trait-default-method` |
| [`04-generic-function`](examples/04-generic-function.rs) | A generic function `send<T: Mailable>` — like `function send(Mailable $item)` | `cargo run -p ch06-concept --example 04-generic-function` |
| [`05-no-constructors`](examples/05-no-constructors.rs) | No constructors: a literal with all fields, and `new` is just a function | `cargo run -p ch06-concept --example 05-no-constructors` |
| [`06-extend-foreign-type`](examples/06-extend-foreign-type.rs) | Your own trait for someone else's type: the method shows up even on `String` (like `Collection::macro`) | `cargo run -p ch06-concept --example 06-extend-foreign-type` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 06
rustc --edition 2024 chapters/06-concept/broken/05-missing-field.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`05-missing-field.rs`](broken/05-missing-field.rs) | An "uninitialized field" is impossible: a literal must list every field | error[E0063]: missing field `paid` in initializer of `Invoice` |
