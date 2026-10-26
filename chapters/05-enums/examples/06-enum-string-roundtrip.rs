//! enum у рядок і назад — аналог `->value` і `::tryFrom()`

#[derive(Debug, Clone, Copy, PartialEq)]
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
}

impl OrderStatus {
    fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "pending",
            OrderStatus::Paid => "paid",
            OrderStatus::Shipped => "shipped",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "paid" => Some(Self::Paid),
            "shipped" => Some(Self::Shipped),
            _ => None,
        }
    }
}

fn main() {
    println!("{}", OrderStatus::Paid.as_str()); // paid — як ->value
    println!("{:?}", OrderStatus::parse("shipped")); // Some(Shipped) — як tryFrom()
    println!("{:?}", OrderStatus::parse("lost")); // None
    println!("{}", OrderStatus::Shipped as i32); // 2 — номер варіанта
}
