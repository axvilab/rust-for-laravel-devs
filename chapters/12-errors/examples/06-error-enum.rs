//! Власний enum помилок: хендлер робить `match` і вибирає HTTP-код

// У книзі третій варіант — `Database(sqlx::Error)`. Щоб приклад не тягнув
// sqlx, тут замість нього своя «чужа» помилка бази.
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
