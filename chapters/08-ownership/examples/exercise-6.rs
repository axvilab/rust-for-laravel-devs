//! Exercise 6: a string into a function in a loop — both fixes in turn
//!
//! On the unfixed version (broken/exercise-6.rs) the compiler gives E0382 and two hints:
//!   note: consider changing this parameter type in function `notify` to borrow instead
//!   help: consider cloning the value if the performance cost is acceptable

// The `note:` option: the function borrows
fn notify(email: &str) {
    println!("email to {email}");
}

// The `help:` option: the function owns, we call it with clone()
fn notify_owned(email: String) {
    println!("email to {email} (copy)");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        notify(&email);
    }
    for _ in 0..3 {
        notify_owned(email.clone());
    }
}
