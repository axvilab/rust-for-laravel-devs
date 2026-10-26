//! `println!` (stdout), `eprintln!` (stderr) and `dbg!` — the real `dd()`

fn main() {
    let price = 100;
    let total = dbg!(price * 3) + 1; // dbg! passes the value on
    eprintln!("this goes to stderr");
    println!("{total}");
}
