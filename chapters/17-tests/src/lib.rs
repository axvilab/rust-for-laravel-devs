//! Chapter 17. Tests, macros, and what's next
//!
//! The chapter's code here is mostly TESTS: `cargo test -p ch17-tests` runs
//! the unit tests in this file, the integration tests in `tests/`, the doc tests in `///`
//! and the tests in the exercise examples. Standalone programs are in `examples/`.

use serde::{Deserialize, Serialize};

/// A fee: zero up to 100, then 2%.
///
/// ```
/// use ch17_tests::calc_fee;
///
/// assert_eq!(calc_fee(50), 0);
/// assert_eq!(calc_fee(1000), 20);
/// ```
pub fn calc_fee(amount: u64) -> u64 {
    if amount < 100 { 0 } else { amount / 50 }
}

/// Division that panics on zero (for `#[should_panic]`).
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }
    a / b
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub port: u16,
}

#[derive(Debug, PartialEq)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid config: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

/// Parses the line `port = 8080`.
pub fn parse(text: &str) -> Result<Config, ParseError> {
    let (key, value) = text
        .split_once('=')
        .ok_or_else(|| ParseError("no `=`".into()))?;
    if key.trim() != "port" {
        return Err(ParseError(format!("unknown key `{}`", key.trim())));
    }
    let port = value
        .trim()
        .parse()
        .map_err(|_| ParseError(format!("`{}` is not a port", value.trim())))?;
    Ok(Config { port })
}

/// A service for the integration test in `tests/api_test.rs`.
pub struct PaymentService;

impl PaymentService {
    pub fn new() -> Self {
        PaymentService
    }

    /// Charge `amount` kopecks to the user `user_id`.
    pub fn charge(&self, user_id: u64, amount: u64) -> Result<u64, String> {
        if amount == 0 {
            return Err("the amount must be greater than zero".into());
        }
        Ok(user_id * 1_000_000 + amount) // "a transaction id"
    }
}

impl Default for PaymentService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
}

/// An async "repository" for `#[tokio::test]`.
pub async fn fetch_user(id: u64) -> Result<User, String> {
    tokio::task::yield_now().await; // an imaginary database query
    match id {
        0 => Err("user not found".into()),
        _ => Ok(User {
            id,
            name: format!("user{id}"),
        }),
    }
}

/// A create-user request: the serde attributes from the chapter.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    #[serde(default)] // missing in the JSON — Default
    pub newsletter: bool,
    #[serde(rename = "phoneNumber")] // camelCase outside, snake_case inside
    pub phone: Option<String>,
    #[serde(skip_serializing)] // never send it out
    pub password: String,
}

/// A `HashMap` in one expression.
///
/// ```
/// let stock = ch17_tests::hashmap! { "laptop" => 5, "mouse" => 40 };
/// assert_eq!(stock["mouse"], 40);
/// ```
#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

/// Exercise 4: `hashset!` modelled on `hashmap!`.
///
/// ```
/// let tags = ch17_tests::hashset!["rust", "php", "rust"];
/// assert_eq!(tags.len(), 2);
/// ```
#[macro_export]
macro_rules! hashset {
    ($($v:expr),* $(,)?) => {{
        let mut s = std::collections::HashSet::new();
        $( s.insert($v); )*
        s
    }};
}

#[cfg(test)] // this module is compiled only for cargo test
mod tests {
    use super::*; // import everything from the parent module

    #[test]
    fn no_fee_for_small_amounts() {
        assert_eq!(calc_fee(50), 0);
    }

    #[test]
    fn two_percent_fee() {
        assert_eq!(calc_fee(1000), 20);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn panics_on_zero() {
        divide(1, 0);
    }

    // A test can return a Result — then `?` works in it
    #[test]
    fn parses_config() -> Result<(), Box<dyn std::error::Error>> {
        let cfg = parse("port = 8080")?;
        assert_eq!(cfg.port, 8080);
        Ok(())
    }

    #[test]
    fn rejects_bad_config() {
        assert_eq!(
            parse("port = abc"),
            Err(ParseError("`abc` is not a port".into()))
        );
        assert!(parse("host = x").is_err());
    }

    #[test]
    fn assert_macros() {
        assert!(calc_fee(5000) > 0);
        assert_ne!(calc_fee(5000), calc_fee(50));
        assert_eq!(
            divide(10, 3),
            3,
            "integer division drops the fractional part"
        );
    }

    #[tokio::test]
    async fn fetches_user() {
        let user = fetch_user(1).await.unwrap();
        assert_eq!(user.id, 1);
    }

    #[tokio::test]
    async fn missing_user_is_an_error() {
        assert!(fetch_user(0).await.is_err());
    }

    // #[tokio::test] is single-threaded by default; blocking code in it
    // may hang. multi_thread gives a real thread pool.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn spawn_blocking_on_multi_thread_runtime() {
        let n = tokio::task::spawn_blocking(|| (1..=100u64).sum::<u64>())
            .await
            .unwrap();
        assert_eq!(n, 5050);
    }

    #[test]
    fn serde_roundtrip() {
        let json =
            r#"{"name":"Vitalii","email":"v@example.com","phoneNumber":"+380","password":"x"}"#;
        let req: CreateUserRequest = serde_json::from_str(json).unwrap();
        assert!(!req.newsletter); // #[serde(default)]
        assert_eq!(req.phone.as_deref(), Some("+380")); // rename
        let out = serde_json::to_string(&req).unwrap();
        assert!(!out.contains("password")); // skip_serializing
        assert!(out.contains("phoneNumber"));

        let bad = r#"{"name":1,"email":"x","password":"x"}"#;
        assert!(serde_json::from_str::<CreateUserRequest>(bad).is_err()); // wrong type — Err
    }

    #[test]
    fn hashmap_and_hashset_macros() {
        let stock = hashmap! { "laptop" => 5, "mouse" => 40, };
        assert_eq!(stock.len(), 2);
        let tags = hashset![1, 2, 2, 3];
        assert_eq!(tags.len(), 3);
    }
}
