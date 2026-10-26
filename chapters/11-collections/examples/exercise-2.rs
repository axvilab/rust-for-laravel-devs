//! Exercise 2: grouping by `paid` into a `HashMap<bool, Vec<&Order>>`

use std::collections::HashMap;

#[derive(Debug)]
struct Order {
    id: u64,
    total: f64,
    paid: bool,
}

fn main() {
    let orders: Vec<Order> = (1..=6)
        .map(|id| Order {
            id,
            total: id as f64 * 100.0,
            paid: id % 2 == 0,
        })
        .collect();

    let mut groups: HashMap<bool, Vec<&Order>> = HashMap::new();
    for o in &orders {
        // or_default() creates an empty Vec for a new key
        groups.entry(o.paid).or_default().push(o);
    }

    for paid in [true, false] {
        let ids: Vec<u64> = groups
            .get(&paid)
            .map(|v| v.iter().map(|o| o.id).collect())
            .unwrap_or_default();
        println!("paid={paid}: {ids:?}");
    }
}
