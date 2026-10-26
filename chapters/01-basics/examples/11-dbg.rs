//! `println!` (stdout), `eprintln!` (stderr) і `dbg!` — справжній `dd()`

fn main() {
    let price = 100;
    let total = dbg!(price * 3) + 1; // dbg! повертає значення далі
    eprintln!("це піде в stderr");
    println!("{total}");
}
