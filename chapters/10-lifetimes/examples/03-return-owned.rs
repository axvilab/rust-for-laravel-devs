//! Created a string in the function — return `String`, not `&str`

fn make_greeting(name: &str) -> String {
    format!("Hello, {name}")
}

fn main() {
    println!("{}", make_greeting("Vitalii"));
}
