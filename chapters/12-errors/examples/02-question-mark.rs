//! The `?` operator: on `Err` return it from the function, on `Ok` keep going

use std::io;

#[derive(Debug)]
struct Config {
    port: u16,
}

// A tiny parser for `port = 8080`. Returns io::Error so that both `?` in
// load_config have the same error type (as in the book).
fn parse(text: &str) -> Result<Config, io::Error> {
    let value = text
        .trim()
        .strip_prefix("port")
        .and_then(|rest| rest.trim_start().strip_prefix('='))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "expected `port = N`"))?;
    let port = value
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Config { port })
}

fn load_config(path: &str) -> Result<Config, io::Error> {
    let text = std::fs::read_to_string(path)?; // on Err — return it from the function
    let cfg = parse(&text)?; // the same
    Ok(cfg)
}

fn main() {
    let path = std::env::temp_dir().join("ch12-config.txt");
    std::fs::write(&path, "port = 8080").unwrap();
    println!("{:?}", load_config(path.to_str().unwrap())); // Ok(Config { port: 8080 })
    println!("{:?}", load_config("/no/such/file")); // Err(Os { code: 2, kind: NotFound, .. })
}
