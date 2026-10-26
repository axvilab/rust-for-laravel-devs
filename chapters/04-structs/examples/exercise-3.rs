//! Вправа 3: `sell` вимагає `let mut product`
//!
//! Без `mut` компілятор скаже:
//!   error[E0596]: cannot borrow `product` as mutable, as it is not declared as mutable
//!   help: consider changing this to be mutable: `mut product`
//! (див. broken/exercise-3.rs). `&mut self` у сигнатурі `sell` вимагає змінного власника.

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
    let mut product = Product::new(3); // виправлення: додали mut
    product.sell(1);
    println!("{}", product.stock); // 2
}
