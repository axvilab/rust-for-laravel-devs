//! «Або те, або інше» з функції — `Box<dyn Trait>`

trait Describable {
    fn describe(&self) -> String;
}
struct User;
struct Product;
impl Describable for User {
    fn describe(&self) -> String {
        "користувач".into()
    }
}
impl Describable for Product {
    fn describe(&self) -> String {
        "товар".into()
    }
}

fn pick(user: bool) -> Box<dyn Describable> {
    if user {
        Box::new(User)
    } else {
        Box::new(Product)
    }
}

fn main() {
    println!("{} / {}", pick(true).describe(), pick(false).describe());
}
