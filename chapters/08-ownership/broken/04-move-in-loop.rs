//! A move in a loop: "value moved here, in previous iteration of loop"
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: use of moved value: `email`
//!   note: consider changing this parameter type in function `send` to borrow instead
//!   help: consider cloning the value if the performance cost is acceptable
//! Fix: see examples/04-move-in-loop-fixed.rs (both options)
// expected-error: E0382

fn send(email: String) {
    println!("sent: {email}");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        send(email); // ERROR E0382
    }
}
