//! `pub use` — a re-export that hides the internal module structure

mod services {
    mod payment {
        pub struct PaymentService;
        impl PaymentService {
            pub fn name(&self) -> &'static str {
                "payment"
            }
        }
    }
    mod mail {
        pub struct Mailer;
        impl Mailer {
            pub fn name(&self) -> &'static str {
                "mail"
            }
        }
    }
    pub use mail::Mailer;
    pub use payment::PaymentService; // from outside: services::PaymentService
}

use services::{Mailer, PaymentService}; // not services::payment::PaymentService

fn main() {
    println!("{} {}", PaymentService.name(), Mailer.name());
}
