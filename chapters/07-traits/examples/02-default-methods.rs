//! A default implementation: a type overrides only what it wants

trait Describable {
    fn name(&self) -> String; // required

    fn describe(&self) -> String {
        // has a default
        format!("This is {}", self.name())
    }
}

struct User {
    name: String,
}

struct Product {
    title: String,
    price: f64,
}

impl Describable for User {
    fn name(&self) -> String {
        self.name.clone()
    }
    // describe() came for free
}

impl Describable for Product {
    fn name(&self) -> String {
        self.title.clone()
    }
    fn describe(&self) -> String {
        // overridden
        format!("Product “{}” for {} UAH", self.title, self.price)
    }
}

fn main() {
    let u = User {
        name: "Vitalii".into(),
    };
    let p = Product {
        title: "Laptop".into(),
        price: 45_000.0,
    };
    println!("{}", u.describe()); // This is Vitalii
    println!("{}", p.describe()); // Product “Laptop” for 45000 UAH
}
