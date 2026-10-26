//! Бінарник: вся логіка в бібліотеці (її можна тестувати), тут — запуск.
//! `cargo run -p ch14-modules`

fn main() {
    if let Err(e) = ch14_modules::run() {
        eprintln!("помилка: {e}");
        std::process::exit(1);
    }
}
