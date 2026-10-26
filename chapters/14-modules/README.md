# 14. Modules, crates and Cargo — how to organize a project

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch14-modules --example 01-modules
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch14-modules --example 01-modules
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-modules`](examples/01-modules/) | Modules as in the book: `mod` in main.rs, files models.rs, services.rs, services/*.rs | `cargo run -p ch14-modules --example 01-modules` |
| [`02-paths-and-visibility`](examples/02-paths-and-visibility.rs) | Paths `crate::`, `super::`, `self::` and visibility levels `pub`, `pub(crate)`, `pub(super)` | `cargo run -p ch14-modules --example 02-paths-and-visibility` |
| [`03-reexport`](examples/03-reexport.rs) | `pub use` — a re-export that hides the internal module structure | `cargo run -p ch14-modules --example 03-reexport` |
| [`04-use-library-crate`](examples/04-use-library-crate.rs) | The example uses this package's own library from outside: a hyphen in the name → an underscore | `cargo run -p ch14-modules --example 04-use-library-crate` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: a "shop" split into modules — models, pricing, errors (see this crate's src/) | `cargo run -p ch14-modules --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: without `pub` a function is private to its module | `cargo run -p ch14-modules --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `cargo add thiserror anyhow`, `cargo tree` — who pulls in `syn` | `cargo run -p ch14-modules --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: what `cargo clippy` suggests most often (already fixed) and `cargo fmt` | `cargo run -p ch14-modules --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `calc_fee` in lib.rs with a doc example — and why it fails without `use` | `cargo run -p ch14-modules --example exercise-5` |

## The crate itself

```bash
cargo run -p ch14-modules                      # src/main.rs
cargo run -p ch14-modules --bin report         # src/bin/report.rs
cargo test -p ch14-modules                     # unit, integration and doc tests
```

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 14
rustc --edition 2024 chapters/14-modules/broken/02-private-function.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`02-private-function.rs`](broken/02-private-function.rs) | Calling a private function from another module | error[E0603]: function `format_money` is private |
| [`03-mod-for-dependency.rs`](broken/03-mod-for-dependency.rs) | `mod serde;` for a dependency from Cargo.toml — that's not how it's done | error[E0583]: file not found for module `serde` |
