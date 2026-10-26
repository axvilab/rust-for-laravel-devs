//! «Поле не ініціалізоване» неможливе: літерал має перелічити всі поля
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0063]: missing field `paid` in initializer of `Invoice`
//! Виправлення: вказати всі поля — див. examples/05-no-constructors.rs
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
