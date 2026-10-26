//! Повертаємо `String` (володіння), а не `&str` на локальну змінну

fn longest_name() -> String {
    let mut name = String::from("Віталій");
    name.push_str(" Ax");
    name // володіння переходить до того, хто викликав
}

fn main() {
    println!("{}", longest_name());
}
