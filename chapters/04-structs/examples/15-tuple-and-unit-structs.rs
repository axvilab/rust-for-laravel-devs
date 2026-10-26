//! Tuple structs as wrappers and unit structs as markers

// Tuple struct: fields without names. For "wrappers" around one value.
struct UserId(u64);
struct OrderId(u64);
struct Email(String);

// Unit struct: no fields at all. For markers and strategy types.
struct Metric;
struct Imperial;

fn find(id: UserId) -> String {
    format!("user #{}", id.0)
}

fn main() {
    let id = UserId(42);
    println!("{}", id.0);
    println!("{}", find(id));

    let order = OrderId(42);
    // find(order); // ERROR E0308: expected `UserId`, found `OrderId`
    println!("order #{}", order.0);

    let email = Email("v@example.com".into());
    println!("{}", email.0);
    let _units = (Metric, Imperial);
}
