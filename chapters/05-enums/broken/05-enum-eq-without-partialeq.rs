//! `==` for an enum without `PartialEq`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0369]: binary operation `==` cannot be applied to type `Status`
//! Fix: `#[derive(PartialEq)]` or `matches!` — see examples/05-enum-eq.rs
// expected-error: E0369

enum Status {
    Pending,
    Paid { amount: i64 },
}

fn main() {
    let s = Status::Paid { amount: 1500 };
    if s == Status::Pending {
        println!("waiting");
    }
}
