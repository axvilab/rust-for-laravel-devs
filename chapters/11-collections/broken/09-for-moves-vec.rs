//! `for s in v` без `&` забирає вектор
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `v`
//!   (`v` moved due to this implicit call to `.into_iter()`)
//! Виправлення: `for s in &v` — див. examples/09-into-iter-on-ref.rs
// expected-error: E0382

fn main() {
    let v = vec![String::from("a"), String::from("b")];
    for s in v {
        println!("{s}");
    }
    println!("{v:?}");
}
