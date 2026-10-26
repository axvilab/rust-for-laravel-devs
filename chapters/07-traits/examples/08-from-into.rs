//! `From` — «з цього типу можна зробити мій»; `Into` — безкоштовно навпаки

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
    let m: Money = 150050.into(); // .into() — дзеркало From
    println!("{m:?}");
    let m = Money::from(150050); // те саме
    println!("{m:?}");
}
