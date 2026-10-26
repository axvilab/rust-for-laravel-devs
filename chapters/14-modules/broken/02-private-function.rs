//! Calling a private function from another module
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0603]: function `format_money` is private
//! Fix: `pub fn` (or `pub(crate)`) — see examples/02-paths-and-visibility.rs
// expected-error: E0603

mod helpers {
    fn format_money(kopecks: u64) -> String {
        format!("{} UAH", kopecks / 100)
    }
}

fn main() {
    println!("{}", helpers::format_money(1500));
}
