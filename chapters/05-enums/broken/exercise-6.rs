//! Вправа 6 (зламана версія): додали `Invoice`, а `match` у `code()` — ні
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0004]: non-exhaustive patterns: `&PaymentMethod::Invoice { .. }` not covered
//! Виправлення: гілка для `Invoice` — див. examples/exercise-6.rs
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
