//! Вправа 3: `move` з числом не ламає нічого, а з `String` — ламає
//!
//! `percent` — f64, тип `Copy`: `move` забрав копію, оригінал живий.
//! `label` — String, не `Copy`: `move` забрав сам рядок. Спроба надрукувати
//! `label` після замикання дає error[E0382]: borrow of moved value: `label`
//! (див. broken/exercise-3.rs). Повністю це пояснює розділ про ownership.

fn main() {
    let percent = 0.1;
    let discount = move |price: f64| price * (1.0 - percent);
    println!("{}", discount(200.0)); // 180
    println!("percent досі доступний: {percent}");

    let label = String::from("знижка");
    let show = move || println!("{label}");
    show();
    // println!("{label}"); // ПОМИЛКА E0382
}
