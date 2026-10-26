//! Exercise 5: `is_cashless` with `matches!`
//!
//! An attempt with `==` (`*self != PaymentMethod::Cash`) does not compile:
//!   error[E0369]: binary operation `!=` cannot be applied to type `PaymentMethod`
//!   note: an implementation of `PartialEq` might be missing for `PaymentMethod`
//! (see broken/exercise-5.rs). The compiler misses `PartialEq`. But `matches!`
//! is better anyway: it checks only the "shape" of the variant, not the data.

enum PaymentMethod {
    Cash,
    Card { last4: String },
    Crypto { wallet: String, network: String },
}

impl PaymentMethod {
    fn is_cashless(&self) -> bool {
        !matches!(self, PaymentMethod::Cash)
    }
}

fn main() {
    let card = PaymentMethod::Card {
        last4: "4242".into(),
    };
    let crypto = PaymentMethod::Crypto {
        wallet: "0xabc".into(),
        network: "TRC20".into(),
    };
    println!(
        "{} {} {}",
        PaymentMethod::Cash.is_cashless(),
        card.is_cashless(),
        crypto.is_cashless()
    ); // false true true
}
