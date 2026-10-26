//! Serde: `from_str` / `to_string` and the default, rename, skip_serializing attributes

use ch17_tests::CreateUserRequest;

fn main() -> Result<(), serde_json::Error> {
    let json =
        r#"{"name":"Vitalii","email":"v@example.com","phoneNumber":null,"password":"secret"}"#;
    let req: CreateUserRequest = serde_json::from_str(json)?;
    println!("{req:?}"); // newsletter: false — from #[serde(default)]

    let out = serde_json::to_string(&req)?;
    println!("{out}"); // no password, with phoneNumber

    let bad = r#"{"name":"x","email":"x","password":"x","newsletter":"yes"}"#;
    println!(
        "{:?}",
        serde_json::from_str::<CreateUserRequest>(bad).map(|_| ())
    ); // Err: not a bool
    Ok(())
}
