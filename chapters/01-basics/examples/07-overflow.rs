//! Overflow: `checked_*`, `saturating_*`, `wrapping_*` and the silent `as`

fn main() {
    let stock: u32 = "3".parse().unwrap(); // a value "from outside"
    let sold: u32 = 5;

    // let left = stock - sold;   // debug: panic, release: 4294967294

    println!("{:?}", stock.checked_sub(sold)); // None — "didn't work"
    println!("{}", stock.saturating_sub(sold)); // 0 — clamped to the limit
    println!("{}", stock.wrapping_sub(sold)); // 4294967294 — wrapped around
    println!("{:?}", stock.checked_add(1)); // Some(4)

    println!("{}", 300_i32 as u8); // 44 — as silently truncates
    println!("{}", -1_i32 as u32); // 4294967295
    println!("{}", 3.99_f64 as i32); // 3 — the fractional part is dropped
}
