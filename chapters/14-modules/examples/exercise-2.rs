//! Вправа 2: без `pub` функція приватна для свого модуля
//!
//! broken/02-private-function.rs: error[E0603]: function `format_money` is private
//! з приміткою `note: the function format_money is defined here`. Виправлення — `pub`.

mod helpers {
    pub fn format_money(kopecks: u64) -> String {
        format!("{} грн", kopecks / 100)
    }
}

fn main() {
    println!("{}", helpers::format_money(1500));
}
