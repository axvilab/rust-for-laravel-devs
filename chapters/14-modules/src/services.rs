//! `services.rs` оголошує підмодулі з `services/` (новий стиль, без `mod.rs`).

pub mod mail; // «є services/mail.rs»
pub mod payment; // «є services/payment.rs»

// Реекспорт: зовні можна писати `services::PaymentService`,
// не знаючи про внутрішній `services::payment`.
pub use mail::Mailer;
pub use payment::PaymentService;
