//! Поле `String` переїхало через `..dev` — `dev.host` більше не використати
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: borrow of moved value: `dev.host`
//! Виправлення: `..dev.clone()` — див. examples/13-struct-update-moves.rs
// expected-error: E0382

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn main() {
    let dev = Config {
        host: "localhost".into(),
        port: 8080,
        debug: true,
    };
    let prod = Config { debug: false, ..dev };
    println!("{}", dev.host);
    println!("{prod:?}");
}
