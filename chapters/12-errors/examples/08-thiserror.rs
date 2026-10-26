//! thiserror: `#[error]` генерує Display, `#[from]` — impl From

use thiserror::Error;

// Замість `sqlx::Error` з книги — своя помилка бази (теж через thiserror).
#[derive(Debug, Error)]
#[error("db: {0}")]
struct DbError(String);

#[derive(Debug, Error)]
enum AppError {
    #[error("запис не знайдено")]
    NotFound,
    #[error("невалідні дані: {0}")]
    Validation(String),
    #[error("помилка бази даних")]
    Database(#[from] DbError), // #[from] генерує impl From
    #[error("помилка вводу-виводу")]
    Io(#[from] std::io::Error),
}

fn query() -> Result<(), DbError> {
    Err(DbError("connection refused".into()))
}

fn handler() -> Result<(), AppError> {
    query()?; // DbError → AppError::Database
    Ok(())
}

fn main() {
    let errors = [
        AppError::NotFound,
        AppError::Validation("email".into()),
        handler().unwrap_err(),
        AppError::from(std::io::Error::other("disk")),
    ];
    for e in &errors {
        println!("{e}"); // Display з #[error(...)]
    }
}
