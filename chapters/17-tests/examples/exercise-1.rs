//! Вправа 1: тест, що навмисно падає, з поясненням у `assert_eq!`
//!
//! Тест позначено `#[ignore]`, щоб `cargo test` у CI проходив. Запуск:
//!   cargo test -p ch17-tests --example exercise-1 -- --ignored
//! Вивід:
//!   ---- tests::broken stdout ----
//!   порахували: 20
//!   thread 'tests::broken' panicked at ...:
//!   assertion `left == right` failed: комісія для 1000
//!     left: 20
//!    right: 25
//! `left` — що порахував код, `right` — що ти очікував.

use ch17_tests::calc_fee;

fn main() {
    println!("calc_fee(1000) = {}", calc_fee(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "навмисно падає: cargo test --example exercise-1 -- --ignored"]
    fn broken() {
        let fee = calc_fee(1000);
        println!("порахували: {fee}");
        assert_eq!(fee, 25, "комісія для 1000");
    }
}
