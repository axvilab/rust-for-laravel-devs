//! Filters and sorting for `GET /orders`. Unknown values never reach SQL:
//! `?sort=id;drop table` fails to deserialize into `SortBy` -> 400.

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::domain::OrderStatus;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrderFilter {
    pub status: Option<OrderStatus>,
    pub customer_email: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    #[serde(default)]
    pub sort: SortBy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    #[default]
    CreatedDesc,
    CreatedAsc,
    TotalDesc,
}

impl SortBy {
    /// `ORDER BY` cannot be a bind parameter, so it comes only from this `match`.
    /// `id` is a tie-breaker so pages are stable.
    pub fn order_by(self) -> &'static str {
        match self {
            SortBy::CreatedDesc => " ORDER BY o.created_at DESC, o.id DESC",
            SortBy::CreatedAsc => " ORDER BY o.created_at ASC, o.id ASC",
            SortBy::TotalDesc => " ORDER BY total DESC, o.id DESC",
        }
    }
}
