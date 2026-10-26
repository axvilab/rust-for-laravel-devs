//! Конструкторів немає: літерал з усіма полями, а `new` — звичайна функція

struct Invoice {
    id: u64,
    total: f64,
    paid: bool,
}

impl Invoice {
    // `new` і `draft` — звичайні функції, названі за домовленістю
    fn new(id: u64, total: f64) -> Self {
        Self {
            id,
            total,
            paid: false,
        }
    }
    fn draft(id: u64) -> Self {
        Self::new(id, 0.0)
    }
}

fn main() {
    let inv = Invoice {
        id: 1,
        total: 1500.0,
        paid: false,
    };
    println!("{} {} {}", inv.id, inv.total, inv.paid);

    let a = Invoice::new(2, 99.0);
    let b = Invoice::draft(3);
    println!("{} {}", a.total, b.total);
}
