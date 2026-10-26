//! Exercise 2: without `pub` a function is private to its module
//!
//! broken/02-private-function.rs: error[E0603]: function `format_money` is private
//! with the note `note: the function format_money is defined here`. The fix is `pub`.

mod helpers {
    pub fn format_money(kopecks: u64) -> String {
        format!("{} UAH", kopecks / 100)
    }
}

fn main() {
    println!("{}", helpers::format_money(1500));
}
