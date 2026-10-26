//! Exercise 3 (broken version): a `String` after a `move` closure
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `label`
//!   explained as `value moved into closure here` / `variable moved due to use in closure`
//! Fix: see examples/exercise-3.rs (or `label.clone()` for the closure)
// expected-error: E0382

fn main() {
    let label = String::from("discount");
    let show = move || println!("{label}");
    show();
    println!("{label}");
}
