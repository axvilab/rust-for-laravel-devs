//! `==` (needs `PartialEq`, compares the data too) vs `matches!` (shape only)

#[derive(Debug, PartialEq)]
enum Status {
    Pending,
    Paid { amount: i64 },
}

fn main() {
    let s = Status::Paid { amount: 1500 };
    if s == Status::Pending {
        println!("waiting");
    } // needs PartialEq
    if matches!(s, Status::Paid { .. }) {
        println!("paid");
    } // no PartialEq needed
    if s == (Status::Paid { amount: 1500 }) {
        println!("exactly 1500");
    }
}
