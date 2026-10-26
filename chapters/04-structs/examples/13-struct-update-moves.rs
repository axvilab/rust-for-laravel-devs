//! `..dev` переносить поля: `u16` копіюється, `String` переїжджає

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
    }; // host ПЕРЕЇХАВ із dev у prod

    println!("{}", dev.port); // ok: u16 скопіювався
    // println!("{}", dev.host);   // ПОМИЛКА: borrow of moved value: `dev.host`
    println!("{prod:?}");

    // Якщо dev ще потрібен цілим — ..dev.clone()
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
