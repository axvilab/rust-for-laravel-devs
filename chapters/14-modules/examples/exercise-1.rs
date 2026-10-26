//! Вправа 1: «shop» по модулях — models, pricing, errors (див. src/ цього крейта)
//!
//! Розв'язок — сама бібліотека: src/models.rs (Product, Order), src/pricing.rs
//! (розрахунки), src/errors.rs (ShopError), підключені в src/lib.rs через `pub mod`.
//! Тут — використання всіх трьох модулів.

use ch14_modules::errors::ShopError;
use ch14_modules::models::{Order, Product};
use ch14_modules::pricing;

fn main() {
    let order = Order::new(
        42,
        vec![
            (Product::new(1, "Ноутбук", 4_500_000), 1),
            (Product::new(2, "Миша", 80_000), 2),
        ],
    )
    .unwrap();
    println!(
        "#{}: {}",
        order.id,
        pricing::format_uah(pricing::total_with_fee(&order))
    );

    match Order::new(43, vec![]) {
        Err(ShopError::EmptyOrder) => println!("порожнє замовлення відхилено"),
        other => println!("неочікувано: {other:?}"),
    }
}
