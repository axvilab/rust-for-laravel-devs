//! Вправа 4: лічильник викликів усередині замикання (FnMut)
//!
//! Без `mut` перед `apply` компілятор скаже:
//!   error[E0596]: cannot borrow `apply` as mutable, as it is not declared as mutable
//!   help: consider changing this to be mutable: `mut apply`
//! (див. broken/exercise-4.rs)

fn main() {
    let mut calls = 0;
    let mut apply = |price: f64| {
        calls += 1; // змінюємо зовнішню змінну → замикання FnMut
        price * 0.9
    };
    println!("{}", apply(100.0));
    println!("{}", apply(200.0));
    println!("{}", apply(300.0));
    // Замикання тримає &mut calls, поки воно використовується; після
    // останнього виклику позичання закінчилось, і calls можна читати.
    println!("викликів: {calls}"); // 3
}
