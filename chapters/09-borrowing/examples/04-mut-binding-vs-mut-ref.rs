//! `let mut r = &x` (можна перенаправити) проти `let r = &mut x` (можна змінити)

fn main() {
    let mut a = 1;
    let b = 2;

    let r = &mut a; // r — незмінна змінна, але через неї МОЖНА змінити a
    *r += 10;
    // r = &mut b;    // а перенаправити r на інше — не можна

    let mut s = &b; // s можна перенаправити, але змінити через неї — ні
    println!("{s}"); // 2
    // *s += 1;       // ПОМИЛКА: cannot assign to `*s`, which is behind a `&` reference
    s = &a;
    println!("{s}"); // 11
}
