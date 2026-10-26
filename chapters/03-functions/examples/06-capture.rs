//! A closure sees outer variables by itself, no `use` needed

fn main() {
    let rate = 1.2;
    let with_tax = |price: f64| price * rate;
    println!("{}", with_tax(100.0)); // 120
    println!("{rate}"); // rate is still available: the closure only borrowed it
}
