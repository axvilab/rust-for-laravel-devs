//! Exercise 6 (broken version): `Invoice` added, but not the `match` arm in `code()`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0004]: non-exhaustive patterns: `&PaymentMethod::Invoice { .. }` not covered
//! Fix: an arm for `Invoice` — see examples/exercise-6.rs
// expected-error: E0004

enum PaymentMethod {
    Cash,
    Card { last4: String },
    Invoice { number: String },
}

impl PaymentMethod {
    fn code(&self) -> &'static str {
        match self {
            PaymentMethod::Cash => "cash",
            PaymentMethod::Card { .. } => "card",
        }
    }
}

fn main() {
    println!("{}", PaymentMethod::Cash.code());
}
