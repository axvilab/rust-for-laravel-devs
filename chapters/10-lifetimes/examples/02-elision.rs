//! Правила виведення: один параметр-посилання або `&self` — лайфтайми не потрібні

struct User {
    name: String,
}

// правило 2: один параметр-посилання → результат отримує його лайфтайм
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

impl User {
    // правило 3: є &self → результат отримує лайфтайм self
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    println!("{}", first_word("Привіт, світе"));
    let u = User {
        name: "Віталій".into(),
    };
    println!("{}", u.name());
}
