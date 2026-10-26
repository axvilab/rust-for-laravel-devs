//! Вправа 3: `top_n` — n найбільших замовлень за `total`, без клонування самих замовлень

#[derive(Debug)]
struct Order {
    id: u64,
    total: f64,
}

fn top_n(orders: &[Order], n: usize) -> Vec<&Order> {
    let mut refs: Vec<&Order> = orders.iter().collect(); // копіюємо лише посилання
    refs.sort_by(|a, b| b.total.total_cmp(&a.total)); // за спаданням; total_cmp — без паніки на NaN
    refs.truncate(n); // те саме, що take(n), але на місці
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
    println!("всього замовлень досі {}", orders.len()); // orders не змінено
}
