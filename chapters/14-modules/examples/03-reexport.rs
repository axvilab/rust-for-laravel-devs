//! `pub use` — реекспорт, щоб сховати внутрішню структуру модулів

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
    pub use payment::PaymentService; // зовні: services::PaymentService
}

use services::{Mailer, PaymentService}; // а не services::payment::PaymentService

fn main() {
    println!("{} {}", PaymentService.name(), Mailer.name());
}
