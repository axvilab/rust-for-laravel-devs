//! A thread borrows a local variable — which may not live long enough
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0373]: closure may outlive the current function, but it
//!   borrows `name`, which is owned by the current function
//!   help: to force the closure to take ownership of `name` ... use the `move` keyword
//! Fix: `thread::spawn(move || ...)` — see examples/02-spawn-move.rs
// expected-error: E0373

use std::thread;

fn main() {
    let name = String::from("Vitalii");
    let h = thread::spawn(|| {
        println!("{name}");
    });
    h.join().unwrap();
}
