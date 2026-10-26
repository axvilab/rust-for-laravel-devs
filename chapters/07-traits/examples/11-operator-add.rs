//! Operators are traits too: `impl Add for Money`

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

    // If you still need the operands, add copies:
    let preview = price.clone() + shipping.clone();
    println!("{preview:?}");

    let total = price + shipping;
    // price and shipping are gone here — add took them (self, not &self)
    println!("{total:?}");
}
