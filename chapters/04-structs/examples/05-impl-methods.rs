//! Блок `impl`: конструктор `new`, методи з `&self` і `&mut self`

struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

impl User {
    // «статичний» метод — без self. Так роблять конструктори.
    fn new(id: u64, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active: true,
        }
    }

    // читає, не змінює
    fn display_name(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }

    // змінює
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user = User::new(1, "Віталій", "v@example.com");
    println!("{}", user.display_name());
    user.deactivate();
    println!("id={} active={}", user.id, user.active);
}
