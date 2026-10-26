//! `unwrap_or` always computes the default, `unwrap_or_else` only on None
//
// clippy (as the book says) suggests the shorter `unwrap_or_else(load_default_name)`
// — the redundant_closure lint. The `||` form is kept on purpose, so the lint is off.
#![allow(clippy::redundant_closure)]

fn find_name() -> Option<String> {
    Some("Vitalii".to_string())
}

fn load_default_name() -> String {
    println!("(an expensive default lookup...)");
    "Guest".to_string()
}

fn main() {
    let a = find_name().unwrap_or(load_default_name()); // called ALWAYS
    let b = find_name().unwrap_or_else(|| load_default_name()); // only on None
    let c = find_name().unwrap_or_default(); // None → ""
    println!("{a} {b} {c}");
}
