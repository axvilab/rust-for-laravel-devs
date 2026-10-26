//! A function with a `String` parameter becomes the owner and destroys the string at the end

fn print_name(name: String) {
    println!("{name}");
} // name is destroyed here

fn main() {
    let n = String::from("Vitalii");
    print_name(n); // n moved into the function
    // println!("{n}");                // ERROR E0382 (broken/03-move-into-function.rs)
}
