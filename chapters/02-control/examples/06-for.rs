//! `for` по діапазонах і колекціях, `&users`, `.iter().enumerate()`

fn main() {
    // У книзі тіла циклів порожні (`for i in 0..10 { }`) — компілятор тоді
    // попереджає про невикористану `i`. Тут друкуємо, щоб було видно діапазони.
    for i in 0..10 {
        print!("{i} "); // 0,1,...,9  — як range(0, 9)
    }
    println!();
    for i in 0..=10 {
        print!("{i} "); // 0,1,...,10 — включно
    }
    println!();
    for i in (0..10).rev() {
        print!("{i} "); // у зворотному порядку
    }
    println!();

    let users = vec!["Іван", "Марія", "Олег"];
    for user in &users {
        // & — позичаємо, не забираємо (розділи про ownership і borrowing)
        println!("{user}");
    }

    for (i, user) in users.iter().enumerate() {
        // з індексом, як foreach з $key
        println!("{i}: {user}");
    }
}
