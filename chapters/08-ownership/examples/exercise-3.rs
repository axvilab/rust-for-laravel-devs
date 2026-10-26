//! Exercise 3: `shout(String)` takes the string — two ways to fix it
//!
//! Without fixes: `shout(s); println!("{s}")` → E0382 (broken/exercise-3.rs).

fn shout(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    // Way 1: "give ownership back" — the result of shout is the new owner
    let s = String::from("hello");
    let s = shout(s);
    println!("{s}"); // HELLO

    // Way 2: give away a copy, the original stays
    let original = String::from("hello");
    let loud = shout(original.clone());
    println!("{original} → {loud}");

    // (Way 3, from the next chapter: fn shout(s: &str) -> String.)
}
