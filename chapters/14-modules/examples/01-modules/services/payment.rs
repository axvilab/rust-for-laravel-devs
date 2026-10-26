use crate::models::User; // crate:: — від кореня проєкту, як \App\

pub struct PaymentService;

impl PaymentService {
    pub fn new() -> Self {
        PaymentService
    }
    pub fn charge(&self, user: &User, amount: u64) {
        println!("списано {amount} з {}", user.email);
    }
}
