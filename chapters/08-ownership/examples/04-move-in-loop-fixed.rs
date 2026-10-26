//! Два виправлення E0382 у циклі: позичити (краще) або клонувати

// 1. Функція тільки читає → параметр &str, передаємо &email
fn send(email: &str) {
    println!("надіслано: {email}");
}

// 2. Функція справді хоче володіти → викликаємо з clone()
fn send_owned(email: String) {
    println!("надіслано (копія): {email}");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        send(&email); // жодної копії
    }
    for _ in 0..3 {
        send_owned(email.clone()); // три копії рядка
    }
}
