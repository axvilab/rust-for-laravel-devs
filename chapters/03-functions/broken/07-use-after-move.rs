//! `move` забирає `String` у замикання: після цього змінна недоступна
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `name`
//! Виправлення: не використовувати `name` після `move`, або передати в замикання
//!   копію (`let name2 = name.clone();`) — див. examples/07-move.rs
// expected-error: E0382

fn main() {
    let name = String::from("Віталій");
    let greet = move || println!("Привіт, {name}");
    greet();
    println!("{name}"); // ПОМИЛКА: name переїхав у замикання
}
