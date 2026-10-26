//! Chapter 17, exercise 3: `#[sqlx::test]` for `OrderRepo` —
//! save, pay, save again, read back and check the status.
//!
//! Needs Postgres via `DATABASE_URL` (see tests/api.rs for details).

use orders_api::{
    domain::{Email, Order, OrderItem, OrderStatus},
    repo::{OrderRepo, UserRepo},
};
use rust_decimal_macros::dec;
use sqlx::PgPool;

#[sqlx::test(migrator = "orders_api::MIGRATOR")]
async fn save_pay_save_find(pool: PgPool) {
    // orders.user_id references users(id), so the order needs an owner
    let email = Email::try_from("v@example.com").unwrap();
    let user = UserRepo::new(pool.clone())
        .create(&email, "not-a-real-hash")
        .await
        .unwrap();

    let repo = OrderRepo::new(pool);
    let mut order = Order::new(user.id, email, None);
    order
        .add_item(OrderItem {
            sku: "SKU-1".into(),
            qty: 2,
            unit_price: dec!(100),
        })
        .unwrap();
    repo.save(&order).await.unwrap();

    order.pay().unwrap();
    repo.save(&order).await.unwrap();

    let found = repo.find(order.id).await.unwrap();
    assert_eq!(found.status(), OrderStatus::Paid);
    assert_eq!(found.customer_email.as_str(), "v@example.com");
    assert_eq!(found.total(), dec!(200));
    assert_eq!(found.items().len(), 1);
}
