//! Exercise 6: `push(v.len())` works, `push(v.pop())` — via a temporary variable

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.len()); // reading in an argument — ok (two-phase borrow)
    println!("{v:?}"); // [1, 2, 3, 3]

    let last = v.pop().unwrap(); // the first &mut ended here
    v.push(last); // the second &mut — no conflict any more
    println!("{v:?}"); // [1, 2, 3, 3]
}
