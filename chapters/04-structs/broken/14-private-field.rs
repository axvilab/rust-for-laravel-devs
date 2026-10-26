//! A struct with a private field can't be built with a literal outside its module
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0451]: field `password_hash` of struct `User` is private
//! Fix: build it with `User::new(...)` — see examples/14-visibility.rs
// expected-error: E0451

mod users {
    pub struct User {
        pub id: u64,
        password_hash: String,
    }
}

fn main() {
    let u = users::User {
        id: 1,
        password_hash: "x".into(),
    };
    println!("{}", u.id);
}
