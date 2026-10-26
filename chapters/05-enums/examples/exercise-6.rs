//! Вправа 6: `code()` і новий варіант `Invoice`
//!
//! Після додавання `Invoice` компілятор вимагає виправити КОЖЕН match по
//! `PaymentMethod` без `_`: тут це `code()` і `fee()` (див. broken/exercise-6.rs —
//! E0004 вказує на обидва). `matches!` у `is_cashless` міняти не довелося.

enum PaymentMethod {
    Cash,
    Card { last4: String },
    Crypto { wallet: String, network: String },
    Invoice { number: String }, // новий варіант
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
