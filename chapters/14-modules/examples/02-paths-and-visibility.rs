//! Paths `crate::`, `super::`, `self::` and visibility levels `pub`, `pub(crate)`, `pub(super)`

mod shop {
    pub mod billing {
        pub fn invoice() -> String {
            // super:: — the parent module (shop)
            format!("invoice: {}", super::helpers::format_money(1500))
        }

        pub(crate) fn hash_password(raw: &str) -> String {
            // pub(crate): visible in the whole crate, but not to whoever uses it as a library
            raw.chars().rev().collect()
        }
    }

    mod helpers {
        // no pub — private to shop; pub(super) — visible only to the parent (shop)
        pub(super) fn format_money(kopecks: u64) -> String {
            format!("{}.{:02} UAH", kopecks / 100, kopecks % 100)
        }
    }

    pub fn summary() -> String {
        self::billing::invoice() // self:: — the current module
    }
}

use crate::shop::billing; // crate:: — from the root

fn main() {
    println!("{}", billing::invoice());
    println!("{}", shop::summary());
    println!("{}", shop::billing::hash_password("secret"));
    // shop::helpers::format_money(1); // ERROR E0603: module `helpers` is private
}
