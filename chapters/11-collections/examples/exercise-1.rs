//! Вправа 1: 10 замовлень — сума оплачених, id неоплачених, середній чек

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
            paid: id % 3 != 0, // кожне третє — неоплачене
        })
        .collect()
}

fn main() {
    let orders = sample_orders();

    let paid_sum: f64 = orders.iter().filter(|o| o.paid).map(|o| o.total).sum();
    let unpaid_ids: Vec<u64> = orders.iter().filter(|o| !o.paid).map(|o| o.id).collect();
    // середнє: сума / кількість; len() — usize, тому `as f64`
    let avg = orders.iter().map(|o| o.total).sum::<f64>() / orders.len() as f64;

    println!("оплачено: {paid_sum}");
    println!("неоплачені: {unpaid_ids:?}");
    println!("середній чек: {avg:.2}");
}
