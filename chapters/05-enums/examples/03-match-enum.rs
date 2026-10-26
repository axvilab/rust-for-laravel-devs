//! `match` по enum: дані витягуються прямо в гілці, `..` — «решта полів»

enum OrderStatus {
    Pending,
    Paid { amount: f64, transaction_id: String },
    Shipped { tracking: String },
    Cancelled { reason: String },
}

fn describe(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Pending => "очікує оплати".to_string(),
        OrderStatus::Paid { amount, .. } => format!("оплачено {amount} грн"),
        OrderStatus::Shipped { tracking } => format!("в дорозі, ТТН {tracking}"),
        OrderStatus::Cancelled { reason } => format!("скасовано: {reason}"),
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
            reason: "клієнт передумав".into(),
        },
    ];
    for s in &all {
        println!("{}", describe(s));
    }
}
