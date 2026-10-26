//! `use std::fmt::Write` — without importing the trait, `write!` on `String` isn't found

use std::fmt::Write; // without this line write! on String isn't found

fn main() {
    let mut report = String::new();
    write!(report, "Total: {} UAH", 1500).unwrap();
    println!("{report}");
}
