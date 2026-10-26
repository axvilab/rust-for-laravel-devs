//! `move` з числами нічого не «забирає»: Copy-типи копіюються

fn main() {
    let percent = 0.1; // f64 — Copy-тип
    let discount = move |p: f64| p * (1.0 - percent);
    println!("{}", discount(100.0)); // 90
    println!("{percent}"); // ok: move забрав копію

    let name = String::from("Віталій"); // String — не Copy
    let greet = move || println!("Привіт, {name}");
    greet();
    // println!("{name}");              // ПОМИЛКА: borrow of moved value
}
