//! A trait is an interface; the implementation is a separate `impl Trait for Type` block

trait Notifiable {
    fn route_for(&self, channel: &str) -> String;
}

struct User {
    email: String,
}

impl Notifiable for User {
    fn route_for(&self, _channel: &str) -> String {
        self.email.clone()
    }
}

fn main() {
    let u = User {
        email: "v@example.com".into(),
    };
    println!("{}", u.route_for("mail"));
}
