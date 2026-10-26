//! Метод трейту не видно без `use`
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0599]: cannot write into `String`
//!   help: trait `Write` which provides `write_fmt` is implemented but not in scope;
//!         perhaps you want to import it: `use std::fmt::Write;`
//! Виправлення: `use std::fmt::Write;` — див. examples/10-trait-in-scope.rs
// expected-error: cannot write into `String`

fn main() {
    let mut report = String::new();
    write!(report, "Разом: {} грн", 1500).unwrap();
    println!("{report}");
}
