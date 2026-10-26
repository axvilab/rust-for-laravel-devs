//! Вправа 2: без гілки `_` match по u16 не вичерпний
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0004]: non-exhaustive patterns:
//!   `0_u16..=99_u16` and `600_u16..=u16::MAX` not covered
//! Компілятор сам перелічує, які саме значення не покрито.
//! Виправлення: повернути `_ => "invalid"` — див. examples/exercise-1.rs
// expected-error: E0004

fn http_class(status: u16) -> &'static str {
    match status {
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
    }
}

fn main() {
    println!("{}", http_class(200));
}
