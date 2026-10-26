//! Рекурсивний тип без `Box` має нескінченний розмір
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0072]: recursive type `Expr` has infinite size
//!   help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
//! Виправлення: `Add(Box<Expr>, Box<Expr>)` — див. examples/07-box-recursive.rs
// expected-error: E0072

enum Expr {
    Num(i64),
    Add(Expr, Expr),
}

fn main() {
    let _ = Expr::Num(1);
}
