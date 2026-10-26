//! `services.rs` declares the submodules from `services/` (the new style, no `mod.rs`).

pub mod mail; // "there's services/mail.rs"
pub mod payment; // "there's services/payment.rs"

// A re-export: outside code can write `services::PaymentService`
// without knowing about the internal `services::payment`.
pub use mail::Mailer;
pub use payment::PaymentService;
