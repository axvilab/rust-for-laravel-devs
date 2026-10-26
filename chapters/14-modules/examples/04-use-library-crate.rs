//! Приклад підключає бібліотеку цього ж пакета ззовні: дефіс у назві → підкреслення
//!
//! Пакет `ch14-modules` у коді — `ch14_modules`, як `tower-http` → `tower_http`.

use ch14_modules::models::{Order, Product};
use ch14_modules::services::PaymentService; // працює завдяки `pub use` у services.rs
use ch14_modules::{calc_fee, pricing};

fn main() {
    let order = Order::new(1, vec![(Product::new(1, "Миша", 80_000), 2)]).unwrap();
    let sub = pricing::subtotal(&order);
    println!(
        "{} + комісія {}",
        pricing::format_uah(sub),
        pricing::format_uah(calc_fee(sub))
    );
    PaymentService::new()
        .charge("v@example.com", pricing::total_with_fee(&order))
        .unwrap();
    // pricing::round_to_hryvnia(1); // ПОМИЛКА E0603: pub(crate) не видно ззовні крейта
}
