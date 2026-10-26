//! A closure that changes an outer variable must itself be `let mut`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0596]: cannot borrow `inc` as mutable, as it is not declared as mutable
//!   with the hint `help: consider changing this to be mutable: mut inc`
//! Fix: `let mut inc = ...` — see examples/08-fnmut-counter.rs
// expected-error: E0596

fn main() {
    let mut count = 0;
    let inc = || count += 1;
    inc();
    inc();
    println!("{count}");
}
