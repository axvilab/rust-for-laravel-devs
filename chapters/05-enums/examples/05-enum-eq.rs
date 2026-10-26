//! `==` (потрібен `PartialEq`, порівнює й дані) проти `matches!` (лише форма)

#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Paid { amount: i64 },
}

fn main() {
    let s = Status::Paid { amount: 1500 };
    if s == Status::Pending {
        println!("чекаємо");
    } // потрібен PartialEq
    if matches!(s, Status::Paid { .. }) {
        println!("оплачено");
    } // PartialEq не потрібен
    if s == (Status::Paid { amount: 1500 }) {
        println!("рівно 1500");
    }
}
