//! Option A: give ownership away and get it back (works, but nobody writes this)

fn print_name(name: String) -> String {
    println!("{name}");
    name // give ownership back
}

fn main() {
    let n = String::from("Vitalii");
    let n = print_name(n);
    println!("mine again: {n}");
}
