//! `if` is an expression, so there's no need for a ternary operator

fn main() {
    let age = 20;
    let status = if age >= 18 { "adult" } else { "minor" };
    println!("{status}");
}
