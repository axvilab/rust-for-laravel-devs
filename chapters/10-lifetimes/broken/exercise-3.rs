//! Exercise 3: an `Excerpt` referring to a String destroyed at the end of a block
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0597]: `text` does not live long enough
//!   — borrowed value does not live long enough / `text` dropped here while still borrowed
//! That is: `e` holds a reference to `text`, and `text` is destroyed at `}` before `e`
//! is used. Fix: examples/exercise-4.rs (Excerpt with a String)
// expected-error: E0597

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let e;
    {
        let text = String::from("First sentence. Second.");
        e = Excerpt { part: &text };
    }
    println!("{}", e.part);
}
