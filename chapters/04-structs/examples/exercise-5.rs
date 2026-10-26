//! Вправа 5: що каже компілятор без `Debug` — і власний `impl Debug` замість derive
//!
//! Без derive: `Product doesn't implement Debug` + `note: add #[derive(Debug)] ...`
//! (див. broken/exercise-5.rs). Друга частина підказки — «or manually impl Debug» —
//! корисна, коли якесь поле друкувати не можна. Ось вона:

use std::fmt;

struct Product {
    id: u64,
    name: String,
    supplier_token: String, // секрет: не повинен потрапити в лог
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
        name: "Ноутбук".into(),
        supplier_token: "tok_live_123".into(),
    };
    println!("{p:?}"); // Product { id: 1, name: "Ноутбук", supplier_token: "***" }
    let _ = &p.supplier_token;
}
