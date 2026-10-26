//! Exercise 5 (broken version): `!=` for an enum without `PartialEq`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0369]: binary operation `!=` cannot be applied to type `PaymentMethod`
//! Fix: `matches!` — see examples/exercise-5.rs (or `#[derive(PartialEq)]`)
// expected-error: E0369

enum PaymentMethod {
    Cash,
    Card { last4: String },
}

impl PaymentMethod {
    fn is_cashless(&self) -> bool {
        *self != PaymentMethod::Cash
    }
}

fn main() {
    let card = PaymentMethod::Card {
        last4: "4242".into(),
    };
    println!("{}", card.is_cashless());
}
