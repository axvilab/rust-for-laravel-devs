//! Exercise 5: `greet` with an optional greeting via `Option<&str>`

fn greet(name: &str, greeting: Option<&str>) -> String {
    let greeting = greeting.unwrap_or("Hello"); // None → "Hello"
    format!("{greeting}, {name}!")
}

fn main() {
    println!("{}", greet("Vitalii", None)); // Hello, Vitalii!
    println!("{}", greet("Vitalii", Some("Good afternoon"))); // Good afternoon, Vitalii!
}
