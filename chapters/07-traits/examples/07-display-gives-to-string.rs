//! Implement `Display` — get `.to_string()` and `format!` for free

use std::fmt;

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
    let m = Money {
        amount: 9990,
        currency: "UAH".into(),
    };
    let s: String = m.to_string(); // from Display, no separate method
    let f = format!("Total: {m}");
    let n = 42.to_string(); // numbers have Display too — hence to_string()
    println!("{s} | {f} | {n}");
}
