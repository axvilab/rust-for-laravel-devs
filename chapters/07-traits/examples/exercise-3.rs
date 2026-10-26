//! Exercise 3: `total` over a `Box<dyn Priced>` list of mixed products and services

trait Priced {
    fn price(&self) -> f64;
    fn price_with_vat(&self) -> f64 {
        self.price() * 1.2
    }
}

struct Product {
    price: f64,
}
struct Service {
    hourly: f64,
    hours: f64,
}

impl Priced for Product {
    fn price(&self) -> f64 {
        self.price
    }
}
impl Priced for Service {
    fn price(&self) -> f64 {
        self.hourly * self.hours
    }
}

// Different types in one list → dyn; a &[...] slice instead of &Vec<...>
fn total(items: &[Box<dyn Priced>]) -> f64 {
    items.iter().map(|i| i.price_with_vat()).sum()
}

fn main() {
    let items: Vec<Box<dyn Priced>> = vec![
        Box::new(Product { price: 100.0 }),
        Box::new(Service {
            hourly: 500.0,
            hours: 3.0,
        }),
        Box::new(Product { price: 50.0 }),
    ];
    println!("{}", total(&items)); // 120 + 1800 + 60 = 1980
}
