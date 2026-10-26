//! `RwLock` — багато читачів або один писець

use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
struct Config {
    port: u16,
}

fn main() {
    let config = Arc::new(RwLock::new(Config::default()));

    {
        // читачі не блокують один одного
        let cfg = config.read().unwrap();
        let cfg2 = config.read().unwrap();
        println!("{} {}", cfg.port, cfg2.port);
    } // читачі пішли — інакше write() нижче чекав би вічно

    // писець чекає, поки всі читачі підуть, і блокує всіх
    config.write().unwrap().port = 9090;
    println!("{:?}", config.read().unwrap());
}
