//! A `&str` parameter accepts both `String` and literals

fn print_name(name: &str) {
    println!("{name}");
}

fn main() {
    let owned = String::from("Vitalii");
    print_name(&owned); // &String automatically becomes &str
    print_name("A literal"); // a literal is already &str
    print_name(&owned[0..6]); // and a piece of a string (the first 6 bytes: "Vitali")
}
