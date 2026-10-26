//! Значення функції — останній вираз; `return` можливий, але не ідіоматичний
// clippy::needless_return свідомо вимкнено: square_verbose показує саме цю форму.
#![allow(clippy::needless_return)]

fn square(n: i32) -> i32 {
    n * n // return не потрібен
}

fn square_verbose(n: i32) -> i32 {
    return n * n; // так теж можна, але ідіоматично — без return
}

fn main() {
    println!("{} {}", square(4), square_verbose(4));
}
