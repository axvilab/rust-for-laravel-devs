//! A recursive type without `Box` has infinite size
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0072]: recursive type `Expr` has infinite size
//!   help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
//! Fix: `Add(Box<Expr>, Box<Expr>)` — see examples/07-box-recursive.rs
// expected-error: E0072

enum Expr {
    Num(i64),
    Add(Expr, Expr),
}

fn main() {
    let _ = Expr::Num(1);
}
