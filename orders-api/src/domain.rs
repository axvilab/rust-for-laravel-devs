//! Domain: orders, items, status state machine.
//!
//! This module knows nothing about HTTP or the database: there is no `use axum`
//! and no `use sqlx` here. The repository maps rows into these types, and the
//! handlers map them into JSON responses.

use std::fmt;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Draft,
    Paid,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Draft => "draft",
            OrderStatus::Paid => "paid",
            OrderStatus::Cancelled => "cancelled",
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    #[error("замовлення вже {0}, змінювати не можна")]
    NotEditable(OrderStatus),
    #[error("не можна оплатити порожнє замовлення")]
    Empty,
    #[error("кількість має бути більше нуля")]
    BadQty,
    #[error("ціна не може бути від'ємною")]
    BadPrice,
    #[error("невалідний email")]
    InvalidEmail,
}

/// An email that is valid by construction (newtype + `TryFrom`).
///
/// `#[serde(try_from = "String")]` makes deserialization go through `TryFrom`,
/// so an invalid email never reaches a handler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(s: String) -> Result<Self, DomainError> {
        let s = s.trim().to_lowercase();
        let valid = s.len() <= 254
            && !s.chars().any(char::is_whitespace)
            && match s.split_once('@') {
                Some((local, domain)) => {
                    !local.is_empty()
                        && !domain.contains('@')
                        && domain.contains('.')
                        && !domain.starts_with('.')
                        && !domain.ends_with('.')
                }
                None => false,
            };
        if valid {
            Ok(Email(s))
        } else {
            Err(DomainError::InvalidEmail)
        }
    }
}

impl TryFrom<&str> for Email {
    type Error = DomainError;

    fn try_from(s: &str) -> Result<Self, DomainError> {
        Email::try_from(s.to_owned())
    }
}

