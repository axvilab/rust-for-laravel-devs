//! Exercise 5: what the compiler says without `Debug` — and a hand-written `impl Debug`
//!
//! Without derive: `Product doesn't implement Debug` + `note: add #[derive(Debug)] ...`
//! (see broken/exercise-5.rs). The second half of the hint, "or manually impl Debug",
//! is useful when some field must not be printed. Here it is:

use std::fmt;

struct Product {
    id: u64,
    name: String,
    supplier_token: String, // a secret: must not end up in a log
}

impl fmt::Debug for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Product")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("supplier_token", &"***")
            .finish()
    }
}

fn main() {
    let p = Product {
        id: 1,
        name: "Laptop".into(),
        supplier_token: "tok_live_123".into(),
    };
    println!("{p:?}"); // Product { id: 1, name: "Laptop", supplier_token: "***" }
    let _ = &p.supplier_token;
}
