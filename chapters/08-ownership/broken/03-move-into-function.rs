//! Передав `String` у функцію — віддав
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `n`
//! Виправлення: повернути володіння (examples/05-give-back.rs) або позичити `&str`
//!   (наступний розділ)
// expected-error: E0382

fn print_name(name: String) {
    println!("{name}");
} // name тут знищується

fn main() {
    let n = String::from("Віталій");
    print_name(n); // n переїхав у функцію
    println!("{n}"); // ПОМИЛКА: n більше не твій
}
