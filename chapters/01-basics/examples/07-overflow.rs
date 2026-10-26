//! Переповнення: `checked_*`, `saturating_*`, `wrapping_*` і мовчазний `as`

fn main() {
    let stock: u32 = "3".parse().unwrap(); // значення «ззовні»
    let sold: u32 = 5;

    // let left = stock - sold;   // debug: паніка, release: 4294967294

    println!("{:?}", stock.checked_sub(sold)); // None — «не вийшло»
    println!("{}", stock.saturating_sub(sold)); // 0 — притиснуло до межі
    println!("{}", stock.wrapping_sub(sold)); // 4294967294 — по колу
    println!("{:?}", stock.checked_add(1)); // Some(4)

    println!("{}", 300_i32 as u8); // 44 — as мовчки обрізає
    println!("{}", -1_i32 as u32); // 4294967295
    println!("{}", 3.99_f64 as i32); // 3 — дробова частина відкидається
}
