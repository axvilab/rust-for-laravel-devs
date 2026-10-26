//! `-> impl Trait` is ONE hidden type, not "any of several"
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0308]: `if` and `else` have incompatible types
//!   (expected `User`, found `Product`)
//! Fix: `-> Box<dyn Describable>` — see examples/05-return-box-dyn.rs
// expected-error: E0308

trait Describable {
    fn describe(&self) -> String;
}
struct User;
struct Product;
impl Describable for User {
    fn describe(&self) -> String {
        "user".into()
    }
}
impl Describable for Product {
    fn describe(&self) -> String {
        "product".into()
    }
}

fn pick(user: bool) -> impl Describable {
    if user { User } else { Product }
}

fn main() {
    println!("{}", pick(true).describe());
}
