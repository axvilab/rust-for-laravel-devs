//! `T: 'static` — «не містить позичених посилань», а не «живе вічно»
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0597]: `local` does not live long enough
//!   (argument requires that `local` is borrowed for `'static`)
//! Виправлення: передати володіння (`keep(local)`) — див. examples/06-static-bound.rs
// expected-error: E0597

fn keep<T: 'static>(_x: T) {}

fn main() {
    let owned = String::from("я String, і мені можна");
    keep(owned); // OK: String нічого не позичає

    let local = String::from("локальний");
    keep(&local); // ПОМИЛКА: `local` does not live long enough
}
