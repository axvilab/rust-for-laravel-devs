//! `v.push(v.len())` працює (two-phase borrows), а `v.push(v.pop())` — ні

fn main() {
    let mut v: Vec<usize> = vec![1, 2, 3];
    v.push(v.len()); // ok: спершу рахується v.len(), потім push
    println!("{v:?}"); // [1, 2, 3, 3]

    // v.push(v.pop().unwrap()); // ПОМИЛКА E0499 (broken/08-two-mut-in-args.rs)
    let last = v.pop().unwrap(); // рішення — тимчасова змінна
    v.push(last);
    println!("{v:?}"); // [1, 2, 3, 3]
}
