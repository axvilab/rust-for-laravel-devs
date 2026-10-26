//! `move` with numbers "takes" nothing: Copy types are copied

fn main() {
    let percent = 0.1; // f64 — a Copy type
    let discount = move |p: f64| p * (1.0 - percent);
    println!("{}", discount(100.0)); // 90
    println!("{percent}"); // ok: move took a copy

    let name = String::from("Vitalii"); // String — not Copy
    let greet = move || println!("Hello, {name}");
    greet();
    // println!("{name}");              // ERROR: borrow of moved value
}
