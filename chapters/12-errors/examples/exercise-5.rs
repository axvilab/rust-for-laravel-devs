//! Exercise 5: `anyhow` + `.context()` around all the processing in `main`
// exits-with-error: invalid data on purpose, to show the chain of causes:
//   Error: failed to create a user from the line 'Vitalii;two hundred'
//   Caused by:
//       0: invalid age: 'two hundred' is not a number
//       ...

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum ValidationError {
    #[error("name is empty")]
    Empty,
    #[error("invalid age: {0}")]
    BadAge(String),
}

fn parse_age(s: &str) -> Result<u8, ValidationError> {
    s.trim()
        .parse()
        .map_err(|_| ValidationError::BadAge(format!("'{s}' is not a number")))
}

fn parse_user(line: &str) -> Result<(String, u8)> {
    let (name, age) = line
        .split_once(';')
        .context("expected the format `name;age`")?;
    if name.is_empty() {
        return Err(ValidationError::Empty.into()); // a thiserror error → anyhow::Error
    }
    Ok((name.to_string(), parse_age(age)?)) // ? accepts any error
}

fn main() -> Result<()> {
    for line in ["Vitalii;35", "Vitalii;two hundred"] {
        let user = parse_user(line)
            .with_context(|| format!("failed to create a user from the line '{line}'"))?;
        println!("{user:?}");
    }
    Ok(())
}
