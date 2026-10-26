//! `match` on an enum: data is pulled out right in the arm, `..` means "the other fields"

enum OrderStatus {
    Pending,
    Paid { amount: f64, transaction_id: String },
    Shipped { tracking: String },
    Cancelled { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => "awaiting payment".to_string(),
        OrderStatus::Paid { amount, .. } => format!("paid {amount} UAH"),
        OrderStatus::Shipped { tracking } => format!("on the way, tracking {tracking}"),
        OrderStatus::Cancelled { reason } => format!("cancelled: {reason}"),
    }
}

fn main() {
    let all = [
        OrderStatus::Pending,
        OrderStatus::Paid {
            amount: 1500.0,
            transaction_id: "tx_abc".into(),
        },
        OrderStatus::Shipped {
            tracking: "20450000000000".into(),
        },
        OrderStatus::Cancelled {
            reason: "the customer changed their mind".into(),
        },
    ];
    for s in &all {
        println!("{}", describe(s));
    }
}
