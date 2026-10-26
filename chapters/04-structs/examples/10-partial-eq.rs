//! Why there's `PartialEq` but no `Eq` for `f64`: NaN isn't equal to itself

#[derive(Debug, PartialEq)]
struct Price {
    amount: f64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sku {
    code: String,
}

fn main() {
    // Not `nan == nan` or `Price { amount: 1.0 } == Price { amount: 1.0 }`:
    // clippy rejects comparing an expression with an identical one (clippy::eq_op
    // is an error, not a warning), so the values live in separate variables.
    let nan = f64::NAN;
    let same_nan = nan;
    println!("{}", nan == same_nan); // false!
    let p1 = Price { amount: 1.0 };
    let p2 = Price { amount: 1.0 };
    println!("{}", p1 == p2); // true
    let a = Sku { code: "A-1".into() };
    println!("{}", a == Sku { code: "A-1".into() }); // true
}
