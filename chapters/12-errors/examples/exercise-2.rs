//! Вправа 2: `parse_user` прокидає помилку `parse_age` нагору через `?`

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

fn parse_user(name: &str, age: &str) -> Result<User, String> {
    let age = parse_age(age)?; // Err піде нагору як є — жодного додаткового коду
    Ok(User {
        name: name.to_string(),
        age,
    })
}

fn main() {
    println!("{:?}", parse_user("Віталій", "35"));
    println!("{:?}", parse_user("Віталій", "сорок"));
}
