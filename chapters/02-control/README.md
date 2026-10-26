# 2. Control flow

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch02-control --example 01-if-expression
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch02-control --example 01-if-expression
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-if-expression`](examples/01-if-expression.rs) | `if` is an expression, so there's no need for a ternary operator | `cargo run -p ch02-control --example 01-if-expression` |
| [`02-if-bool`](examples/02-if-bool.rs) | The condition is an explicit comparison that yields a `bool` | `cargo run -p ch02-control --example 02-if-bool` |
| [`03-loop`](examples/03-loop.rs) | `loop` — an infinite loop until `break` (in PHP: `while (true)`) | `cargo run -p ch02-control --example 03-loop` |
| [`04-loop-break-value`](examples/04-loop-break-value.rs) | `break value` — the result of the whole `loop` | `cargo run -p ch02-control --example 04-loop-break-value` |
| [`05-while`](examples/05-while.rs) | `while` — like in PHP, just without parentheses around the condition | `cargo run -p ch02-control --example 05-while` |
| [`06-for`](examples/06-for.rs) | `for` over ranges and collections, `&users`, `.iter().enumerate()` | `cargo run -p ch02-control --example 06-for` |
| [`07-clippy-loops`](examples/07-clippy-loops.rs) | What `cargo clippy` says about "PHP-style" loops — and how to rewrite them | `cargo run -p ch02-control --example 07-clippy-loops` |
| [`08-labels`](examples/08-labels.rs) | Loop labels: `break 'outer` is PHP's `break 2` | `cargo run -p ch02-control --example 08-labels` |
| [`09-match`](examples/09-match.rs) | `match` — a `switch` that grew up (and returns a value) | `cargo run -p ch02-control --example 09-match` |
| [`10-match-ranges`](examples/10-match-ranges.rs) | `match` with ranges and a guard | `cargo run -p ch02-control --example 10-match-ranges` |
| [`11-exhaustive`](examples/11-exhaustive.rs) | An exhaustive `match` on bool: both arms, no `_` needed | `cargo run -p ch02-control --example 11-exhaustive` |
| [`12-unreachable-pattern`](examples/12-unreachable-pattern.rs) | A `_` at the top swallows everything: the arms below are unreachable | `cargo run -p ch02-control --example 12-unreachable-pattern` |
| [`13-match-tuple`](examples/13-match-tuple.rs) | Destructuring a tuple right in a `match` arm | `cargo run -p ch02-control --example 13-match-tuple` |
| [`14-if-let`](examples/14-if-let.rs) | `if let` — when you care about one arm | `cargo run -p ch02-control --example 14-if-let` |
| [`15-let-else`](examples/15-let-else.rs) | `let else` — return early if the pattern doesn't match | `cargo run -p ch02-control --example 15-let-else` |
| [`16-while-let`](examples/16-while-let.rs) | `while let` — loop while the pattern matches | `cargo run -p ch02-control --example 16-while-let` |
| [`17-let-chains`](examples/17-let-chains.rs) | `if let ... && condition` — let chains (edition 2024, Rust 1.88+) | `cargo run -p ch02-control --example 17-let-chains` |
| [`18-shadowing`](examples/18-shadowing.rs) | Shadowing: a new variable with the same name (and possibly another type) | `cargo run -p ch02-control --example 18-shadowing` |
| [`19-block-shadowing`](examples/19-block-shadowing.rs) | A trap: shadowing inside a block doesn't change the outer variable | `cargo run -p ch02-control --example 19-block-shadowing` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: `http_class` with a `match` on ranges | `cargo run -p ch02-control --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: exhaustiveness without `_` — list the ranges explicitly | `cargo run -p ch02-control --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: numbers in 0..=100 divisible by 3 but not by 5 (`continue`) | `cargo run -p ch02-control --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: `_` at the top of a `match` — the function always returns "invalid" | `cargo run -p ch02-control --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: how many doublings until 1 exceeds 1000 (`break value`) | `cargo run -p ch02-control --example exercise-5` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 02
rustc --edition 2024 chapters/02-control/broken/02-if-not-bool.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`02-if-not-bool.rs`](broken/02-if-not-bool.rs) | An `if` condition must be a `bool`: numbers have no "truthiness" | error[E0308]: mismatched types — expected `bool`, found integer |
| [`11-non-exhaustive.rs`](broken/11-non-exhaustive.rs) | `match` must be exhaustive: a forgotten arm is a compile error | error[E0004]: non-exhaustive patterns: `false` not covered |
| [`exercise-2.rs`](broken/exercise-2.rs) | Exercise 2: without the `_` arm a match on u16 is not exhaustive | error[E0004]: non-exhaustive patterns: |
