//! Шляхи `crate::`, `super::`, `self::` і рівні видимості `pub`, `pub(crate)`, `pub(super)`

mod shop {
    pub mod billing {
        pub fn invoice() -> String {
            // super:: — батьківський модуль (shop)
            format!("рахунок: {}", super::helpers::format_money(1500))
        }

        pub(crate) fn hash_password(raw: &str) -> String {
            // pub(crate): видно в усьому крейті, але не тим, хто підключить його як бібліотеку
            raw.chars().rev().collect()
        }
    }

    mod helpers {
        // без pub — приватний для shop; pub(super) — видно лише батьку (shop)
        pub(super) fn format_money(kopecks: u64) -> String {
            format!("{}.{:02} грн", kopecks / 100, kopecks % 100)
        }
    }

    pub fn summary() -> String {
        self::billing::invoice() // self:: — поточний модуль
    }
}

use crate::shop::billing; // crate:: — від кореня

fn main() {
    println!("{}", billing::invoice());
    println!("{}", shop::summary());
    println!("{}", shop::billing::hash_password("secret"));
    // shop::helpers::format_money(1); // ПОМИЛКА E0603: module `helpers` is private
}
