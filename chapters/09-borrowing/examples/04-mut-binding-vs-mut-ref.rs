//! `let mut r = &x` (can be redirected) vs `let r = &mut x` (can change the value)

fn main() {
    let mut a = 1;
    let b = 2;

    let r = &mut a; // r is an immutable variable, but through it you CAN change a
    *r += 10;
    // r = &mut b;    // but you can't point r at something else

    let mut s = &b; // s can be redirected, but you can't change anything through it
    println!("{s}"); // 2
    // *s += 1;       // ERROR: cannot assign to `*s`, which is behind a `&` reference
    s = &a;
    println!("{s}"); // 11
}
