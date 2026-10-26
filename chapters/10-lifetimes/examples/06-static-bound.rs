//! `T: 'static` is satisfied by owned types: String, Vec, numbers, structs with String

fn keep<T: 'static>(_x: T) {}

#[derive(Debug)]
struct Order {
    id: u64,
    email: String,
}

fn main() {
    let owned = String::from("I'm a String, I'm allowed");
    keep(owned); // OK: a String borrows nothing
    keep(vec![1u8, 2, 3]);
    keep(42_u64);
    keep(Order {
        id: 1,
        email: "v@example.com".into(),
    });
    keep("a literal — &'static str"); // a reference, but to 'static data

    let local = String::from("local");
    // keep(&local);   // ERROR E0597 (broken/06-static-bound.rs)
    keep(local.clone()); // the fix: pass ownership (a clone)
    println!("{local}");
}
