//! Модулі як у книзі: `mod` у main.rs, файли models.rs, services.rs, services/*.rs
//!
//! Багатофайловий приклад: Cargo збирає `examples/01-modules/main.rs`, а `mod`
//! шукає модулі поруч, рівно як у `src/`.

mod models; // «є файл models.rs, підключи його як модуль models»
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
