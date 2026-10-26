//! `.into()` without an annotation: the compiler doesn't know what to convert into
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0283]: type annotations needed
//! Fix: `let m: Money = 150050.into();` or `Money::from(150050)` —
//!   see examples/08-from-into.rs
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
