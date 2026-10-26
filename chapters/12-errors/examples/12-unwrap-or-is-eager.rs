//! `unwrap_or` always computes the default, `unwrap_or_else` only on `Err`

fn default_port() -> u16 {
    println!("computing the default...");
    8080
}

fn parse_port(s: &str) -> Result<u16, String> {
    s.parse().map_err(|_| format!("'{s}' is not a port"))
}

fn main() {
    // Not `let r: Result<u16, String> = Ok(80);`: on an `Ok(...)` literal clippy
    // says "unwrap_or_else on Ok — just remove it" (unnecessary_literal_unwrap),
    // so the value comes from a function, as in real code.
    let r: Result<u16, String> = parse_port("80");
    let a = r.clone().unwrap_or(default_port()); // prints "computing the default..."
    let b = r.unwrap_or_else(|_| default_port()); // prints nothing
    println!("{a} {b}");
}
