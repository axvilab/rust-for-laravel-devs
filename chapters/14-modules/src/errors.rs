//! Помилки магазину (без зовнішніх крейтів — руками, як у розділі 12).

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ShopError {
    EmptyOrder,
    PaymentDeclined(String),
}

impl fmt::Display for ShopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopError::EmptyOrder => write!(f, "замовлення порожнє"),
            ShopError::PaymentDeclined(why) => write!(f, "платіж відхилено: {why}"),
        }
    }
}

impl std::error::Error for ShopError {}
