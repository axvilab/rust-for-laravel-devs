//! Що `?` робить за тебе: той самий код через `match`

use std::io;

// clippy радить замінити цей match на `?` (question_mark) — саме це і показує розділ.
#[allow(clippy::question_mark)]
fn read(path: &str) -> Result<String, io::Error> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    Ok(text)
}

fn read_short(path: &str) -> Result<String, io::Error> {
    let text = std::fs::read_to_string(path)?;
    Ok(text)
}

fn main() {
    println!("{}", read("/немає").is_err());
    println!("{}", read_short("/немає").is_err());
}
