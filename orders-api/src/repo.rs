//! SQL. The only module that knows about Postgres.
//!
//! Uses runtime-checked `sqlx::query_as` so the project builds without a
//! database. See README for switching to the compile-time-checked
//! `query_as!` macros + `cargo sqlx prepare`.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    domain::{Email, Order, OrderItem, OrderSnapshot, OrderStatus},
    error::{AppError, conflict_on_unique},
    filter::OrderFilter,
    pagination::Pagination,
};

// ---------------------------------------------------------------------------
// Row types: the mapping between Postgres and the domain lives here, so the
// domain does not need `#[derive(sqlx::Type, sqlx::FromRow)]`.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
enum DbStatus {
    Draft,
    Paid,
    Cancelled,
}

impl From<OrderStatus> for DbStatus {
    fn from(s: OrderStatus) -> Self {
        match s {
            OrderStatus::Draft => DbStatus::Draft,
            OrderStatus::Paid => DbStatus::Paid,
            OrderStatus::Cancelled => DbStatus::Cancelled,
        }
    }
}

impl From<DbStatus> for OrderStatus {
    fn from(s: DbStatus) -> Self {
        match s {
            DbStatus::Draft => OrderStatus::Draft,
            DbStatus::Paid => OrderStatus::Paid,
            DbStatus::Cancelled => OrderStatus::Cancelled,
        }
    }
}

#[derive(sqlx::FromRow)]
struct OrderRow {
    id: Uuid,
    user_id: Uuid,
    customer_email: String,
    status: DbStatus,
    note: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct ItemRow {
    order_id: Uuid,
    sku: String,
    qty: i32,
    unit_price: Decimal,
}

impl OrderRow {
    fn into_order(self, items: Vec<OrderItem>) -> Result<Order, AppError> {
        let customer_email = Email::try_from(self.customer_email)
            .map_err(|e| anyhow::anyhow!("corrupt email in order {}: {e}", self.id))?;
        Ok(Order::restore(OrderSnapshot {
            id: self.id,
            user_id: self.user_id,
            customer_email,
            note: self.note,
            status: self.status.into(),
            created_at: self.created_at,
            items,
        }))
    }
}

impl From<ItemRow> for OrderItem {
    fn from(r: ItemRow) -> Self {
        OrderItem {
            sku: r.sku,
            qty: r.qty,
            unit_price: r.unit_price,
        }
    }
}

const ORDER_COLUMNS: &str = "o.id, o.user_id, o.customer_email, o.status, o.note, o.created_at";

/// Who is asking: a regular user sees only their own orders, an admin sees all.
#[derive(Debug, Clone, Copy)]
pub enum Scope {
    User(Uuid),
    All,
}

#[derive(Clone)]
pub struct OrderRepo {
    pool: PgPool,
}

impl OrderRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[tracing::instrument(skip(self))]
    pub async fn find(&self, id: Uuid) -> Result<Order, AppError> {
        // `fetch_optional` + `ok_or` = `findOrFail()` -> 404, not 500
        let row = sqlx::query_as::<_, OrderRow>(
            "SELECT o.id, o.user_id, o.customer_email, o.status, o.note, o.created_at
             FROM orders o WHERE o.id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound)?;

        let items = sqlx::query_as::<_, ItemRow>(
            "SELECT order_id, sku, qty, unit_price FROM order_items
             WHERE order_id = $1 ORDER BY line_no",
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        row.into_order(items.into_iter().map(OrderItem::from).collect())
    }

    /// Upsert the order and replace its items — all in one transaction.
    /// Returning early through `?` drops `tx`, and `Drop` issues a ROLLBACK.
    #[tracing::instrument(skip(self, order), fields(order_id = %order.id))]
    pub async fn save(&self, order: &Order) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO orders (id, user_id, customer_email, status, note, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, note = EXCLUDED.note",
        )
        .bind(order.id)
        .bind(order.user_id)
        .bind(order.customer_email.as_str())
        .bind(DbStatus::from(order.status()))
        .bind(&order.note)
        .bind(order.created_at)
        .execute(&mut *tx)
        .await?;

        sqlx::query("DELETE FROM order_items WHERE order_id = $1")
            .bind(order.id)
            .execute(&mut *tx)
            .await?;

        if !order.items().is_empty() {
            // one multi-row INSERT instead of a query per item
            let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO order_items (order_id, line_no, sku, qty, unit_price) ",
            );
            qb.push_values(order.items().iter().enumerate(), |mut b, (n, item)| {
                b.push_bind(order.id)
                    .push_bind(n as i32 + 1)
                    .push_bind(&item.sku)
                    .push_bind(item.qty)
                    .push_bind(item.unit_price);
            });
            qb.build().execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// Paginated list with filters. Items are loaded with ONE extra query
    /// (`WHERE order_id = ANY($1)`) — what `Order::with('items')` does.
    #[tracing::instrument(skip(self))]
    pub async fn list(
        &self,
        scope: Scope,
        f: &OrderFilter,
        p: &Pagination,
    ) -> Result<Vec<Order>, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new(format!(
            "SELECT {ORDER_COLUMNS},
                    COALESCE((SELECT SUM(i.qty * i.unit_price) FROM order_items i
                              WHERE i.order_id = o.id), 0) AS total
             FROM orders o WHERE 1=1"
        ));
        push_filters(&mut qb, scope, f);
        qb.push(f.sort.order_by());
        qb.push(" LIMIT ")
            .push_bind(p.limit())
            .push(" OFFSET ")
            .push_bind(p.offset());

        let rows: Vec<OrderRow> = qb.build_query_as().fetch_all(&self.pool).await?;

        let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
        let items = sqlx::query_as::<_, ItemRow>(
            "SELECT order_id, sku, qty, unit_price FROM order_items
             WHERE order_id = ANY($1) ORDER BY order_id, line_no",
        )
        .bind(&ids)
        .fetch_all(&self.pool)
        .await?;

        let mut by_order: HashMap<Uuid, Vec<OrderItem>> = HashMap::new();
        for item in items {
            by_order.entry(item.order_id).or_default().push(item.into());
        }

        rows.into_iter()
            .map(|r| {
                let items = by_order.remove(&r.id).unwrap_or_default();
                r.into_order(items)
            })
            .collect()
    }

