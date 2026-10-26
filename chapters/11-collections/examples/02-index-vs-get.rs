//! `[i]` panics out of bounds, `.get(i)` returns `Option<&T>`; `.copied()` removes the `&`
#![allow(clippy::useless_vec)] // the point is Vec, although an array would do here too

fn main() {
    let names = vec!["Ivan", "Maria"];

    // For index 0 clippy suggests `.first()` instead of `.get(0)` (get_first),
    // so this uses the second element.
    let maybe: Option<&&str> = names.get(1); // a reference to an element (&str) → &&str
    let name: Option<&str> = names.get(1).copied(); // for Copy types — copied()
    println!("{maybe:?} {name:?}");

    let idx = 10; // "came from outside"
    match names.get(idx) {
        Some(n) => println!("{n}"),
        None => println!("no element {idx}"),
    }
    // println!("{}", names[idx]); // panic: index out of bounds: the len is 2 but the index is 10

    let owned = vec![String::from("a")];
    let first: Option<String> = owned.first().cloned(); // for String — cloned()
    println!("{first:?}");
}
