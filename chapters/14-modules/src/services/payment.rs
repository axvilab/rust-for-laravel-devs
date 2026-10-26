//! `services::payment`

use crate::errors::ShopError;

pub struct PaymentService; // a unit struct — "a place for methods"

impl PaymentService {
    pub fn new() -> Self {
        PaymentService
    }

    pub fn charge(&self, email: &str, amount: u64) -> Result<(), ShopError> {
        if !is_payable(amount) {
            return Err(ShopError::PaymentDeclined(format!("amount {amount}")));
        }
        println!("charged {amount} kopecks to the card of {email}");
        Ok(())
    }
}

impl Default for PaymentService {
    fn default() -> Self {
        Self::new()
    }
}

// pub(super) — visible only to the parent module `services`
pub(super) fn is_payable(amount: u64) -> bool {
    amount > 0
}
