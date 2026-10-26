//! Реалізував `Display` — отримав `.to_string()` і `format!` безкоштовно

use std::fmt;

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
    let m = Money {
        amount: 9990,
        currency: "UAH".into(),
    };
    let s: String = m.to_string(); // з Display, жодного окремого методу
    let f = format!("Разом: {m}");
    let n = 42.to_string(); // у чисел теж є Display — тому і to_string()
    println!("{s} | {f} | {n}");
}
