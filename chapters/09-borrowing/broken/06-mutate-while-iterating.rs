//! You can't change a collection while iterating over it
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0502]: cannot borrow `list` as mutable because it is
//!   also borrowed as immutable
//! Fix: first collect what to add, then add it — see
//!   examples/06-collect-then-push.rs
// expected-error: E0502

fn main() {
    let mut list = vec![1, 2, 3];
    for item in &list {
        if *item == 2 {
            list.push(99); // ERROR: list is already borrowed by the loop as &
        }
    }
}
