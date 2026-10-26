//! Вправа 1: `let t = s;` — і `s` більше недоступний
//!
//! `println!("{s}")` після `let t = s;` дає error[E0382]: borrow of moved value: `s`
//! з позначками: `value moved here` — рядок `let t = s;`, і
//! `value borrowed here after move` — місце використання (див. broken/exercise-1.rs).

fn main() {
    let s = String::from("тест");
    let t = s;
    println!("{t}"); // власник тепер t
}
