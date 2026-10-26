use crate::models::User;

pub fn notify(user: &User) {
    println!("email to {}", user.email);
}
