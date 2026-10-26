//! Вправа 7: `From<f64> for Product`; `.into()` потребує анотації типу
//!
//! `let p = 99.9.into();` без типу дає error[E0283]: type annotations needed
//! (див. broken/exercise-7.rs): `.into()` може перетворити f64 у багато типів,
//! і компілятор не вгадує. Виправлення — назвати тип одним із двох способів.

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

impl From<f64> for Product {
    fn from(price: f64) -> Self {
        Product {
            name: "Без назви".to_string(),
            price,
        }
    }
}

fn main() {
    let p: Product = 99.9.into(); // спосіб 1: анотація змінної
    let q = Product::from(10.0); // спосіб 2: From напряму — читабельніше
    println!("{p:?}\n{q:?}");
}
