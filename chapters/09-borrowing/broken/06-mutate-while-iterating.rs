//! Змінювати колекцію, поки по ній ітеруєшся, не можна
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0502]: cannot borrow `list` as mutable because it is
//!   also borrowed as immutable
//! Виправлення: спочатку зібрати, що додати, потім додати — див.
//!   examples/06-collect-then-push.rs
// expected-error: E0502

fn main() {
    let mut list = vec![1, 2, 3];
    for item in &list {
        if *item == 2 {
            list.push(99); // ПОМИЛКА: list уже позичений циклом як &
        }
    }
}
