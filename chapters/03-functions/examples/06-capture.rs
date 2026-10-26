//! Замикання бачить зовнішні змінні саме, без `use`

fn main() {
    let rate = 1.2;
    let with_tax = |price: f64| price * rate;
    println!("{}", with_tax(100.0)); // 120
    println!("{rate}"); // rate усе ще доступний: замикання його лише позичило
}
