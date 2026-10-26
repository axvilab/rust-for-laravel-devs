//! String length: bytes (`len`) vs characters (`chars().count()`)

fn main() {
    let s = "Дніпро";
    println!("{}", s.len()); // 12 (bytes) — Cyrillic letters take 2 bytes each in UTF-8
    println!("{}", s.chars().count()); // 6 (characters)
}
