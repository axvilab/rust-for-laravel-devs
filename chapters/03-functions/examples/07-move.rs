//! `move` — the closure takes ownership of what it captures

fn main() {
    let name = String::from("Vitalii");
    let greet = move || println!("Hello, {name}");
    greet();
    // println!("{name}");   // ERROR: name moved into the closure (broken/07-use-after-move.rs)

    // If you still need the original, give the closure a copy:
    let city = String::from("Dnipro");
    let city_for_closure = city.clone();
    let show = move || println!("City: {city_for_closure}");
    show();
    println!("{city}"); // ok
}
