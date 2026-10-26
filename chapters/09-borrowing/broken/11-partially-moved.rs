//! `match` without `&` took the string out of the Option — `name` isn't whole any more
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of partially moved value: `name`
//! Fix: `match &name` — see examples/11-match-on-reference.rs
// expected-error: partially moved

fn main() {
    let name: Option<String> = Some("Vitalii".to_string());
    if let Some(n) = name {
        println!("took: {n}");
    }
    println!("{name:?}");
}
