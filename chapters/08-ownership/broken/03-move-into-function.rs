//! Passed a `String` to a function — gave it away
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `n`
//! Fix: give ownership back (examples/05-give-back.rs) or borrow a `&str`
//!   (the next chapter)
// expected-error: E0382

fn print_name(name: String) {
    println!("{name}");
} // name is destroyed here

fn main() {
    let n = String::from("Vitalii");
    print_name(n); // n moved into the function
    println!("{n}"); // ERROR: n isn't yours any more
}
