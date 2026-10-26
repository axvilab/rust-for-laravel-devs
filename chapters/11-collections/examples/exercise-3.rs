//! Exercise 3: `top_n` — the n largest orders by `total`, without cloning the orders

#[derive(Debug)]
struct Order {
    id: u64,
    total: f64,
}

fn top_n(orders: &[Order], n: usize) -> Vec<&Order> {
    let mut refs: Vec<&Order> = orders.iter().collect(); // only references are copied
    refs.sort_by(|a, b| b.total.total_cmp(&a.total)); // descending; total_cmp — no panic on NaN
    refs.truncate(n); // the same as take(n), but in place
    refs
}

fn main() {
    let orders = vec![
        Order {
            id: 1,
            total: 300.0,
        },
        Order {
            id: 2,
            total: 1200.0,
        },
        Order { id: 3, total: 50.0 },
        Order {
            id: 4,
            total: 800.0,
        },
    ];
    for o in top_n(&orders, 2) {
        println!("{} → {}", o.id, o.total); // 2 → 1200, 4 → 800
    }
    println!("still {} orders in total", orders.len()); // orders unchanged
}
