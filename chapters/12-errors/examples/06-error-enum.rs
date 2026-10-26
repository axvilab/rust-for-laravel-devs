//! Your own error enum: a handler does a `match` and picks the HTTP status

// In the book the third variant is `Database(sqlx::Error)`. To avoid pulling in
// sqlx, this uses its own "foreign" database error instead.
#[derive(Debug)]
struct DbError(String);

#[derive(Debug)]
enum AppError {
    NotFound,
    Validation(String),
    Database(DbError),
    Io(std::io::Error),
}

fn status(e: &AppError) -> u16 {
    match e {
        AppError::NotFound => 404,
        AppError::Validation(_) => 422,
        AppError::Database(_) | AppError::Io(_) => 500,
    }
}

fn main() {
    let errors = [
        AppError::NotFound,
        AppError::Validation("email".into()),
        AppError::Database(DbError("connection refused".into())),
        AppError::Io(std::io::Error::other("disk")),
    ];
    for e in &errors {
        println!("{} ← {e:?}", status(e));
    }
}
