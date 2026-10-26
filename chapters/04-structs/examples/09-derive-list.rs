//! `#[derive(Debug, Clone, PartialEq, Default)]` in action

#[derive(Debug, Clone, PartialEq, Default)]
struct Money {
    amount: i64,
    currency: String,
}

fn main() {
    let a = Money {
        amount: 150_050,
        currency: "UAH".into(),
    };
    let b = a.clone(); // Clone
    println!("{a:?}"); // Debug
    println!("{}", a == b); // PartialEq: true
    println!("{:?}", Money::default()); // Default: Money { amount: 0, currency: "" }
}
