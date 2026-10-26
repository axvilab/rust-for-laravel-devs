//! `&T` — borrow to look: the owner stays the owner

// clippy suggests &str instead of &String (ptr_arg) — that's the book's next example;
// here it's &String on purpose, as in the text.
#[allow(clippy::ptr_arg)]
fn print_name(name: &String) {
    // we take a reference
    println!("{name}");
} // name is only a reference, nothing is destroyed

fn main() {
    let n = String::from("Vitalii");
    print_name(&n); // & — we borrow
    print_name(&n); // as many times as you like
    println!("{n}"); // n is still yours
}
