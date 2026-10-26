//! A new enum variant — and every `match` without it stops compiling
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0004]: non-exhaustive patterns: `&OrderStatus::Refunded` not covered
//! Fix: add an arm for `Refunded` — the compiler showed where.
// expected-error: E0004

enum OrderStatus {
    Pending,
    Paid { amount: f64 },
    Refunded, // a new variant
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => "awaiting payment".to_string(),
        OrderStatus::Paid { amount } => format!("paid {amount} UAH"),
    }
}

fn main() {
    println!("{}", describe(&OrderStatus::Refunded));
}
