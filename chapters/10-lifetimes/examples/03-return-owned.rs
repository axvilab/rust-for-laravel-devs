//! Створив рядок у функції — повертай `String`, не `&str`

fn make_greeting(name: &str) -> String {
    format!("Привіт, {name}")
}

fn main() {
    println!("{}", make_greeting("Віталій"));
}
