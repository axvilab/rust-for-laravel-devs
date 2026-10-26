//! Список різнотипних об'єктів — `Vec<Box<dyn Trait>>`

trait Describable {
    fn describe(&self) -> String;
}

struct User {
    name: String,
}
struct Product {
    title: String,
}

impl Describable for User {
    fn describe(&self) -> String {
        format!("Це {}", self.name)
    }
}
impl Describable for Product {
    fn describe(&self) -> String {
        format!("Товар «{}»", self.title)
    }
}

fn main() {
    let user = User {
        name: "Віталій".into(),
    };
    let product = Product {
        title: "Ноутбук".into(),
    };

    let items: Vec<Box<dyn Describable>> = vec![Box::new(user), Box::new(product)];
    for item in &items {
        println!("{}", item.describe());
    }
}
