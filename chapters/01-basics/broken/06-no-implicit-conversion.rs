//! No implicit conversions: i32 + f64 does not compile
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0277]: cannot add `f64` to `i32`
//! Fix: an explicit conversion `x as f64 + y` — see examples/06-as-cast.rs
// expected-error: E0277

fn main() {
    let x: i32 = 5;
    let y: f64 = 2.5;
    let z = x + y; // ERROR: cannot add f64 to i32
    println!("{z}");
}
