//! Exercise 3: `sell` requires `let mut product`
//!
//! Without `mut` the compiler says:
//!   error[E0596]: cannot borrow `product` as mutable, as it is not declared as mutable
//!   help: consider changing this to be mutable: `mut product`
//! (see broken/exercise-3.rs). `&mut self` in the signature of `sell` needs a mutable owner.

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
    let mut product = Product::new(3); // the fix: added mut
    product.sell(1);
    println!("{}", product.stock); // 2
}
