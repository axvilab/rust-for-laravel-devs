//! `+` забирає доданки (`self`, не `&self`): після додавання їх не використати
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `price`
//! Виправлення: `price.clone() + shipping.clone()` — див. examples/11-operator-add.rs
// expected-error: E0382

use std::ops::Add;

#[derive(Debug)]
struct Money {
    amount: i64,
}

impl Add for Money {
    type Output = Money;
    fn add(self, other: Money) -> Money {
        Money {
            amount: self.amount + other.amount,
        }
    }
}

fn main() {
    let price = Money { amount: 100 };
    let shipping = Money { amount: 7 };
    let total = price + shipping;
    println!("{total:?} {price:?}");
}
