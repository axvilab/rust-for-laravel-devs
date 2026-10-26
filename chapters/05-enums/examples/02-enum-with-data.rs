//! Every enum variant carries its own data

#[derive(Debug)]
enum OrderStatus {
    Pending,
    Paid { amount: f64, transaction_id: String },
    Shipped { tracking: String },
    Cancelled { reason: String },
}

fn main() {
    let status = OrderStatus::Paid {
        amount: 1500.0,
        transaction_id: String::from("tx_abc"),
    };
    println!("{status:?}");
}
