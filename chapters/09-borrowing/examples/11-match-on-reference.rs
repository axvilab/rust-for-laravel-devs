//! `match &x` looks, `match x` takes the contents

fn main() {
    let name: Option<String> = Some("Vitalii".to_string());

    match &name {
        // we look, we don't take
        Some(n) => println!("present: {n}"), // n here is a &String
        None => println!("absent"),
    }
    println!("{name:?}"); // name is alive

    // The book's `match` has an empty `None => {}` arm. Clippy suggests
    // `if let` for that (clippy::single_match); the `match` form is kept on purpose.
    #[allow(clippy::single_match)]
    match name {
        // no & — we take the contents
        Some(n) => println!("took: {n}"), // n is a String, it moved
        None => {}
    }
    // println!("{name:?}");            // ERROR: borrow of partially moved value
}
