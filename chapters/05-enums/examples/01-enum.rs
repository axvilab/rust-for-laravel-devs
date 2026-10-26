//! A simple enum — like a PHP 8.1 enum without values

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
