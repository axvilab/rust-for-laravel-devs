//! Вправа 5: `greet` з необов'язковим привітанням через `Option<&str>`

fn greet(name: &str, greeting: Option<&str>) -> String {
    let greeting = greeting.unwrap_or("Привіт"); // None → "Привіт"
    format!("{greeting}, {name}!")
}

fn main() {
    println!("{}", greet("Віталій", None)); // Привіт, Віталій!
    println!("{}", greet("Віталій", Some("Добрий день"))); // Добрий день, Віталій!
}
