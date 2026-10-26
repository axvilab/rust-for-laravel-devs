//! Move: after `let b = a` the owner of the string is `b`

fn main() {
    let a = String::from("hello");
    let b = a; // ownership MOVED to b
    // println!("{a}");           // ERROR E0382 (broken/02-move.rs)
    println!("{b}");
}
