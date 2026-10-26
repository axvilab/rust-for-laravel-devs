//! `Result<(), Box<dyn Error>>` — "any error in a box" for main

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let port: u16 = "8080".parse()?; // ParseIntError → Box<dyn Error>
    let text = std::fs::read_to_string("Cargo.toml").unwrap_or_default(); // the io error is swallowed here
    println!("port {port}, Cargo.toml: {} bytes", text.len());
    Ok(())
}
