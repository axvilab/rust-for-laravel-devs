//! Простий enum — як PHP 8.1 enum без значень

#[derive(Debug)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

fn main() {
    let status = OrderStatus::Paid;
    println!("{status:?}");
    let _others = [
        OrderStatus::Pending,
        OrderStatus::Shipped,
        OrderStatus::Cancelled,
    ];
}
