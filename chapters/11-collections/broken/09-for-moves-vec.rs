//! `for s in v` without `&` takes the vector
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0382]: borrow of moved value: `v`
//!   (`v` moved due to this implicit call to `.into_iter()`)
//! Fix: `for s in &v` — see examples/09-into-iter-on-ref.rs
// expected-error: E0382

fn main() {
    let v = vec![String::from("a"), String::from("b")];
    for s in v {
        println!("{s}");
    }
    println!("{v:?}");
}
