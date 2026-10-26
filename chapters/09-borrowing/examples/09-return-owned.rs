//! Return a `String` (ownership), not a `&str` to a local variable

fn longest_name() -> String {
    let mut name = String::from("Vitalii");
    name.push_str(" Ax");
    name // ownership passes to the caller
}

fn main() {
    println!("{}", longest_name());
}
