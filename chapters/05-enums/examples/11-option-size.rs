//! Розмір Option: для посилань і Box обгортка безкоштовна

use std::mem::size_of;

struct User {
    id: u64,
    name: String,
}

fn main() {
    println!("&User         = {} байт", size_of::<&User>());
    println!("Option<&User> = {} байт", size_of::<Option<&User>>()); // те саме
    println!("Box<User>         = {} байт", size_of::<Box<User>>());
    println!(
        "Option<Box<User>> = {} байт",
        size_of::<Option<Box<User>>>()
    ); // те саме
    println!("u64         = {} байт", size_of::<u64>());
    println!("Option<u64> = {} байт", size_of::<Option<u64>>()); // тут потрібен прапорець
}
