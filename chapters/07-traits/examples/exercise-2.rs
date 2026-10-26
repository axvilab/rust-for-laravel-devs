//! Вправа 2: `Priced` для товару і для послуги (ціна = ставка × години)

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
        self.hourly * self.hours // обидва отримали price_with_vat безкоштовно
    }
}

fn main() {
    let p = Product { price: 100.0 };
    let s = Service {
        hourly: 500.0,
        hours: 3.0,
    };
    println!("{} {}", p.price_with_vat(), s.price_with_vat()); // 120 1800
}
