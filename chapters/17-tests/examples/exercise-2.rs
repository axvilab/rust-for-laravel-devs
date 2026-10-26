//! Вправа 2: `println!` у тесті, що проходить — видно лише з `--nocapture`
//!
//!   cargo test -p ch17-tests --example exercise-2                  # рядка «рахую...» нема
//!   cargo test -p ch17-tests --example exercise-2 -- --nocapture   # є
//! Вивід тестів, що ПРОЙШЛИ, cargo ховає; вивід тестів, що впали, показує завжди.

use ch17_tests::calc_fee;

fn main() {
    println!("{}", calc_fee(5000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fee_for_5000() {
        println!("рахую комісію для 5000...");
        assert_eq!(calc_fee(5000), 100);
    }
}
