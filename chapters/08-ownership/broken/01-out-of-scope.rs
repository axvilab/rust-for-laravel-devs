//! Змінна поза своїм блоком не існує
//!
//! НЕ КОМПІЛЮЄТЬСЯ — так задумано.
//! Очікувана помилка: error[E0425]: cannot find value `name` in this scope
//! Виправлення: використовувати `name` всередині блоку — див. examples/01-scope.rs
// expected-error: E0425

fn main() {
    {
        let name = String::from("Віталій");
        println!("{name}");
    }
    println!("{name}");
}
