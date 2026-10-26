use crate::models::User; // crate:: — from the project root, like \App\

pub struct PaymentService;

impl PaymentService {
    pub fn new() -> Self {
        PaymentService
    }
    pub fn charge(&self, user: &User, amount: u64) {
        println!("charged {amount} to {}", user.email);
    }
}
