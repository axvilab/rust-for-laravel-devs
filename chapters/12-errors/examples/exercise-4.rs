//! Вправа 4: той самий `ValidationError` на thiserror; вивід через `{}`

use thiserror::Error;

#[derive(Debug, Error)]
enum ValidationError {
    #[error("ім'я порожнє")]
    Empty,
    #[error("ім'я задовге: {0} символів (максимум 50)")]
    TooLong(usize),
    #[error("невалідний вік: {0}")]
    BadAge(String),
}

impl From<String> for ValidationError {
    fn from(msg: String) -> Self {
        ValidationError::BadAge(msg)
    }
}

fn parse_age(s: &str) -> Result<u8, String> {
    let n: u8 = s.trim().parse().map_err(|_| format!("«{s}» — не число"))?;
    if n > 150 {
        return Err(format!("{n} — забагато"));
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
    for (n, a) in [("Віталій", "35"), ("", "1"), ("Віталій", "двісті")] {
        match parse_user(n, a) {
            Ok(u) => println!("ok: {u:?}"),
            Err(e) => println!("помилка: {e}"), // текст з #[error(...)]
        }
    }
}
