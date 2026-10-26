//! The orphan rule: you can't implement a foreign trait for a foreign type
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0117]: only traits defined in the current crate can be
//!   implemented for types defined outside of the crate
//! Fix: a newtype wrapper `struct Tags(Vec<String>)` — see examples/09-orphan-rule.rs
// expected-error: E0117

use std::fmt;

impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} items", self.len())
    }
}

fn main() {}
