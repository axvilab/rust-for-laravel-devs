//! Exercise 6 (broken version): the same String three times into a function that takes String
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: use of moved value: `email`
//!   (value moved here, in previous iteration of loop)
//! Fix: see examples/exercise-6.rs
// expected-error: in previous iteration of loop

fn notify(email: String) {
    println!("email to {email}");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        notify(email);
    }
}
