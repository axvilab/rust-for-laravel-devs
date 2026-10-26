//! Вправа 1: структура `Product` з `#[derive(Debug)]`

#[derive(Debug)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    stock: u32,
}

fn main() {
    let p = Product {
        id: 1,
        name: "Ноутбук".into(),
        price: 45_000.0,
        stock: 3,
    };
    println!("{p:?}");
}
