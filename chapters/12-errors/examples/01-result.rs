//! `Result<T, E>`: a function that may fail, and a `match` on the result

fn parse_port(s: &str) -> Result<u16, String> {
    match s.parse::<u16>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err("port cannot be 0".to_string()),
        Err(_) => Err(format!("'{s}' is not a number")),
    }
}

fn main() {
    for input in ["8080", "0", "abc"] {
        match parse_port(input) {
            Ok(port) => println!("listening on {port}"),
            Err(msg) => eprintln!("error: {msg}"),
        }
    }
}
