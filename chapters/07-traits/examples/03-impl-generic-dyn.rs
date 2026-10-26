//! Three parameter forms: `&impl Trait`, `<T: Trait>`, `&dyn Trait`

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

// 1. impl Trait — the simplest, write it by default
fn announce(item: &impl Describable) {
    println!("{}", item.describe());
}

// 2. A generic with a bound — the same, but you can refer to T several times
fn announce_pair<T: Describable>(a: &T, b: &T) {
    println!("{} and {}", a.describe(), b.describe());
}

// 3. dyn Trait — "dynamic" dispatch, the implementation is picked at runtime
fn announce_dyn(item: &dyn Describable) {
    println!("{}", item.describe());
}

fn main() {
    let u1 = User {
        name: "Vitalii".into(),
    };
    let u2 = User {
        name: "Maria".into(),
    };
    let p = Product {
        title: "Laptop".into(),
    };

    announce(&u1);
    announce(&p);
    announce_pair(&u1, &u2); // two Users — yes
    // announce_pair(&u1, &p); // a User and a Product — no: E0308 mismatched types
    announce_dyn(&p);
}
