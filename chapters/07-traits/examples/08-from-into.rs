//! `From` — "my type can be made from this one"; `Into` comes free the other way round

#[derive(Debug)]
struct Money {
    amount: i64,
    currency: String,
}

impl From<i64> for Money {
    fn from(kopecks: i64) -> Self {
        Money {
            amount: kopecks,
            currency: "UAH".to_string(),
        }
    }
}

fn main() {
    let m: Money = 150050.into(); // .into() — the mirror of From
    println!("{m:?}");
    let m = Money::from(150050); // the same
    println!("{m:?}");
}
