//! Exercise 3: `#[sqlx::test]` for `OrderRepo` — save, pay, save, read back
//!
//! The solution is in the project: orders-api/tests/repo.rs, the test `save_pay_save_find`.
//! It needs Postgres (`DATABASE_URL`), so it lives next to the repository:
//!
//!   cd orders-api && docker compose up -d db
//!   cargo test -p orders-api --test repo
//!
//! The difference from the book's snippet: in the project `Order::new(user_id, email, note)`
//! takes an `Email` (a newtype), and the status is read with the `status()` method.

fn main() {
    println!("see orders-api/tests/repo.rs");
}
