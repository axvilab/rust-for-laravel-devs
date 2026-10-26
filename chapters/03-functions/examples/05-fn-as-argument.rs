//! Замість замикання можна передати ім'я функції чи методу

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    // У книзі `.map` викликається прямо на `Some(...)`; clippy на це каже
    // unnecessary_map_on_constructor («просто напиши Some(double(21))»).
    // У реальному коді Option приходить ззовні — тому тут він у змінних.
    let num = Some(-5_i32);
    let answer = Some(21);
    let text = Some("текст");

    let a = num.map(|n| n.abs()); // замикання
    let b = num.map(i32::abs); // те саме: ім'я методу
    let c = answer.map(double); // своя функція
    let d = text.map(String::from);
    println!("{a:?} {b:?} {c:?} {d:?}"); // Some(5) Some(5) Some(42) Some("текст")
}
