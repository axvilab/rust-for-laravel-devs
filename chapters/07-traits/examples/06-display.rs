//! `Display` пишеться руками — людський формат для `{}`

use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
struct Money {
    amount: i64,
    currency: String,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{:02} {}",
            self.amount / 100,
            self.amount % 100,
            self.currency
        )
    }
}

fn main() {
    println!(
        "{}",
        Money {
            amount: 150050,
            currency: "UAH".into()
        }
    ); // 1500.50 UAH
}
