//! Вправа 6: рядок у функцію в циклі — обидва виправлення по черзі
//!
//! Компілятор на версії без виправлень (broken/exercise-6.rs) дає E0382 і дві підказки:
//!   note: consider changing this parameter type in function `notify` to borrow instead
//!   help: consider cloning the value if the performance cost is acceptable

// Варіант з note: функція позичає
fn notify(email: &str) {
    println!("лист на {email}");
}

// Варіант з help: функція володіє, викликаємо з clone()
fn notify_owned(email: String) {
    println!("лист на {email} (копія)");
}

fn main() {
    let email = String::from("v@example.com");
    for _ in 0..3 {
        notify(&email);
    }
    for _ in 0..3 {
        notify_owned(email.clone());
    }
}
