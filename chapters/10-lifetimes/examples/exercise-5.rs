//! Вправа 5: `make_greeting` повертає `String`
//!
//! З `-> &str` і `&format!(...)` компілятор дає E0515 «cannot return reference
//! to temporary value» (broken/exercise-5.rs): рядок з format! знищується на
//! виході з функції. Жоден `'a` тут не допоможе — треба віддати сам рядок.

fn make_greeting(name: &str) -> String {
    format!("Привіт, {name}")
}

fn main() {
    let g = make_greeting("Віталій");
    println!("{g}");
}
