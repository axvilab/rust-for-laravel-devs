# 17. Tests, macros, and what's next

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch17-tests --example 01-macro-rules
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch17-tests --example 01-macro-rules
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-macro-rules`](examples/01-macro-rules.rs) | A declarative macro `hashmap!`: `$k:expr`, the repetition `$( ... ),*`, a trailing comma `$(,)?` | `cargo run -p ch17-tests --example 01-macro-rules` |
| [`02-serde`](examples/02-serde.rs) | Serde: `from_str` / `to_string` and the default, rename, skip_serializing attributes | `cargo run -p ch17-tests --example 02-serde` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: a test that fails on purpose, with an explanation in `assert_eq!` | `cargo run -p ch17-tests --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: `println!` in a passing test — visible only with `--nocapture` | `cargo run -p ch17-tests --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `#[sqlx::test]` for `OrderRepo` — save, pay, save, read back | `cargo run -p ch17-tests --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: your own `macro_rules! hashset` modelled on `hashmap!` | `cargo run -p ch17-tests --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: what `#[derive(Debug, Clone)]` expands to (as `cargo expand` would show) | `cargo run -p ch17-tests --example exercise-5` |

## The crate itself

```bash
cargo test -p ch17-tests                     # unit, integration and doc tests
```

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 17
rustc --edition 2024 chapters/17-tests/broken/01-async-test-without-runtime.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`01-async-test-without-runtime.rs`](broken/01-async-test-without-runtime.rs) | `#[test]` on an `async fn` — the test runner can't wait for a future | error: async functions cannot be used for tests |
