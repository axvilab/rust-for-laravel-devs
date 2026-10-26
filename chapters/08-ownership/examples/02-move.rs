//! Move: після `let b = a` власник рядка — `b`

fn main() {
    let a = String::from("привіт");
    let b = a; // володіння ПЕРЕЙШЛО до b
    // println!("{a}");           // ПОМИЛКА E0382 (broken/02-move.rs)
    println!("{b}");
}
