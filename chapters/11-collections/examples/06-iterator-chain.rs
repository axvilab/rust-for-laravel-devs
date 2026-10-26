//! A `.iter().filter().map().sum()` chain — like a Laravel Collection

struct Order {
    total: f64,
    paid: bool,
}

fn main() {
    let orders = [
        Order {
            total: 100.0,
            paid: true,
        },
        Order {
            total: 250.0,
            paid: false,
        },
        Order {
            total: 80.0,
            paid: true,
        },
    ];

    let total: f64 = orders.iter().filter(|o| o.paid).map(|o| o.total).sum();
    println!("{total}"); // 180

    // the same with the turbofish instead of a variable annotation
    let count = orders.iter().filter(|o| !o.paid).count();
    let unpaid = orders
        .iter()
        .filter(|o| !o.paid)
        .map(|o| o.total)
        .sum::<f64>();
    println!("{count} unpaid for {unpaid}");
}
