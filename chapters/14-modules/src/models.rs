//! Моделі: дані без логіки розрахунків.

use crate::errors::ShopError; // crate:: — від кореня крейта, як \App\

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: u64, // копійки
}

impl Product {
    pub fn new(id: u64, name: &str, price: u64) -> Self {
        Product {
            id,
            name: name.to_string(),
            price,
        }
    }
}

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub lines: Vec<(Product, u32)>,
}

impl Order {
    pub fn new(id: u64, lines: Vec<(Product, u32)>) -> Result<Self, ShopError> {
        if lines.is_empty() {
            return Err(ShopError::EmptyOrder);
        }
        Ok(Order { id, lines })
    }
}
