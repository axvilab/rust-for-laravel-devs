//! Modules as in the book: `mod` in main.rs, files models.rs, services.rs, services/*.rs
//!
//! A multi-file example: Cargo builds `examples/01-modules/main.rs`, and `mod`
//! looks for modules next to it, exactly as in `src/`.

mod models; // "there's a file models.rs, include it as the models module"
mod services;

use models::User;
use services::payment::PaymentService;

fn main() {
    let svc = PaymentService::new();
    let user = User {
        email: "v@example.com".into(),
    };
    svc.charge(&user, 1500);
    services::mail::notify(&user);
}
