//! Exercise 5: with a `String` inside — only `Clone`, the copy is explicit
//!
//! `#[derive(Copy)]` here gives E0204 (see broken/exercise-5.rs).

#[derive(Debug, Clone)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let a = User {
        id: 1,
        name: "Vitalii".into(),
    };
    let b = a.clone(); // an explicit copy instead of an implicit one
    println!("{a:?} {b:?}");
}
