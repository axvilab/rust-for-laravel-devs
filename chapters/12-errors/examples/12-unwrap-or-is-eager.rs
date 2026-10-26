//! `unwrap_or` рахує дефолт завжди, `unwrap_or_else` — лише при `Err`

fn default_port() -> u16 {
    println!("рахую дефолт...");
    8080
}

fn parse_port(s: &str) -> Result<u16, String> {
    s.parse().map_err(|_| format!("«{s}» — не порт"))
}

fn main() {
    // У книзі `let r: Result<u16, String> = Ok(80);`. На літералі `Ok(...)` clippy
    // скаже «unwrap_or_else на Ok — просто прибери» (unnecessary_literal_unwrap),
    // тому значення приходить із функції, як у справжньому коді.
    let r: Result<u16, String> = parse_port("80");
    let a = r.clone().unwrap_or(default_port()); // надрукує «рахую дефолт...»
    let b = r.unwrap_or_else(|_| default_port()); // нічого не надрукує
    println!("{a} {b}");
}
