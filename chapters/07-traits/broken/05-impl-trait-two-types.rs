//! `-> impl Trait` — це ОДИН прихований тип, а не «будь-який з кількох»
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0308]: `if` and `else` have incompatible types
//!   (expected `User`, found `Product`)
//! Виправлення: `-> Box<dyn Describable>` — див. examples/05-return-box-dyn.rs
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
