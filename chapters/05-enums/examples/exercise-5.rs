//! Вправа 5: `is_cashless` через `matches!`
//!
//! Спроба через `==` (`*self != PaymentMethod::Cash`) не компілюється:
//!   error[E0369]: binary operation `!=` cannot be applied to type `PaymentMethod`
//!   note: an implementation of `PartialEq` might be missing for `PaymentMethod`
//! (див. broken/exercise-5.rs). Компілятору бракує `PartialEq`. Але `matches!`
//! кращий: він перевіряє лише «форму» варіанта, не порівнюючи дані.

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
