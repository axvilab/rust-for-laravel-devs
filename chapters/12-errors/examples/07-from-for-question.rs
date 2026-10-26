//! `impl From<io::Error> for AppError` — and `?` converts errors by itself

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Validation(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

fn load(path: &str) -> Result<String, AppError> {
    let text = std::fs::read_to_string(path)?; // io::Error → AppError::Io via From
    if text.is_empty() {
        return Err(AppError::Validation("empty file".into()));
    }
    Ok(text)
}

fn main() {
    println!("{:?}", load("/no/file").map(|t| t.len()));
    println!("{:?}", load("Cargo.toml").map(|t| t.len()));
}
