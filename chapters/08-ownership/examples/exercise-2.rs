//! Exercise 2: no error with a number — `i32` implements `Copy`
//!
//! `let m = n;` for an i32 copies 4 bytes on the stack; ownership doesn't come up,
//! the two variables are independent values.

fn main() {
    let n = 42;
    let m = n;
    println!("{n} {m}"); // 42 42
}
