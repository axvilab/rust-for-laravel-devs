# 12. Error handling — Result instead of exceptions

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch12-errors --example 01-result
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch12-errors --example 01-result
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-result`](examples/01-result.rs) | `Result<T, E>`: a function that may fail, and a `match` on the result | `cargo run -p ch12-errors --example 01-result` |
| [`02-question-mark`](examples/02-question-mark.rs) | The `?` operator: on `Err` return it from the function, on `Ok` keep going | `cargo run -p ch12-errors --example 02-question-mark` |
| [`03-question-mark-desugared`](examples/03-question-mark-desugared.rs) | What `?` does for you: the same code with `match` | `cargo run -p ch12-errors --example 03-question-mark-desugared` |
| [`04-main-result`](examples/04-main-result.rs) | `main` returns a `Result`: the error is printed with `{:?}`, exit code 1 | `cargo run -p ch12-errors --example 04-main-result` |
| [`05-box-dyn-error`](examples/05-box-dyn-error.rs) | `Result<(), Box<dyn Error>>` — "any error in a box" for main | `cargo run -p ch12-errors --example 05-box-dyn-error` |
| [`06-error-enum`](examples/06-error-enum.rs) | Your own error enum: a handler does a `match` and picks the HTTP status | `cargo run -p ch12-errors --example 06-error-enum` |
| [`07-from-for-question`](examples/07-from-for-question.rs) | `impl From<io::Error> for AppError` — and `?` converts errors by itself | `cargo run -p ch12-errors --example 07-from-for-question` |
| [`08-thiserror`](examples/08-thiserror.rs) | thiserror: `#[error]` generates Display, `#[from]` — impl From | `cargo run -p ch12-errors --example 08-thiserror` |
| [`09-anyhow-context`](examples/09-anyhow-context.rs) | anyhow + `.context()`: a human-readable chain of causes in `main` | `cargo run -p ch12-errors --example 09-anyhow-context` |
| [`10-panics`](examples/10-panics.rs) | Panics are for bugs: expect, out-of-bounds index, unwrap, assert!, unreachable!, todo! | `cargo run -p ch12-errors --example 10-panics` |
| [`11-result-methods`](examples/11-result-methods.rs) | Result methods: unwrap_or, unwrap_or_else, map, map_err, ok, and_then, ok_or | `cargo run -p ch12-errors --example 11-result-methods` |
| [`12-unwrap-or-is-eager`](examples/12-unwrap-or-is-eager.rs) | `unwrap_or` always computes the default, `unwrap_or_else` only on `Err` | `cargo run -p ch12-errors --example 12-unwrap-or-is-eager` |
| [`13-collect-result`](examples/13-collect-result.rs) | `collect()` into `Result<Vec<_>, _>` stops at the first error; `partition` collects everything | `cargo run -p ch12-errors --example 13-collect-result` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: `parse_age` — not a number or > 150 → `Err` | `cargo run -p ch12-errors --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: `parse_user` propagates the `parse_age` error upward with `?` | `cargo run -p ch12-errors --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `ValidationError` + `impl From<String>` so that `?` on `parse_age` works | `cargo run -p ch12-errors --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: the same `ValidationError` with thiserror; printed with `{}` | `cargo run -p ch12-errors --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `anyhow` + `.context()` around all the processing in `main` | `cargo run -p ch12-errors --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: `parse_ids` — `split`, `map`, `collect()` into `Result<Vec<_>, _>` | `cargo run -p ch12-errors --example exercise-6` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 12
rustc --edition 2024 chapters/12-errors/broken/04-question-in-unit-main.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`04-question-in-unit-main.rs`](broken/04-question-in-unit-main.rs) | `?` in a plain `main` that returns nothing | error[E0277]: the `?` operator can only be used in a function |
