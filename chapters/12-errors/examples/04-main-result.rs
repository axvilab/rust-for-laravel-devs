//! `main` returns a `Result`: the error is printed with `{:?}`, exit code 1
// exits-with-error: on purpose, to show `Error: ParseIntError { kind: InvalidDigit }`

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let port: u16 = "80a".parse()?;
    println!("port {port}");
    Ok(())
}
