//! A generic without a bound: the compiler doesn't know whether `T` can be compared
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0369]: binary operation `>` cannot be applied to type `&T`
//!   help: consider restricting type parameter `T` with trait `PartialOrd`
//! Fix: `fn largest<T: PartialOrd>` — see examples/02-largest.rs
// expected-error: E0369

fn largest<T>(items: &[T]) -> &T {
    let mut max = &items[0];
    for item in items {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    println!("{}", largest(&[3, 7, 2]));
}
