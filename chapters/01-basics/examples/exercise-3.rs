//! Вправа 3: без `mut` повторне присвоєння не компілюється
//!
//! Якщо прибрати `mut`, компілятор скаже:
//!   error[E0384]: cannot assign twice to immutable variable `price`
//!   help: consider making this binding mutable: `mut price`
//! Тобто він сам підказує виправлення — саме воно і стоїть нижче.
//! Зламана версія: broken/exercise-3.rs

fn main() {
    let mut price = 100.0;
    price *= 0.9;
    println!("{price}");
}