    pub async fn count(&self, scope: Scope, f: &OrderFilter) -> Result<i64, AppError> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT count(*) FROM orders o WHERE 1=1");
        push_filters(&mut qb, scope, f);
        let n: i64 = qb.build_query_scalar().fetch_one(&self.pool).await?;
        Ok(n)
    }
}

/// `push_bind` is always a bind parameter, never string concatenation.
fn push_filters(qb: &mut QueryBuilder<Postgres>, scope: Scope, f: &OrderFilter) {
    if let Scope::User(uid) = scope {
        qb.push(" AND o.user_id = ").push_bind(uid);
    }
    if let Some(s) = f.status {
        qb.push(" AND o.status = ").push_bind(DbStatus::from(s));
    }
    if let Some(e) = &f.customer_email {
        qb.push(" AND o.customer_email = ")
            .push_bind(e.trim().to_lowercase());
    }
    if let Some(ts) = f.created_after {
        qb.push(" AND o.created_at > ").push_bind(ts);
    }
}

// ---------------------------------------------------------------------------
// Users (for login)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Clone)]
pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, role FROM users WHERE email = $1",
        )
        .bind(email.trim().to_lowercase())
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn create(&self, email: &Email, password_hash: &str) -> Result<User, AppError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)
             RETURNING id, email, password_hash, role",
        )
        .bind(Uuid::new_v4())
        .bind(email.as_str())
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| conflict_on_unique(e, "email already registered"))
    }
}
