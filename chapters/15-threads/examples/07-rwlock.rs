//! `RwLock` — many readers or one writer

use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
struct Config {
    port: u16,
}

fn main() {
    let config = Arc::new(RwLock::new(Config::default()));

    {
        // readers don't block each other
        let cfg = config.read().unwrap();
        let cfg2 = config.read().unwrap();
        println!("{} {}", cfg.port, cfg2.port);
    } // the readers are gone — otherwise write() below would wait forever

    // the writer waits until all readers leave, and blocks everyone
    config.write().unwrap().port = 9090;
    println!("{:?}", config.read().unwrap());
}
