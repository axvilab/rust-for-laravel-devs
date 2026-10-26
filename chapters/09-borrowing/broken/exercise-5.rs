//! Exercise 5 (broken version): `match` without `&` took the contents of the Option
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of partially moved value: `maybe`
//! One-character fix: `match &maybe` — see examples/exercise-5.rs
// expected-error: partially moved

fn main() {
    let maybe: Option<String> = Some("test".into());
    match maybe {
        Some(s) => println!("present: {s}"),
        None => println!("absent"),
    }
    println!("{maybe:?}");
}
