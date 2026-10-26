//! Розділ 14. Модулі, крейти і Cargo — як організувати проєкт
//!
//! Цей крейт сам є прикладом розділу: бібліотека (`lib.rs` + модулі) з логікою,
//! тонкий `main.rs`, окрема команда в `src/bin/`, doc-тести.
//!
//! ```text
//! src/
//! ├── lib.rs           ← корінь бібліотеки: оголошує модулі
//! ├── main.rs          ← бінарник: кілька рядків, викликає бібліотеку
//! ├── bin/report.rs    ← ще одна команда: cargo run -p ch14-modules --bin report
//! ├── models.rs        ← Product, Order
//! ├── pricing.rs       ← розрахунки
//! ├── errors.rs        ← ShopError
//! ├── services.rs      ← оголошує підмодулі + pub use
//! └── services/
//!     ├── payment.rs   ← services::payment
//!     └── mail.rs      ← services::mail
//! ```
//!
//! Модульна структура «як у книзі» з `mod` у `main.rs` — приклад
//! `examples/01-modules/` (`cargo run -p ch14-modules --example 01-modules`).

pub mod errors;
pub mod models;
pub mod pricing;
pub mod services;

/// Нараховує комісію за платіж: 2% (сума в копійках).
///
/// Doc-тест — окрема програма, яка підключає крейт ЗЗОВНІ, тому потрібен `use`:
///
/// ```
/// use ch14_modules::calc_fee;
///
/// assert_eq!(calc_fee(1000), 20);
/// ```
///
/// Без `use` той самий приклад не скомпілюється — `compile_fail` перевіряє саме це
/// (error[E0425]: cannot find function `calc_fee` in this scope):
///
/// ```compile_fail,E0425
/// let fee = calc_fee(1000);
/// assert_eq!(fee, 20);
/// ```
pub fn calc_fee(amount: u64) -> u64 {
    amount / 50
}

/// Точка входу застосунку: `main.rs` лише викликає її.
pub fn run() -> Result<(), errors::ShopError> {
    use models::{Order, Product};
    use services::{Mailer, PaymentService};

    let laptop = Product::new(1, "Ноутбук", 4_500_000);
    let order = Order::new(7, vec![(laptop, 1)])?;
    let total = pricing::total_with_fee(&order);

    PaymentService::new().charge("v@example.com", total)?;
    Mailer::new().send(
        "v@example.com",
        &format!("Замовлення #{} оплачено", order.id),
    );
    Ok(())
}
