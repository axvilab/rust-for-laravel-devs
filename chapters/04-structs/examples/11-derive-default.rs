//! `derive(Default)` and `..Default::default()` / `..another_instance`

#[derive(Debug, Default)]
struct Config {
    host: String, // "" by default
    port: u16,    // 0
    debug: bool,  // false
}

fn main() {
    let cfg = Config {
        port: 8080,
        ..Default::default()
    };
    println!("{cfg:?}");

    let dev_config = Config {
        host: "localhost".into(),
        port: 8080,
        debug: true,
    };
    let prod = Config {
        debug: false,
        ..dev_config
    };
    println!("{prod:?}");
}
