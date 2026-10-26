//! `&str` and `String`: a literal, an owned string, `format!`

fn main() {
    let greeting = "Hello"; // &str, baked into the binary
    let mut name = String::from("Vitalii"); // String, on the heap
    // `push` adds one char, `push_str` a whole string
    name.push('!'); // can be changed
    name.push_str(" from Dnipro");
    let full = format!("{greeting}, {name}"); // String
    println!("{full}");
}
