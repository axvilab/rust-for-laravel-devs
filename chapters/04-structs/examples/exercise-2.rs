//! Вправа 2: `new`, `is_available`, `sell` з `saturating_sub`

#[derive(Debug)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    fn new(id: u64, name: &str, price: f64, stock: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            price,
            stock,
        }
    }

    fn is_available(&self) -> bool {
        self.stock > 0
    }

    fn sell(&mut self, qty: u32) {
        // saturating_sub: 2 - 5 = 0, а не паніка чи 4294967293
        self.stock = self.stock.saturating_sub(qty);
    }
}

fn main() {
    let mut p = Product::new(1, "Ноутбук", 45_000.0, 2);
    p.sell(1);
    println!("{} в наявності: {}", p.stock, p.is_available()); // 1 true
    p.sell(5);
    println!("{} в наявності: {}", p.stock, p.is_available()); // 0 false
}
