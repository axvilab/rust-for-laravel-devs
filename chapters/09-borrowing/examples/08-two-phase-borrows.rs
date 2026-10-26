//! `v.push(v.len())` works (two-phase borrows), `v.push(v.pop())` doesn't

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.len()); // ok: v.len() is computed first, then push
    println!("{v:?}"); // [1, 2, 3, 3]

    // v.push(v.pop().unwrap()); // ERROR E0499 (broken/08-two-mut-in-args.rs)
    let last = v.pop().unwrap(); // the fix is a temporary variable
    v.push(last);
    println!("{v:?}"); // [1, 2, 3, 3]
}
