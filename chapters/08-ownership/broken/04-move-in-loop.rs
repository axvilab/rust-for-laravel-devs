//! Move у циклі: «value moved here, in previous iteration of loop»
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0382]: use of moved value: `email`
//!   note: consider changing this parameter type in function `send` to borrow instead
//!   help: consider cloning the value if the performance cost is acceptable
//! Виправлення: див. examples/04-move-in-loop-fixed.rs (обидва варіанти)
// expected-error: E0382

fn send(email: String) {
    println!("надіслано: {email}");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        send(email); // ПОМИЛКА E0382
    }
}
