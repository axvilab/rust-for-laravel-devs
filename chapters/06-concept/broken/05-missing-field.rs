//! An "uninitialized field" is impossible: a literal must list every field
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: error[E0063]: missing field `paid` in initializer of `Invoice`
//! Fix: give all the fields — see examples/05-no-constructors.rs
// expected-error: E0063

struct Invoice {
    id: u64,
    total: f64,
    paid: bool,
}

fn main() {
    let inv = Invoice { id: 1, total: 1500.0 };
    println!("{} {}", inv.id, inv.total);
}
