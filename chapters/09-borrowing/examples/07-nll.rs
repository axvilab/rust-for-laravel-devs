//! NLL: a borrow lives until its last use, not until `}`

fn main() {
    let mut scores = vec![10, 20];
    let first = &scores[0]; // the borrow starts
    println!("{first}"); // ...and ends here: the last use
    scores.push(30); // ok, even though first is still "in scope"
    println!("{scores:?}");
}
