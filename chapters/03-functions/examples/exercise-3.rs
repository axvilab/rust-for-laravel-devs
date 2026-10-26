//! Exercise 3: `move` with a number breaks nothing, with a `String` it does
//!
//! `percent` is an f64, a `Copy` type: `move` took a copy, the original lives on.
//! `label` is a String, not `Copy`: `move` took the string itself. Printing
//! `label` after the closure gives error[E0382]: borrow of moved value: `label`
//! (see broken/exercise-3.rs). The ownership chapter explains it fully.

fn main() {
    let percent = 0.1;
    let discount = move |price: f64| price * (1.0 - percent);
    println!("{}", discount(200.0)); // 180
    println!("percent is still available: {percent}");

    let label = String::from("discount");
    let show = move || println!("{label}");
    show();
    // println!("{label}"); // ERROR E0382
}
