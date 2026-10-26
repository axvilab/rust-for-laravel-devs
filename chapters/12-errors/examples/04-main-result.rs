//! `main` повертає `Result`: помилка друкується через `{:?}`, код виходу 1
// exits-with-error: навмисно, щоб побачити `Error: ParseIntError { kind: InvalidDigit }`

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let port: u16 = "80a".parse()?;
    println!("порт {port}");
    Ok(())
}
