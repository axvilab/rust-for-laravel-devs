//! Паніка — для багів: expect, індекс за межами, unwrap, assert!, unreachable!, todo!
//!
//! Кожна функція нижче панікує на «поганих» даних. main ловить паніки через
//! `catch_unwind` лише для демонстрації — у звичайному коді так не роблять.

fn item_at(items: &[i32], i: usize) -> i32 {
    items[i] // паніка, якщо менше i+1 елементів
}

fn parse_strict(s: &str) -> u32 {
    s.parse().unwrap() // паніка на "abc"
}

fn withdraw(balance: i64, amount: i64) -> i64 {
    let left = balance - amount;
    assert!(left >= 0, "баланс не може бути від'ємним");
    left
}

fn status_text(code: u8) -> &'static str {
    match code {
        0 => "ok",
        1 => "error",
        _ => unreachable!("сюди не можна потрапити"),
    }
}

#[allow(dead_code)]
fn send_report() {
    todo!("реалізую пізніше"); // зручно для заглушок
}

fn main() {
    // `expect` на конфіг: у книзі `load_config().expect(...)`; тут env-змінна,
    // а Some-літерал clippy вважає зайвим unwrap (unnecessary_literal_unwrap).
    let config = std::env::var("HOME").expect("HOME обов'язковий для старту");
    println!("конфіг: {config}");
    println!(
        "{} {} {}",
        item_at(&[1, 2, 3], 1),
        parse_strict("7"),
        withdraw(10, 3)
    );
    println!("{}", status_text(0));

    std::panic::set_hook(Box::new(|info| eprintln!("  (паніка: {info})")));
    // Замикання без захоплень приводяться до `fn()` — їх можна передати в catch_unwind.
    let cases: [(&str, fn()); 4] = [
        ("item_at", || {
            let _ = item_at(&[1, 2, 3], 5);
        }),
        ("parse_strict", || {
            let _ = parse_strict("abc");
        }),
        ("withdraw", || {
            let _ = withdraw(10, 30);
        }),
        ("status_text", || {
            let _ = status_text(9);
        }),
    ];
    for (name, f) in cases {
        let failed = std::panic::catch_unwind(f).is_err();
        println!("{name}: запанікувала = {failed}");
    }
}
