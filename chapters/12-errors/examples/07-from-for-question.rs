//! `impl From<io::Error> for AppError` — і `?` сам конвертує помилки

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
    let text = std::fs::read_to_string(path)?; // io::Error → AppError::Io через From
    if text.is_empty() {
        return Err(AppError::Validation("порожній файл".into()));
    }
    Ok(text)
}

fn main() {
    println!("{:?}", load("/немає/файлу").map(|t| t.len()));
    println!("{:?}", load("Cargo.toml").map(|t| t.len()));
}
