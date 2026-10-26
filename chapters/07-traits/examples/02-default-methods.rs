//! Реалізація за замовчуванням: тип перевизначає лише те, що хоче

trait Describable {
    fn name(&self) -> String; // обов'язковий

    fn describe(&self) -> String {
        // є дефолт
        format!("Це {}", self.name())
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
    // describe() отримали безкоштовно
}

impl Describable for Product {
    fn name(&self) -> String {
        self.title.clone()
    }
    fn describe(&self) -> String {
        // перевизначили
        format!("Товар «{}» за {} грн", self.title, self.price)
    }
}

fn main() {
    let u = User {
        name: "Віталій".into(),
    };
    let p = Product {
        title: "Ноутбук".into(),
        price: 45_000.0,
    };
    println!("{}", u.describe()); // Це Віталій
    println!("{}", p.describe()); // Товар «Ноутбук» за 45000 грн
}
