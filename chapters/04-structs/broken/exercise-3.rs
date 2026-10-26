//! Вправа 3 (зламана версія): метод з `&mut self` на незмінній змінній
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0596]: cannot borrow `product` as mutable, as it is not declared as mutable
//! Виправлення: `let mut product` — див. examples/exercise-3.rs
// expected-error: E0596

struct Product {
    stock: u32,
}

impl Product {
    fn new(stock: u32) -> Self {
        Self { stock }
    }
    fn sell(&mut self, qty: u32) {
        self.stock = self.stock.saturating_sub(qty);
    }
}

fn main() {
    let product = Product::new(3);
    product.sell(1);
    println!("{}", product.stock);
}
