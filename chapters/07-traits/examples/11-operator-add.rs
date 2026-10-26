//! Оператори — теж трейти: `impl Add for Money`

use std::ops::Add;

#[derive(Debug, Clone)]
struct Money {
    amount: i64,
    currency: String,
}

impl Add for Money {
    type Output = Money;
    fn add(self, other: Money) -> Money {
        Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}

fn main() {
    let price = Money {
        amount: 150_000,
        currency: "UAH".into(),
    };
    let shipping = Money {
        amount: 7_000,
        currency: "UAH".into(),
    };

    // Якщо доданки ще потрібні — додаємо копії:
    let preview = price.clone() + shipping.clone();
    println!("{preview:?}");

    let total = price + shipping;
    // price і shipping тут уже недоступні — add забрав їх собі (self, а не &self)
    println!("{total:?}");
}
