//! Future нічого не робить, поки його не `.await`-нули

async fn fetch_user(id: u64) -> u64 {
    println!("  → запит для {id} пішов");
    id
}

#[tokio::main]
async fn main() {
    let fut = fetch_user(5); // створили Future — запит ще НЕ пішов
    println!("future створено, але нічого не надруковано");
    let id = fut.await; // тільки тепер
    println!("отримали {id}");
    // Просто `fetch_user(5);` без змінної і без .await компілятор позначить
    // попередженням `unused implementer of Future that must be used`.
}
