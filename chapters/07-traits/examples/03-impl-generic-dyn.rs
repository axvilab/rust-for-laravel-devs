//! Три форми параметра: `&impl Trait`, `<T: Trait>`, `&dyn Trait`

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

// 1. impl Trait — найпростіша, пиши її за замовчуванням
fn announce(item: &impl Describable) {
    println!("{}", item.describe());
}

// 2. Generic з обмеженням — те саме, але можна посилатись на тип T кілька разів
fn announce_pair<T: Describable>(a: &T, b: &T) {
    println!("{} і {}", a.describe(), b.describe());
}

// 3. dyn Trait — «динамічна» диспетчеризація, вибір реалізації в рантаймі
fn announce_dyn(item: &dyn Describable) {
    println!("{}", item.describe());
}

fn main() {
    let u1 = User {
        name: "Віталій".into(),
    };
    let u2 = User {
        name: "Марія".into(),
    };
    let p = Product {
        title: "Ноутбук".into(),
    };

    announce(&u1);
    announce(&p);
    announce_pair(&u1, &u2); // два User — так
    // announce_pair(&u1, &p); // User і Product — ні: E0308 mismatched types
    announce_dyn(&p);
}
