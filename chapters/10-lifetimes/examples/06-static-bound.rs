//! `T: 'static` виконують власні типи: String, Vec, числа, структури з String

fn keep<T: 'static>(_x: T) {}

#[derive(Debug)]
struct Order {
    id: u64,
    email: String,
}

fn main() {
    let owned = String::from("я String, і мені можна");
    keep(owned); // OK: String нічого не позичає
    keep(vec![1u8, 2, 3]);
    keep(42_u64);
    keep(Order {
        id: 1,
        email: "v@example.com".into(),
    });
    keep("літерал — &'static str"); // посилання, але на 'static дані

    let local = String::from("локальний");
    // keep(&local);   // ПОМИЛКА E0597 (broken/06-static-bound.rs)
    keep(local.clone()); // рішення: передати володіння (клон)
    println!("{local}");
}
