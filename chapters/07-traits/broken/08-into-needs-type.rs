//! `.into()` без анотації: компілятор не знає, у що перетворювати
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0283]: type annotations needed
//! Виправлення: `let m: Money = 150050.into();` або `Money::from(150050)` —
//!   див. examples/08-from-into.rs
// expected-error: type annotations needed

#[derive(Debug)]
struct Money {
    amount: i64,
}

impl From<i64> for Money {
    fn from(kopecks: i64) -> Self {
        Money { amount: kopecks }
    }
}

fn main() {
    let m = 150050.into();
    println!("{m:?}");
}
