//! A trap: shadowing inside a block doesn't change the outer variable

fn main() {
    let has_discount = true;
    let total = 100;
    if has_discount {
        let total = total + 50; // a NEW variable, lives only in this block
        println!("inside: {total}"); // 150
    }
    println!("outside: {total}"); // 100 — nobody touched the outer one

    // Correct: mut outside and a change WITHOUT let inside.
    let mut total = 100;
    if has_discount {
        total += 50;
    }
    println!("fixed: {total}"); // 150
}
