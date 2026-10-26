//! Новий варіант enum — і кожен `match` без нього перестає компілюватися
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0004]: non-exhaustive patterns: `&OrderStatus::Refunded` not covered
//! Виправлення: додати гілку для `Refunded` — компілятор сам показав, де.
// expected-error: E0004

enum OrderStatus {
    Pending,
    Paid { amount: f64 },
    Refunded, // новий варіант
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => "очікує оплати".to_string(),
        OrderStatus::Paid { amount } => format!("оплачено {amount} грн"),
    }
}

fn main() {
    println!("{}", describe(&OrderStatus::Refunded));
}
