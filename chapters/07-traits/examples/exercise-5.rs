//! Вправа 5: `#[derive(Debug, Clone, PartialEq)]` і `product == product.clone()`

#[derive(Debug, Clone, PartialEq)]
struct Product {
    name: String,
    price: f64,
}

fn main() {
    let product = Product {
        name: "Ноутбук".into(),
        price: 45_000.0,
    };
    // clone() — нова незалежна копія; PartialEq порівнює поле за полем
    println!("{}", product == product.clone()); // true
}
