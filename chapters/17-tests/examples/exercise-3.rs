//! Вправа 3: `#[sqlx::test]` для `OrderRepo` — зберегти, оплатити, зберегти, прочитати
//!
//! Розв'язок у проєкті: orders-api/tests/repo.rs, тест `save_pay_save_find`.
//! Йому потрібен Postgres (`DATABASE_URL`), тому він живе поруч із репозиторієм:
//!
//!   cd orders-api && docker compose up -d db
//!   cargo test -p orders-api --test repo
//!
//! Відмінність від фрагмента в книзі: у проєкті `Order::new(user_id, email, note)`
//! приймає `Email` (newtype), а статус читається методом `status()`.

fn main() {
    println!("див. orders-api/tests/repo.rs");
}
