//! Exercise 5: `#[derive(Debug, Clone, PartialEq)]` and `product == product.clone()`

#[derive(Debug, Clone, PartialEq)]
struct Product {
    name: String,
    price: f64,
}

fn main() {
    let product = Product {
        name: "Laptop".into(),
        price: 45_000.0,
    };
    // clone() — a new independent copy; PartialEq compares field by field
    println!("{}", product == product.clone()); // true
}
