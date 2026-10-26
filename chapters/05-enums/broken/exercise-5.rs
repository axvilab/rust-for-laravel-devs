//! Вправа 5 (зламана версія): `!=` для enum без `PartialEq`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0369]: binary operation `!=` cannot be applied to type `PaymentMethod`
//! Виправлення: `matches!` — див. examples/exercise-5.rs (або `#[derive(PartialEq)]`)
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
