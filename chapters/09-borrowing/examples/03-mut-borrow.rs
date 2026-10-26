//! `&mut T` — позичити з правом змін: три `mut`, і всі потрібні

fn add_suffix(name: &mut String) {
    name.push_str(" (admin)");
}

fn main() {
    let mut n = String::from("Віталій"); // сам власник має бути mut
    add_suffix(&mut n); // позичаємо з правом змін
    println!("{n}"); // Віталій (admin)
}
