//! `for s in &v` (borrow), `&mut v` (change), `v` (take)

fn main() {
    let mut v = vec![String::from("a"), String::from("b")];

    for s in &v {
        // s: &String — we borrow, v is alive after the loop
        print!("{s} ");
    }
    println!();
    for s in &mut v {
        // s: &mut String — the elements can be changed
        s.push('!');
    }
    for s in v {
        // s: String — we take, v is gone
        print!("{s} ");
    }
    println!();
    // println!("{v:?}"); // ERROR E0382 (broken/09-for-moves-vec.rs)
}
