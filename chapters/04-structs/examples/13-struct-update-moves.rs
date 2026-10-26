//! `..dev` moves fields: `u16` is copied, `String` moves

#[derive(Debug, Clone)]
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
    let prod = Config {
        debug: false,
        ..dev
    }; // host MOVED from dev into prod

    println!("{}", dev.port); // ok: the u16 was copied
    // println!("{}", dev.host);   // ERROR: borrow of moved value: `dev.host`
    println!("{prod:?}");

    // If you still need dev in one piece — ..dev.clone()
    let dev2 = Config {
        host: "localhost".into(),
        port: 8080,
        debug: true,
    };
    let staging = Config {
        port: 9090,
        ..dev2.clone()
    };
    println!("{dev2:?} {staging:?}");
}
