//! No constructors: a literal with all fields, and `new` is just a function

struct Invoice {
    id: u64,
    total: f64,
    paid: bool,
}

impl Invoice {
    // `new` and `draft` are ordinary functions, named by convention
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
