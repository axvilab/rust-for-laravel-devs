use crate::models::User;

pub fn notify(user: &User) {
    println!("лист на {}", user.email);
}
