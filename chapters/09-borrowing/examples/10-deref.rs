//! Dereferencing with `*`: the value behind a reference

fn main() {
    let x = 10;
    let r = &x;
    println!("{}", r + 1); // ok: arithmetic dereferences by itself
    let y: i32 = *r; // explicitly: take the value behind the reference
    println!("{y}");

    let mut count = 0;
    let c = &mut count;
    *c += 1; // change the value through a mutable reference
    println!("{count}"); // 1

    let s = String::from("text");
    let rs = &s;
    println!("{}", rs.len()); // method calls need no asterisk
}
