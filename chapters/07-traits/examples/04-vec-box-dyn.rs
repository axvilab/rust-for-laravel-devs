//! A list of objects of different types — `Vec<Box<dyn Trait>>`

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
        format!("This is {}", self.name)
    }
}
impl Describable for Product {
    fn describe(&self) -> String {
        format!("Product “{}”", self.title)
    }
}

fn main() {
    let user = User {
        name: "Vitalii".into(),
    };
    let product = Product {
        title: "Laptop".into(),
    };

    let items: Vec<Box<dyn Describable>> = vec![Box::new(user), Box::new(product)];
    for item in &items {
        println!("{}", item.describe());
    }
}
