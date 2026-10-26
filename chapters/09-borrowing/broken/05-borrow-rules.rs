//! A reader and an editor at the same time
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0502]: cannot borrow `list` as immutable because it is
//!   also borrowed as mutable
//! Fix: reorder the lines (the last use of `c` before `&list`) —
//!   see examples/05-borrow-rules.rs
// expected-error: E0502

fn main() {
    let mut list = vec![1, 2, 3];
    let c = &mut list;
    c.push(4);
    let d = &list; // ERROR: c is still used below
    c.push(5);
    println!("{d:?}");
}
