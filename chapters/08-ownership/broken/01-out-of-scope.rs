//! A variable doesn't exist outside its block
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0425]: cannot find value `name` in this scope
//! Fix: use `name` inside the block — see examples/01-scope.rs
// expected-error: E0425

fn main() {
    {
        let name = String::from("Vitalii");
        println!("{name}");
    }
    println!("{name}");
}
