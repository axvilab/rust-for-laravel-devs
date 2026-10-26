//! Вправа 3: `std::thread::sleep` у `join!` — знову послідовно
//!
//! Чому: `join!` крутить три futures в ОДНІЙ задачі на одному потоці, по черзі
//! опитуючи їх. `tokio::time::sleep` віддає керування («я чекаю, займись іншими»),
//! а `std::thread::sleep` блокує сам потік — інші futures чекають, поки він прокинеться.

use std::time::{Duration, Instant};

async fn slow_blocking(name: &str, ms: u64) -> String {
    std::thread::sleep(Duration::from_millis(ms)); // НЕ робіть так в async-коді
    name.to_string()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        slow_blocking("a", 200),
        slow_blocking("b", 200),
        slow_blocking("c", 200)
    );
    println!("{a}{b}{c} за {:?}", start.elapsed()); // ~600 мс, а не 200
}
