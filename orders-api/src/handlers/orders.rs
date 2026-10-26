//! Order endpoints: extractors -> domain -> JSON.

use std::sync::LazyLock;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
    auth::{AdminUser, AuthUser, Claims},
    domain::{DomainError, Email, Order, OrderItem, OrderStatus},
    error::AppError,
    filter::OrderFilter,
    pagination::{Page, Pagination},
    repo::Scope,
    state::AppState,
    validation::ValidatedJson,
};

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

static SKU_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z0-9][A-Z0-9_-]*$").expect("valid regex"));

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrder {
    /// Newtype: an invalid email fails during deserialization.
    pub customer_email: Email,
    #[serde(default)]
    #[validate(length(max = 50), nested)]
    pub items: Vec<ItemReq>,
    #[validate(length(max = 500))]
    pub note: Option<String>,
}

// `Serialize` is required by validator: `length(...)` on `Vec<ItemReq>`
// puts the offending value into the error params.
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ItemReq {
    #[validate(length(min = 1, max = 64), regex(path = *SKU_RE))]
    pub sku: String,
    /// `u32`: `-5` is rejected by the type before validation even runs.
    #[validate(range(min = 1, max = 1000))]
    pub qty: u32,
    #[validate(custom(function = "validate_price"))]
    pub unit_price: Decimal,
}

/// Matches the `numeric(12, 2)` column: non-negative, at most 2 decimals.
fn validate_price(p: &Decimal) -> Result<(), ValidationError> {
    if p.is_sign_negative() {
        return Err(ValidationError::new("price_negative"));
    }
    if p.normalize().scale() > 2 {
        return Err(ValidationError::new("price_scale"));
    }
    if *p >= Decimal::from(10_000_000_000_i64) {
        return Err(ValidationError::new("price_too_large"));
    }
    Ok(())
}

impl TryFrom<ItemReq> for OrderItem {
    type Error = DomainError;

    fn try_from(r: ItemReq) -> Result<Self, DomainError> {
        Ok(OrderItem {
            sku: r.sku,
            qty: i32::try_from(r.qty).map_err(|_| DomainError::BadQty)?,
            unit_price: r.unit_price,
        })
    }
}

// ---------------------------------------------------------------------------
// Response — our `JsonResource`. The domain type is never serialized as is.
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub customer_email: String,
    pub status: OrderStatus,
    pub total: Decimal,
    pub items: Vec<OrderItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Money always leaves the API with exactly two decimals ("200.00"), whether
/// the order came from the request (`100`) or from `numeric(12, 2)` (`100.00`).
fn money(mut d: Decimal) -> Decimal {
    d.rescale(2);
    d
}

impl From<Order> for OrderResponse {
    fn from(o: Order) -> Self {
        Self {
            total: money(o.total()),
            status: o.status(),
            items: o
                .items()
                .iter()
                .map(|i| OrderItem {
                    unit_price: money(i.unit_price),
                    ..i.clone()
                })
                .collect(),
            id: o.id,
            customer_email: o.customer_email.into(),
            note: o.note,
            created_at: o.created_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Someone else's order looks exactly like a missing one: 404, not 403,
/// so ids of other users' orders do not leak.
async fn load_owned(st: &AppState, id: Uuid, user: &Claims) -> Result<Order, AppError> {
    let order = st.repo.find(id).await?;
    if order.user_id != user.sub && !user.is_admin() {
        return Err(AppError::NotFound);
    }
    Ok(order)
}

pub async fn create(
    State(st): State<AppState>,
    AuthUser(user): AuthUser, // no / bad token -> 401 before the body is read
    ValidatedJson(body): ValidatedJson<CreateOrder>,
) -> Result<(StatusCode, Json<OrderResponse>), AppError> {
    let mut order = Order::new(user.sub, body.customer_email, body.note);
    for item in body.items {
        order.add_item(item.try_into()?)?;
    }
    st.repo.save(&order).await?;
    tracing::info!(order_id = %order.id, "order created");
    Ok((StatusCode::CREATED, Json(order.into())))
}

pub async fn show(
    State(st): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    Ok(Json(load_owned(&st, id, &user).await?.into()))
}

pub async fn add_item(
    State(st): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
    ValidatedJson(body): ValidatedJson<ItemReq>,
) -> Result<Json<OrderResponse>, AppError> {
    let mut order = load_owned(&st, id, &user).await?;
    order.add_item(body.try_into()?)?;
    st.repo.save(&order).await?;
    Ok(Json(order.into()))
}

pub async fn pay(
    State(st): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let mut order = load_owned(&st, id, &user).await?; // missing -> 404
    order.pay()?; // already paid / empty -> 422
    st.repo.save(&order).await?; // db down -> 500 + log
    tracing::info!(order_id = %order.id, total = %order.total(), "order paid");
    Ok(Json(order.into()))
}

pub async fn cancel(
    State(st): State<AppState>,
    AuthUser(user): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<OrderResponse>, AppError> {
    let mut order = load_owned(&st, id, &user).await?;
    order.cancel()?;
    st.repo.save(&order).await?;
    Ok(Json(order.into()))
}

/// `GET /orders` — the current user's orders.
pub async fn list(
    State(st): State<AppState>,
    AuthUser(user): AuthUser,
    Query(p): Query<Pagination>,
    Query(f): Query<OrderFilter>,
) -> Result<Json<Page<OrderResponse>>, AppError> {
    page(&st, Scope::User(user.sub), &f, &p).await
}

/// `GET /admin/orders` — everyone's orders; `AdminUser` = `can:admin`.
pub async fn admin_list(
    State(st): State<AppState>,
    AdminUser(_admin): AdminUser,
    Query(p): Query<Pagination>,
    Query(f): Query<OrderFilter>,
) -> Result<Json<Page<OrderResponse>>, AppError> {
    page(&st, Scope::All, &f, &p).await
}

async fn page(
    st: &AppState,
    scope: Scope,
    f: &OrderFilter,
    p: &Pagination,
) -> Result<Json<Page<OrderResponse>>, AppError> {
    p.validate()?; // ?per_page=1000 or ?page=0 -> 422
    // list and count run concurrently on two pool connections
    let (rows, total) = tokio::join!(st.repo.list(scope, f, p), st.repo.count(scope, f));
    let (rows, total) = (rows?, total?);
    Ok(Json(Page::new(rows, p, total).map(OrderResponse::from)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn item_request_validation() {
        let ok = ItemReq {
            sku: "SKU-1".into(),
            qty: 1,
            unit_price: dec!(9.99),
        };
        assert!(ok.validate().is_ok());

        let bad = ItemReq {
            sku: "bad sku".into(),
            qty: 0,
            unit_price: dec!(1.001),
        };
        let errs = bad.validate().unwrap_err();
        let fields = errs.field_errors();
        assert!(fields.contains_key("sku"));
        assert!(fields.contains_key("qty"));
        assert!(fields.contains_key("unit_price"));
    }

    #[test]
    fn create_order_rejects_bad_email_at_deserialization() {
        let r: Result<CreateOrder, _> = serde_json::from_str(r#"{"customer_email":"nope"}"#);
        assert!(r.is_err());
        let ok: CreateOrder =
            serde_json::from_str(r#"{"customer_email":"v@example.com"}"#).unwrap();
        assert!(ok.items.is_empty());
    }
}
