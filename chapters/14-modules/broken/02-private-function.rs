//! Виклик приватної функції з іншого модуля
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0603]: function `format_money` is private
//! Виправлення: `pub fn` (або `pub(crate)`) — див. examples/02-paths-and-visibility.rs
// expected-error: E0603

mod helpers {
    fn format_money(kopecks: u64) -> String {
        format!("{} грн", kopecks / 100)
    }
}

fn main() {
    println!("{}", helpers::format_money(1500));
}
