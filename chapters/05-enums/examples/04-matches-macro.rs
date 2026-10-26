//! Enum methods and the `matches!` macro

enum OrderStatus {
    Pending,
    Paid { amount: f64 },
    Shipped { tracking: String },
    Cancelled { reason: String },
}

impl OrderStatus {
    fn is_final(&self) -> bool {
        matches!(
            self,
            OrderStatus::Shipped { .. } | OrderStatus::Cancelled { .. }
        )
    }
}

fn main() {
    let a = OrderStatus::Paid { amount: 100.0 };
    let b = OrderStatus::Cancelled {
        reason: "duplicate".into(),
    };
    println!("{} {}", a.is_final(), b.is_final()); // false true
    let _ = (
        OrderStatus::Pending,
        OrderStatus::Shipped {
            tracking: "x".into(),
        },
    );
}
