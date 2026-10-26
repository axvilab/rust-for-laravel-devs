# 16. Async — what web servers run on

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch16-async --example 01-async-await
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch16-async --example 01-async-await
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-async-await`](examples/01-async-await.rs) | `async fn` returns a Future; `.await` waits; `.await?` — wait and propagate the error | `cargo run -p ch16-async --example 01-async-await` |
| [`02-tokio-main`](examples/02-tokio-main.rs) | `#[tokio::main]` — creates the runtime and runs the async main | `cargo run -p ch16-async --example 02-tokio-main` |
| [`03-futures-are-lazy`](examples/03-futures-are-lazy.rs) | A Future does nothing until it's `.await`ed | `cargo run -p ch16-async --example 03-futures-are-lazy` |
| [`04-tokio-spawn`](examples/04-tokio-spawn.rs) | `tokio::spawn(async move { ... })` — a cheap task, like `thread::spawn` | `cargo run -p ch16-async --example 04-tokio-spawn` |
| [`05-spawn-static-recipes`](examples/05-spawn-static-recipes.rs) | Three recipes for `'static` in `tokio::spawn`: move, clone, Arc | `cargo run -p ch16-async --example 05-spawn-static-recipes` |
| [`06-many-tasks`](examples/06-many-tasks.rs) | A thousand concurrent tasks; `h.await??` — two levels of errors | `cargo run -p ch16-async --example 06-many-tasks` |
| [`07-joinset`](examples/07-joinset.rs) | `JoinSet` hands out results as they finish, not in order | `cargo run -p ch16-async --example 07-joinset` |
| [`08-join-and-select`](examples/08-join-and-select.rs) | `join!` — wait for both at once; `select!` — whoever is first; `timeout` | `cargo run -p ch16-async --example 08-join-and-select` |
| [`09-mutex-and-await`](examples/09-mutex-and-await.rs) | A `std::sync::Mutex` isn't held across `.await`: close it in a block or use `tokio::sync::Mutex` | `cargo run -p ch16-async --example 09-mutex-and-await` |
| [`11-spawn-blocking`](examples/11-spawn-blocking.rs) | Don't block the runtime: `tokio::time::sleep`, `tokio::fs`, `spawn_blocking` for CPU work | `cargo run -p ch16-async --example 11-spawn-blocking` |
| [`12-async-trait`](examples/12-async-trait.rs) | `async fn` in traits (Rust 1.75+) — without the async_trait crate | `cargo run -p ch16-async --example 12-async-trait` |
| [`13-pin-box-future`](examples/13-pin-box-future.rs) | `Pin<Box<dyn Future<Output = T> + Send>>` — when branches return different futures | `cargo run -p ch16-async --example 13-pin-box-future` |
| [`14-axum-server`](examples/14-axum-server.rs) | A minimal Axum app: State, Path, Json, Result, the route `/users/{id}` | `cargo run -p ch16-async --example 14-axum-server` |
| [`15-debug-handler`](examples/15-debug-handler.rs) | `#[axum::debug_handler]` — human-readable errors for wrong handler signatures | `cargo run -p ch16-async --example 15-debug-handler` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: three `slow` calls in sequence (~600 ms) vs `tokio::join!` (~200 ms) | `cargo run -p ch16-async --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: the same with `tokio::spawn` and a loop over `handles` | `cargo run -p ch16-async --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: `std::thread::sleep` inside `join!` — sequential again | `cargo run -p ch16-async --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: `select!` with a 100 ms timeout for a function that sleeps 500 ms | `cargo run -p ch16-async --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: the first HTTP server — `/hello/{name}` | `cargo run -p ch16-async --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: 5 tasks, task i sleeps (6 - i) * 100 ms — the order in a JoinSet and in a Vec | `cargo run -p ch16-async --example exercise-6` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 16
rustc --edition 2024 chapters/16-async/broken/02-async-main-without-runtime.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`02-async-main-without-runtime.rs`](broken/02-async-main-without-runtime.rs) | `async fn main` without `#[tokio::main]` | error[E0752]: `main` function is not allowed to be `async` |
| [`10-drop-guard-does-not-help.rs`](broken/10-drop-guard-does-not-help.rs) | The trap: `drop(guard)` before `.await` does NOT help (still true on Rust 1.97) | error: future cannot be sent between threads safely |
| [`10-std-mutex-across-await.rs`](broken/10-std-mutex-across-await.rs) | A `std::sync::MutexGuard` alive across `.await` — the future is no longer `Send` | error: future cannot be sent between threads safely |
