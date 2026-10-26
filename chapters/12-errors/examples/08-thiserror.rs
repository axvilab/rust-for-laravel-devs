//! thiserror: `#[error]` generates Display, `#[from]` — impl From

use thiserror::Error;

// Instead of the book's `sqlx::Error` — our own database error (also via thiserror).
#[derive(Debug, Error)]
#[error("db: {0}")]
struct DbError(String);

#[derive(Debug, Error)]
enum AppError {
    #[error("record not found")]
    NotFound,
    #[error("invalid data: {0}")]
    Validation(String),
    #[error("database error")]
    Database(#[from] DbError), // #[from] generates impl From
    #[error("I/O error")]
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
        println!("{e}"); // Display from #[error(...)]
    }
}
