//! A `&str` field in a struct without a lifetime does not compile
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0106]: missing lifetime specifier
//! Fix: struct fields hold `String` (see examples/03-string-conversions.rs)
// expected-error: E0106

struct User {
    name: &str,
}

fn main() {
    let u = User { name: "Vitalii" };
    println!("{}", u.name);
}
