//! `Arc::clone(&x)` замість `x.clone()`: видно, що це дешевий «пропуск», а не копія

use std::sync::Arc;

struct Config {
    port: u16,
}

fn main() {
    let config = Arc::new(Config { port: 3000 });
    let a = Arc::clone(&config); // ще один власник, без копіювання Config
    let b = config.clone(); // те саме, але читач може подумати про дорогу копію
    println!("{} {} {}", a.port, b.port, Arc::strong_count(&config)); // 3000 3000 3
}
