//! `Result<(), Box<dyn Error>>` — «будь-яка помилка в коробці» для main

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let port: u16 = "8080".parse()?; // ParseIntError → Box<dyn Error>
    let text = std::fs::read_to_string("Cargo.toml").unwrap_or_default(); // io-помилку тут гасимо
    println!("порт {port}, Cargo.toml: {} байт", text.len());
    Ok(())
}
