//! `if` — вираз, тому тернарний оператор не потрібен

fn main() {
    let age = 20;
    let status = if age >= 18 { "adult" } else { "minor" };
    println!("{status}");
}
