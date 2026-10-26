//! `Display` is written by hand — a human-readable format for `{}`

use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
struct Money {
    amount: i64,
    currency: String,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // the sign goes separately: `%` keeps the dividend's sign, and without abs() -150 would give "-1.-50"
        let sign = if self.amount < 0 { "-" } else { "" };
        let abs = self.amount.abs();
        write!(f, "{sign}{}.{:02} {}", abs / 100, abs % 100, self.currency)
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
    for amount in [-150, -5, 0] {
        let m = Money {
            amount,
            currency: "UAH".into(),
        };
        println!("{m}"); // -1.50 UAH / -0.05 UAH / 0.00 UAH
    }
}
