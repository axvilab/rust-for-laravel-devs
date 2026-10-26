//! Shop errors (no external crates — written by hand, as in chapter 12).

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ShopError {
    EmptyOrder,
    PaymentDeclined(String),
}

impl fmt::Display for ShopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShopError::EmptyOrder => write!(f, "the order is empty"),
            ShopError::PaymentDeclined(why) => write!(f, "payment declined: {why}"),
        }
    }
}

impl std::error::Error for ShopError {}
