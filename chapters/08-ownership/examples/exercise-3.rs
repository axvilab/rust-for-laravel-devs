//! Вправа 3: `shout(String)` забирає рядок — два способи виправити
//!
//! Без виправлень: `shout(s); println!("{s}")` → E0382 (broken/exercise-3.rs).

fn shout(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    // Спосіб 1: «повернути володіння» — результат shout і є новий власник
    let s = String::from("привіт");
    let s = shout(s);
    println!("{s}"); // ПРИВІТ

    // Спосіб 2: віддати копію, оригінал лишається
    let original = String::from("привіт");
    let loud = shout(original.clone());
    println!("{original} → {loud}");

    // (Спосіб 3, з наступного розділу: fn shout(s: &str) -> String.)
}
