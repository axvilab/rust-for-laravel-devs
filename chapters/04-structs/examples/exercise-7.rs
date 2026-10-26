//! Exercise 7: a hand-written `impl Default` and `..Default::default()`

#[derive(Debug)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    stock: u32,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Untitled".to_string(),
            price: 0.0,
            stock: 1,
        }
    }
}

fn main() {
    let p = Product {
        id: 7,
        ..Default::default()
    };
    println!("{p:?}"); // Product { id: 7, name: "Untitled", price: 0.0, stock: 1 }
}
