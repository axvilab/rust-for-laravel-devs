//! Вправа 3: `ValidationError` + `impl From<String>`, щоб `?` з `parse_age` працював

#[derive(Debug)]
enum ValidationError {
    Empty,
    TooLong(usize),
    BadAge(String),
}

// parse_age лишилась із String-помилкою; From вчить `?` перетворювати її.
impl From<String> for ValidationError {
    fn from(msg: String) -> Self {
        ValidationError::BadAge(msg)
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn parse_age(s: &str) -> Result<u8, String> {
    let n: u8 = s.trim().parse().map_err(|_| format!("«{s}» — не вік"))?;
    if n > 150 {
        return Err(format!("{n} — забагато"));
    }
    Ok(n)
}

fn parse_user(name: &str, age: &str) -> Result<User, ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::Empty);
    }
    let len = name.chars().count();
    if len > 50 {
        return Err(ValidationError::TooLong(len));
    }
    let age = parse_age(age)?; // String → ValidationError::BadAge через From
    Ok(User {
        name: name.to_string(),
        age,
    })
}

fn main() {
    println!("{:?}", parse_user("Віталій", "35"));
    println!("{:?}", parse_user("", "35"));
    println!("{:?}", parse_user(&"я".repeat(60), "35"));
    println!("{:?}", parse_user("Віталій", "x"));
}
