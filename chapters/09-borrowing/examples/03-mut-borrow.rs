//! `&mut T` — borrow with the right to change: three `mut`, all of them needed

fn add_suffix(name: &mut String) {
    name.push_str(" (admin)");
}

fn main() {
    let mut n = String::from("Vitalii"); // the owner itself must be mut
    add_suffix(&mut n); // borrow with the right to change
    println!("{n}"); // Vitalii (admin)
}
