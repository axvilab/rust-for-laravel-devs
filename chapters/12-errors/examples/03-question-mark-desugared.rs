//! What `?` does for you: the same code with `match`

use std::io;

// clippy suggests replacing this match with `?` (question_mark) — which is the chapter's point.
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
    println!("{}", read("/nope").is_err());
    println!("{}", read_short("/nope").is_err());
}
