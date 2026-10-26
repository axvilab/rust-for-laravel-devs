# 15. Threads — the parallelism PHP never had

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch15-threads --example 01-spawn-join
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch15-threads --example 01-spawn-join
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-spawn-join`](examples/01-spawn-join.rs) | `thread::spawn` — start a thread; `join` — wait for it and take the result | `cargo run -p ch15-threads --example 01-spawn-join` |
| [`02-spawn-move`](examples/02-spawn-move.rs) | `move` — give the data to the thread | `cargo run -p ch15-threads --example 02-spawn-move` |
| [`03-arc-mutex`](examples/03-arc-mutex.rs) | `Arc<Mutex<T>>`: several owners + change in turn | `cargo run -p ch15-threads --example 03-arc-mutex` |
| [`05-short-lock`](examples/05-short-lock.rs) | Keep the lock short: do slow work before `lock()` | `cargo run -p ch15-threads --example 05-short-lock` |
| [`06-match-on-lock`](examples/06-match-on-lock.rs) | A temporary guard in `match` lives until the end of the whole `match` | `cargo run -p ch15-threads --example 06-match-on-lock` |
| [`07-rwlock`](examples/07-rwlock.rs) | `RwLock` — many readers or one writer | `cargo run -p ch15-threads --example 07-rwlock` |
| [`08-mpsc`](examples/08-mpsc.rs) | An `mpsc` channel: many senders, one receiver | `cargo run -p ch15-threads --example 08-mpsc` |
| [`09-scoped-threads`](examples/09-scoped-threads.rs) | `thread::scope` — lend local data to threads without `Arc` and `move` | `cargo run -p ch15-threads --example 09-scoped-threads` |
| [`10-boxed-send-jobs`](examples/10-boxed-send-jobs.rs) | `Vec<Box<dyn FnOnce() -> u64 + Send>>` — a list of jobs for threads | `cargo run -p ch15-threads --example 10-boxed-send-jobs` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: 4 threads sum parts of `0..1_000_000`, the results come via `join` | `cargo run -p ch15-threads --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: the same with a shared `Arc<Mutex<u64>>` | `cargo run -p ch15-threads --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: the same with an `mpsc` channel | `cargo run -p ch15-threads --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: an `Rc<i32>` won't go into a thread — an `Arc<i32>` will | `cargo run -p ch15-threads --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `match` on `lock()` holds the lock until the end of the match — `try_lock` shows it | `cargo run -p ch15-threads --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: a deadlock from a different lock order — and the fix with one order | `cargo run -p ch15-threads --example exercise-6` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 15
rustc --edition 2024 chapters/15-threads/broken/02-spawn-without-move.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`02-spawn-without-move.rs`](broken/02-spawn-without-move.rs) | A thread borrows a local variable — which may not live long enough | error[E0373]: closure may outlive the current function, but it |
| [`04-arc-not-cloned.rs`](broken/04-arc-not-cloned.rs) | Forgot `Arc::clone` before `move` in a loop | error[E0382]: the type `Arc` does not implement `Copy` |
| [`10-rc-is-not-send.rs`](broken/10-rc-is-not-send.rs) | An `Rc` can't be sent to another thread | error[E0277]: `Rc<i32>` cannot be sent between threads safely |
