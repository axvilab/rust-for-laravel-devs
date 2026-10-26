//! Вправа 6 (зламана версія): той самий String тричі в функцію, що приймає String
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: use of moved value: `email`
//!   (value moved here, in previous iteration of loop)
//! Виправлення: див. examples/exercise-6.rs
// expected-error: in previous iteration of loop

fn notify(email: String) {
    println!("лист на {email}");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        notify(email);
    }
}
