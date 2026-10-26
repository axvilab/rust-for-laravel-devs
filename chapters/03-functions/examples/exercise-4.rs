//! Exercise 4: a call counter inside a closure (FnMut)
//!
//! Without `mut` before `apply` the compiler says:
//!   error[E0596]: cannot borrow `apply` as mutable, as it is not declared as mutable
//!   help: consider changing this to be mutable: `mut apply`
//! (see broken/exercise-4.rs)

fn main() {
    let mut calls = 0;
    let mut apply = |price: f64| {
        calls += 1; // we change an outer variable → the closure is FnMut
        price * 0.9
    };
    println!("{}", apply(100.0));
    println!("{}", apply(200.0));
    println!("{}", apply(300.0));
    // The closure holds &mut calls while it's in use; after the last
    // call the borrow is over and calls can be read.
    println!("calls: {calls}"); // 3
}
