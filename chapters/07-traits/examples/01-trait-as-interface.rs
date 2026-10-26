//! Трейт = інтерфейс; реалізація — окремий блок `impl Trait for Type`

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
