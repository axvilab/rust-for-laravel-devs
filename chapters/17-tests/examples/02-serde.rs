//! Serde: `from_str` / `to_string` і атрибути default, rename, skip_serializing

use ch17_tests::CreateUserRequest;

fn main() -> Result<(), serde_json::Error> {
    let json =
        r#"{"name":"Віталій","email":"v@example.com","phoneNumber":null,"password":"secret"}"#;
    let req: CreateUserRequest = serde_json::from_str(json)?;
    println!("{req:?}"); // newsletter: false — з #[serde(default)]

    let out = serde_json::to_string(&req)?;
    println!("{out}"); // без password, з phoneNumber

    let bad = r#"{"name":"x","email":"x","password":"x","newsletter":"так"}"#;
    println!(
        "{:?}",
        serde_json::from_str::<CreateUserRequest>(bad).map(|_| ())
    ); // Err: не bool
    Ok(())
}
