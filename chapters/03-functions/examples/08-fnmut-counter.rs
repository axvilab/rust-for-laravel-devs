//! The closure changes `count` → the closure itself must be `mut`

fn main() {
    let mut count = 0;
    let mut inc = || count += 1; // the closure changes count → it must be mut itself
    inc();
    inc();
    println!("{count}"); // 2
}
