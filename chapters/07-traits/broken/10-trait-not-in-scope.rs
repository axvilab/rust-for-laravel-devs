//! A trait method isn't visible without `use`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0599]: cannot write into `String`
//!   help: trait `Write` which provides `write_fmt` is implemented but not in scope;
//!         perhaps you want to import it: `use std::fmt::Write;`
//! Fix: `use std::fmt::Write;` — see examples/10-trait-in-scope.rs
// expected-error: cannot write into `String`

fn main() {
    let mut report = String::new();
    write!(report, "Total: {} UAH", 1500).unwrap();
    println!("{report}");
}
