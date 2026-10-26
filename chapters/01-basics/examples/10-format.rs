//! Formatting: positional, named, `{:?}`, `{:#?}`, alignment, precision

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let name = "Vitalii";
    let age = 35;
    let some_struct = User {
        id: 1,
        name: "Vitalii".into(),
    };

    println!("{} is {} years old", name, age); // positional
    println!("{name} is {age} years old"); // named (since Rust 2021)
    println!("{:?}", vec![1, 2, 3]); // {:?} — "debug" output, like var_dump
    println!("{:#?}", some_struct); // {:#?} — the same, but with line breaks
    println!("{:>8}|{:<8}|", "right", "left"); // alignment
    // Not 3.14159: clippy treats such a number as a hand-written π and refuses
    // (clippy::approx_constant is an error, not a warning). Hence another number:
    println!("{:.2}", 1.23456); // 1.23

    let price = 99.5;
    let users = ["Ivan", "Maria"];
    println!("{users:?} {price:.2}"); // modifiers with names
}