impl From<Email> for String {
    fn from(e: Email) -> String {
        e.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderItem {
    pub sku: String,
    pub qty: i32,
    pub unit_price: Decimal,
}

impl OrderItem {
    pub fn line_total(&self) -> Decimal {
        self.unit_price * Decimal::from(self.qty)
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub customer_email: Email,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    status: OrderStatus,   // private: changes only through pay()/cancel()
    items: Vec<OrderItem>, // private: changes only through add_item()
}

/// Everything needed to rebuild an `Order` that was already stored.
///
/// Used by the repository only. Validation already happened when the order was
/// first created, so `restore` trusts the data.
#[derive(Debug)]
pub struct OrderSnapshot {
    pub id: Uuid,
    pub user_id: Uuid,
    pub customer_email: Email,
    pub note: Option<String>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub items: Vec<OrderItem>,
}

impl Order {
    pub fn new(user_id: Uuid, customer_email: Email, note: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            customer_email,
            note,
            created_at: Utc::now(),
            status: OrderStatus::Draft,
            items: Vec::new(),
        }
    }

    pub fn restore(s: OrderSnapshot) -> Self {
        Self {
            id: s.id,
            user_id: s.user_id,
            customer_email: s.customer_email,
            note: s.note,
            created_at: s.created_at,
            status: s.status,
            items: s.items,
        }
    }

    pub fn status(&self) -> OrderStatus {
        self.status
    }

    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    pub fn total(&self) -> Decimal {
        self.items.iter().map(OrderItem::line_total).sum()
    }

    pub fn add_item(&mut self, item: OrderItem) -> Result<(), DomainError> {
        if self.status != OrderStatus::Draft {
            return Err(DomainError::NotEditable(self.status));
        }
        if item.qty <= 0 {
            return Err(DomainError::BadQty);
        }
        if item.unit_price.is_sign_negative() {
            return Err(DomainError::BadPrice);
        }
        self.items.push(item);
        Ok(())
    }

    pub fn pay(&mut self) -> Result<(), DomainError> {
        match self.status {
            OrderStatus::Draft if self.items.is_empty() => Err(DomainError::Empty),
            OrderStatus::Draft => {
                self.status = OrderStatus::Paid;
                Ok(())
            }
            other => Err(DomainError::NotEditable(other)),
        }
    }

    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status == OrderStatus::Cancelled {
            return Err(DomainError::NotEditable(self.status));
        }
        self.status = OrderStatus::Cancelled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn item(qty: i32) -> OrderItem {
        OrderItem {
            sku: "SKU-1".into(),
            qty,
            unit_price: dec!(100),
        }
    }

    fn order() -> Order {
        Order::new(Uuid::new_v4(), Email::try_from("a@b.co").unwrap(), None)
    }

    #[test]
    fn new_order_is_draft_and_empty() {
        let o = order();
        assert_eq!(o.status(), OrderStatus::Draft);
        assert!(o.items().is_empty());
        assert_eq!(o.total(), Decimal::ZERO);
    }

    #[test]
    fn cannot_pay_empty_order() {
        let mut o = order();
        assert_eq!(o.pay(), Err(DomainError::Empty));
        assert_eq!(o.status(), OrderStatus::Draft);
    }

    #[test]
    fn paid_order_is_frozen() {
        let mut o = order();
        o.add_item(item(2)).unwrap();
        o.pay().unwrap();
        assert!(matches!(
            o.add_item(item(1)),
            Err(DomainError::NotEditable(OrderStatus::Paid))
        ));
        assert_eq!(o.total(), dec!(200));
    }

    #[test]
    fn cannot_pay_twice() {
        let mut o = order();
        o.add_item(item(1)).unwrap();
        o.pay().unwrap();
        assert_eq!(o.pay(), Err(DomainError::NotEditable(OrderStatus::Paid)));
    }

    #[test]
    fn rejects_non_positive_qty_and_negative_price() {
        let mut o = order();
        assert_eq!(o.add_item(item(0)), Err(DomainError::BadQty));
        assert_eq!(o.add_item(item(-3)), Err(DomainError::BadQty));
        let bad = OrderItem {
            unit_price: dec!(-1),
            ..item(1)
        };
        assert_eq!(o.add_item(bad), Err(DomainError::BadPrice));
        assert!(o.items().is_empty());
    }

    #[test]
    fn cancel_is_allowed_once() {
        let mut o = order();
        o.add_item(item(1)).unwrap();
        o.pay().unwrap();
        o.cancel().unwrap();
        assert_eq!(o.status(), OrderStatus::Cancelled);
        assert_eq!(
            o.cancel(),
            Err(DomainError::NotEditable(OrderStatus::Cancelled))
        );
        assert_eq!(
            o.pay(),
            Err(DomainError::NotEditable(OrderStatus::Cancelled))
        );
    }

    #[test]
    fn total_sums_lines() {
        let mut o = order();
        o.add_item(item(2)).unwrap();
        o.add_item(OrderItem {
            sku: "SKU-2".into(),
            qty: 3,
            unit_price: dec!(0.10),
        })
        .unwrap();
        assert_eq!(o.total(), dec!(200.30));
    }

    #[test]
    fn email_newtype_validates_and_normalizes() {
        assert_eq!(
            Email::try_from("  V@Example.COM ").unwrap().as_str(),
            "v@example.com"
        );
        for bad in [
            "",
            "no-at",
            "@example.com",
            "a@b",
            "a@@b.com",
            "a b@c.com",
            "a@.com",
        ] {
            assert_eq!(
                Email::try_from(bad),
                Err(DomainError::InvalidEmail),
                "{bad}"
            );
        }
    }

    #[test]
    fn email_deserialization_goes_through_try_from() {
        let ok: Result<Email, _> = serde_json::from_str("\"x@y.org\"");
        assert!(ok.is_ok());
        let bad: Result<Email, _> = serde_json::from_str("\"nope\"");
        assert!(bad.unwrap_err().to_string().contains("невалідний email"));
    }

    #[test]
    fn status_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&OrderStatus::Cancelled).unwrap(),
            "\"cancelled\""
        );
        assert_eq!(
            DomainError::NotEditable(OrderStatus::Paid).to_string(),
            "замовлення вже paid, змінювати не можна"
        );
    }
}
