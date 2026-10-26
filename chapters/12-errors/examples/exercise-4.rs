//! Exercise 4: the same `ValidationError` with thiserror; printed with `{}`

use thiserror::Error;

#[derive(Debug, Error)]
enum ValidationError {
    #[error("name is empty")]
    Empty,
    #[error("name is too long: {0} characters (max 50)")]
    TooLong(usize),
    #[error("invalid age: {0}")]
    BadAge(String),
}

impl From<String> for ValidationError {
    fn from(msg: String) -> Self {
        ValidationError::BadAge(msg)
    }
}

fn parse_age(s: &str) -> Result<u8, String> {
    let n: u8 = s
        .trim()
        .parse()
        .map_err(|_| format!("'{s}' is not a number"))?;
    if n > 150 {
        return Err(format!("{n} is too much"));
    }
    Ok(n)
}

fn parse_user(name: &str, age: &str) -> Result<(String, u8), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::Empty);
    }
    let len = name.chars().count();
    if len > 50 {
        return Err(ValidationError::TooLong(len));
    }
    Ok((name.to_string(), parse_age(age)?))
}

fn main() {
    for (n, a) in [("Vitalii", "35"), ("", "1"), ("Vitalii", "two hundred")] {
        match parse_user(n, a) {
            Ok(u) => println!("ok: {u:?}"),
            Err(e) => println!("error: {e}"), // the text from #[error(...)]
        }
    }
}
