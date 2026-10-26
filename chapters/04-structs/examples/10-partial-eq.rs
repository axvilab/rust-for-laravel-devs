//! Чому `PartialEq` є, а `Eq` для `f64` — ні: NaN не дорівнює собі

#[derive(Debug, PartialEq)]
struct Price {
    amount: f64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sku {
    code: String,
}

fn main() {
    // У книзі `nan == nan` і `Price { amount: 1.0 } == Price { amount: 1.0 }`;
    // clippy відхиляє порівняння виразу з таким самим виразом (clippy::eq_op —
    // це помилка, не попередження), тому значення розкладено по змінних.
    let nan = f64::NAN;
    let same_nan = nan;
    println!("{}", nan == same_nan); // false!
    let p1 = Price { amount: 1.0 };
    let p2 = Price { amount: 1.0 };
    println!("{}", p1 == p2); // true
    let a = Sku { code: "A-1".into() };
    println!("{}", a == Sku { code: "A-1".into() }); // true
}
