//! `T: 'static` means "holds no borrowed references", not "lives forever"
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0597]: `local` does not live long enough
//!   (argument requires that `local` is borrowed for `'static`)
//! Fix: pass ownership (`keep(local)`) — see examples/06-static-bound.rs
// expected-error: E0597

fn keep<T: 'static>(_x: T) {}

fn main() {
    let owned = String::from("I'm a String, I'm allowed");
    keep(owned); // OK: a String borrows nothing

    let local = String::from("local");
    keep(&local); // ERROR: `local` does not live long enough
}
