//! `move` takes the `String` into the closure: the variable is gone afterwards
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `name`
//! Fix: don't use `name` after `move`, or give the closure a copy
//!   (`let name2 = name.clone();`) — see examples/07-move.rs
// expected-error: E0382

fn main() {
    let name = String::from("Vitalii");
    let greet = move || println!("Hello, {name}");
    greet();
    println!("{name}"); // ERROR: name moved into the closure
}
