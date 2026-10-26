//! Кортежні структури-обгортки і одиничні структури-маркери

// Кортежна: поля без імен. Для «обгорток» навколо одного значення.
struct UserId(u64);
struct OrderId(u64);
struct Email(String);

// Одинична: без полів узагалі. Для маркерів і типів-стратегій.
struct Metric;
struct Imperial;

fn find(id: UserId) -> String {
    format!("користувач #{}", id.0)
}

fn main() {
    let id = UserId(42);
    println!("{}", id.0);
    println!("{}", find(id));

    let order = OrderId(42);
    // find(order); // ПОМИЛКА E0308: expected `UserId`, found `OrderId`
    println!("замовлення #{}", order.0);

    let email = Email("v@example.com".into());
    println!("{}", email.0);
    let _units = (Metric, Imperial);
}
