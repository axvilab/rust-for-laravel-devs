//! Exercise 2: `parse_user` propagates the `parse_age` error upward with `?`

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn parse_age(s: &str) -> Result<u8, String> {
    let n: u8 = s
        .trim()
        .parse()
        .map_err(|_| format!("'{s}' is not an age"))?;
    if n > 150 {
        return Err(format!("{n} is too much"));
    }
    Ok(n)
}

fn parse_user(name: &str, age: &str) -> Result<User, String> {
    let age = parse_age(age)?; // Err goes up as is — no extra code
    Ok(User {
        name: name.to_string(),
        age,
    })
}

fn main() {
    println!("{:?}", parse_user("Vitalii", "35"));
    println!("{:?}", parse_user("Vitalii", "forty"));
}
