//! Exercise 6: `code()` and the new `Invoice` variant
//!
//! After adding `Invoice` the compiler demands fixing EVERY match on
//! `PaymentMethod` without `_`: here that's `code()` and `fee()` (see broken/exercise-6.rs —
//! E0004 points at both). `matches!` in `is_cashless` didn't need any change.

enum PaymentMethod {
    Cash,
    Card { last4: String },
    Crypto { wallet: String, network: String },
    Invoice { number: String }, // a new variant
}

impl PaymentMethod {
    fn code(&self) -> &'static str {
        match self {
            PaymentMethod::Cash => "cash",
            PaymentMethod::Card { .. } => "card",
            PaymentMethod::Crypto { .. } => "crypto",
            PaymentMethod::Invoice { .. } => "invoice",
        }
    }

    fn fee(&self, amount: f64) -> f64 {
        match self {
            PaymentMethod::Cash | PaymentMethod::Invoice { .. } => 0.0,
            PaymentMethod::Card { .. } => amount * 0.02,
            PaymentMethod::Crypto { .. } => amount * 0.01 + 10.0,
        }
    }

    fn is_cashless(&self) -> bool {
        !matches!(self, PaymentMethod::Cash)
    }
}

fn main() {
    let all = [
        PaymentMethod::Cash,
        PaymentMethod::Card {
            last4: "4242".into(),
        },
        PaymentMethod::Crypto {
            wallet: "0xabc".into(),
            network: "TRC20".into(),
        },
        PaymentMethod::Invoice {
            number: "INV-1".into(),
        },
    ];
    for m in &all {
        println!(
            "{} fee={} cashless={}",
            m.code(),
            m.fee(1000.0),
            m.is_cashless()
        );
    }
}
