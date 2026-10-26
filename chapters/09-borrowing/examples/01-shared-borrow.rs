//! `&T` — позичити подивитись: власник лишається власником

// clippy радить &str замість &String (ptr_arg) — це наступний приклад книги;
// тут навмисно &String, як у тексті.
#[allow(clippy::ptr_arg)]
fn print_name(name: &String) {
    // приймаємо посилання
    println!("{name}");
} // name — лише посилання, нічого не знищується

fn main() {
    let n = String::from("Віталій");
    print_name(&n); // & — позичаємо
    print_name(&n); // можна скільки завгодно
    println!("{n}"); // n усе ще твій
}
