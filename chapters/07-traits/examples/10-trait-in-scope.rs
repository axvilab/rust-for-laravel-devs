//! `use std::fmt::Write` — без імпорту трейту `write!` на `String` не знайдеться

use std::fmt::Write; // без цього рядка write! на String не знайдеться

fn main() {
    let mut report = String::new();
    write!(report, "Разом: {} грн", 1500).unwrap();
    println!("{report}");
}
