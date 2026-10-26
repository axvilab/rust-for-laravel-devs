//! `services::payment`

use crate::errors::ShopError;

pub struct PaymentService; // одинична структура — «місце для методів»

impl PaymentService {
    pub fn new() -> Self {
        PaymentService
    }

    pub fn charge(&self, email: &str, amount: u64) -> Result<(), ShopError> {
        if !is_payable(amount) {
            return Err(ShopError::PaymentDeclined(format!("сума {amount}")));
        }
        println!("списано {amount} коп. з картки {email}");
        Ok(())
    }
}

impl Default for PaymentService {
    fn default() -> Self {
        Self::new()
    }
}

// pub(super) — видно лише батьківському модулю `services`
pub(super) fn is_payable(amount: u64) -> bool {
    amount > 0
}
