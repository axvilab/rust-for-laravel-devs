//! An integration test: a separate crate that sees only the library's public API.
//! The package `ch17-tests` is `ch17_tests` in code (hyphen → underscore).

use ch17_tests::{PaymentService, fetch_user};

#[test]
fn charges_user() {
    let svc = PaymentService::new();
    assert!(svc.charge(1, 500).is_ok());
}

#[test]
fn rejects_zero_amount() {
    let svc = PaymentService::new();
    assert!(svc.charge(1, 0).is_err());
}

#[tokio::test]
async fn fetches_user_through_public_api() {
    let user = fetch_user(7).await.unwrap();
    assert_eq!(user.name, "user7");
}
