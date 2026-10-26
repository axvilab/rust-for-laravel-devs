//! An `impl` block: a `new` constructor, methods with `&self` and `&mut self`

struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

impl User {
    // a "static" method — no self. That's how constructors are written.
    fn new(id: u64, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active: true,
        }
    }

    // reads, doesn't change
    fn display_name(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }

    // changes
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut user = User::new(1, "Vitalii", "v@example.com");
    println!("{}", user.display_name());
    user.deactivate();
    println!("id={} active={}", user.id, user.active);
}
