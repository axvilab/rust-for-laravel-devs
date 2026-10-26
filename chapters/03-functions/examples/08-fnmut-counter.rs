//! Замикання змінює `count` → саме замикання має бути `mut`

fn main() {
    let mut count = 0;
    let mut inc = || count += 1; // замикання змінює count → саме має бути mut
    inc();
    inc();
    println!("{count}"); // 2
}
