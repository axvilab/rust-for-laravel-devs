//! Вправа 5: `anyhow` + `.context()` навколо всієї обробки в `main`
// exits-with-error: навмисно невалідні дані, щоб побачити ланцюжок причин:
//   Error: не вдалось створити користувача з рядка «Віталій;двісті»
//   Caused by:
//       0: невалідний вік: «двісті» — не число
//       ...

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum ValidationError {
    #[error("ім'я порожнє")]
    Empty,
    #[error("невалідний вік: {0}")]
    BadAge(String),
}

fn parse_age(s: &str) -> Result<u8, ValidationError> {
    s.trim()
        .parse()
        .map_err(|_| ValidationError::BadAge(format!("«{s}» — не число")))
}

fn parse_user(line: &str) -> Result<(String, u8)> {
    let (name, age) = line.split_once(';').context("очікую формат `ім'я;вік`")?;
    if name.is_empty() {
        return Err(ValidationError::Empty.into()); // thiserror-помилка → anyhow::Error
    }
    Ok((name.to_string(), parse_age(age)?)) // ? приймає будь-яку помилку
}

fn main() -> Result<()> {
    for line in ["Віталій;35", "Віталій;двісті"] {
        let user = parse_user(line)
            .with_context(|| format!("не вдалось створити користувача з рядка «{line}»"))?;
        println!("{user:?}");
    }
    Ok(())
}
