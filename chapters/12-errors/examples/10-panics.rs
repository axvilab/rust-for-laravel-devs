//! Panics are for bugs: expect, out-of-bounds index, unwrap, assert!, unreachable!, todo!
//!
//! Every function below panics on "bad" data. main catches the panics with
//! `catch_unwind` only for the demo — normal code doesn't do this.

fn item_at(items: &[i32], i: usize) -> i32 {
    items[i] // panics if there are fewer than i+1 elements
}

fn parse_strict(s: &str) -> u32 {
    s.parse().unwrap() // panics on "abc"
}

fn withdraw(balance: i64, amount: i64) -> i64 {
    let left = balance - amount;
    assert!(left >= 0, "balance cannot be negative");
    left
}

fn status_text(code: u8) -> &'static str {
    match code {
        0 => "ok",
        1 => "error",
        _ => unreachable!("we can never get here"),
    }
}

#[allow(dead_code)]
fn send_report() {
    todo!("will implement later"); // handy for stubs
}

fn main() {
    // `expect` on the config: the book has `load_config().expect(...)`; here an env var,
    // since clippy considers unwrapping a Some literal pointless (unnecessary_literal_unwrap).
    let config = std::env::var("HOME").expect("HOME is required to start");
    println!("config: {config}");
    println!(
        "{} {} {}",
        item_at(&[1, 2, 3], 1),
        parse_strict("7"),
        withdraw(10, 3)
    );
    println!("{}", status_text(0));

    std::panic::set_hook(Box::new(|info| eprintln!("  (panic: {info})")));
    // Closures without captures coerce to `fn()` — so they can go into catch_unwind.
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
        println!("{name}: panicked = {failed}");
    }
}
