//! You can't reach a field inside an `Option` without checking
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0609]: no field `name` on type `Option<User>`
//! Fix: `match` / `if let` — see examples/07-option.rs
// expected-error: E0609

struct User {
    name: String,
}

fn find_user(id: u64) -> Option<User> {
    (id == 1).then(|| User {
        name: "Vitalii".into(),
    })
}

fn main() {
    println!("{}", find_user(5).name);
}
