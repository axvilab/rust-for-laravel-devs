//! Exercise 1: 10 orders — the sum of paid ones, ids of unpaid ones, the average order

#[derive(Debug)]
struct Order {
    id: u64,
    total: f64,
    paid: bool,
}

fn sample_orders() -> Vec<Order> {
    (1..=10)
        .map(|id| Order {
            id,
            total: (id * 150) as f64,
            paid: id % 3 != 0, // every third one is unpaid
        })
        .collect()
}

fn main() {
    let orders = sample_orders();

    let paid_sum: f64 = orders.iter().filter(|o| o.paid).map(|o| o.total).sum();
    let unpaid_ids: Vec<u64> = orders.iter().filter(|o| !o.paid).map(|o| o.id).collect();
    // average: sum / count; len() is usize, hence `as f64`
    let avg = orders.iter().map(|o| o.total).sum::<f64>() / orders.len() as f64;

    println!("paid: {paid_sum}");
    println!("unpaid: {unpaid_ids:?}");
    println!("average order: {avg:.2}");
}
