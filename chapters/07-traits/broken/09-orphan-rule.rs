//! Правило сироти: чужий трейт для чужого типу реалізувати не можна
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0117]: only traits defined in the current crate can be
//!   implemented for types defined outside of the crate
//! Виправлення: обгортка-newtype `struct Tags(Vec<String>)` — див. examples/09-orphan-rule.rs
// expected-error: E0117

use std::fmt;

impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} items", self.len())
    }
}

fn main() {}
