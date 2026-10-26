# 9. Borrowing — lend it without giving it away

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch09-borrowing --example 01-shared-borrow
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch09-borrowing --example 01-shared-borrow
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-shared-borrow`](examples/01-shared-borrow.rs) | `&T` — borrow to look: the owner stays the owner | `cargo run -p ch09-borrowing --example 01-shared-borrow` |
| [`02-str-parameter`](examples/02-str-parameter.rs) | A `&str` parameter accepts both `String` and literals | `cargo run -p ch09-borrowing --example 02-str-parameter` |
| [`03-mut-borrow`](examples/03-mut-borrow.rs) | `&mut T` — borrow with the right to change: three `mut`, all of them needed | `cargo run -p ch09-borrowing --example 03-mut-borrow` |
| [`04-mut-binding-vs-mut-ref`](examples/04-mut-binding-vs-mut-ref.rs) | `let mut r = &x` (can be redirected) vs `let r = &mut x` (can change the value) | `cargo run -p ch09-borrowing --example 04-mut-binding-vs-mut-ref` |
| [`05-borrow-rules`](examples/05-borrow-rules.rs) | Either many `&` or one `&mut`; a borrow ends at its last use | `cargo run -p ch09-borrowing --example 05-borrow-rules` |
| [`06-collect-then-push`](examples/06-collect-then-push.rs) | First collect what to add, then add it | `cargo run -p ch09-borrowing --example 06-collect-then-push` |
| [`07-nll`](examples/07-nll.rs) | NLL: a borrow lives until its last use, not until `}` | `cargo run -p ch09-borrowing --example 07-nll` |
| [`08-two-phase-borrows`](examples/08-two-phase-borrows.rs) | `v.push(v.len())` works (two-phase borrows), `v.push(v.pop())` doesn't | `cargo run -p ch09-borrowing --example 08-two-phase-borrows` |
| [`09-return-owned`](examples/09-return-owned.rs) | Return a `String` (ownership), not a `&str` to a local variable | `cargo run -p ch09-borrowing --example 09-return-owned` |
| [`10-deref`](examples/10-deref.rs) | Dereferencing with `*`: the value behind a reference | `cargo run -p ch09-borrowing --example 10-deref` |
| [`11-match-on-reference`](examples/11-match-on-reference.rs) | `match &x` looks, `match x` takes the contents | `cargo run -p ch09-borrowing --example 11-match-on-reference` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: `count_vowels(&str)` works for both a String and a literal | `cargo run -p ch09-borrowing --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: `append_exclamation(&mut String)` needs `let mut` on the owner | `cargo run -p ch09-borrowing --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: why `push` is forbidden while `&v[0]` is alive | `cargo run -p ch09-borrowing --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: two working options — copy the value or print before `push` | `cargo run -p ch09-borrowing --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: `match &maybe` — one character, and `maybe` stays alive | `cargo run -p ch09-borrowing --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: `push(v.len())` works, `push(v.pop())` — via a temporary variable | `cargo run -p ch09-borrowing --example exercise-6` |

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 09
rustc --edition 2024 chapters/09-borrowing/broken/04-assign-through-shared-ref.rs   # see the compiler message
```

| File | What it shows | Expected error |
|---|---|---|
| [`04-assign-through-shared-ref.rs`](broken/04-assign-through-shared-ref.rs) | You can't change a value through `&` | error[E0594]: cannot assign to `*s`, which is behind a `&` reference |
| [`05-borrow-rules.rs`](broken/05-borrow-rules.rs) | A reader and an editor at the same time | error[E0502]: cannot borrow `list` as immutable because it is |
| [`06-mutate-while-iterating.rs`](broken/06-mutate-while-iterating.rs) | You can't change a collection while iterating over it | error[E0502]: cannot borrow `list` as mutable because it is |
| [`07-nll-late-use.rs`](broken/07-nll-late-use.rs) | The same, but `first` is used AFTER `push` | error[E0502]: cannot borrow `scores` as mutable because it is |
| [`08-two-mut-in-args.rs`](broken/08-two-mut-in-args.rs) | Two `&mut` at once: you can't mutate in the arguments of the same call | error[E0499]: cannot borrow `v` as mutable more than once at a time |
| [`09-return-ref-to-local.rs`](broken/09-return-ref-to-local.rs) | You can't return a reference to a local variable | error[E0106]: missing lifetime specifier |
| [`11-partially-moved.rs`](broken/11-partially-moved.rs) | `match` without `&` took the string out of the Option — `name` isn't whole any more | error[E0382]: borrow of partially moved value: `name` |
| [`exercise-2.rs`](broken/exercise-2.rs) | Exercise 2 (broken version): `&mut` to an immutable variable | error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable |
| [`exercise-3.rs`](broken/exercise-3.rs) | Exercise 3: a reference to an element, then `push`, then reading the reference | error[E0502]: cannot borrow `v` as mutable because it is |
| [`exercise-5.rs`](broken/exercise-5.rs) | Exercise 5 (broken version): `match` without `&` took the contents of the Option | error[E0382]: borrow of partially moved value: `maybe` |
| [`exercise-6.rs`](broken/exercise-6.rs) | Exercise 6 (broken version): `v.push(v.pop().unwrap())` | error[E0499]: cannot borrow `v` as mutable more than once at a time |
