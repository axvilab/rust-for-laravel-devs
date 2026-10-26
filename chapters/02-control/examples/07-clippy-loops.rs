//! Що скаже `cargo clippy` про цикли «в стилі PHP» — і як переписати
//!
//! Перша половина — навмисно «PHP-шна»; три лінти вимкнено атрибутами на функції,
//! інакше CI (`clippy -D warnings`) не пропустить. Прибери `#[allow(...)]` і запусти
//! `cargo clippy -p ch02-control --example 07-clippy-loops` — побачиш усі три поради.

#[allow(clippy::len_zero, clippy::needless_range_loop, clippy::useless_vec)]
fn php_style() {
    let users = vec!["Іван", "Марія"];
    if users.len() > 0 {
        for i in 0..users.len() {
            println!("{}", users[i]);
        }
    }
}

// Користувачі приходять параметром-зрізом. Якщо написати масив-літерал прямо
// тут, clippy скаже вже інше: `!users.is_empty()` для літерала завжди true
// (clippy::const_is_empty) — перевірка порожнечі має сенс для даних «ззовні».
fn idiomatic(users: &[&str]) {
    if !users.is_empty() {
        // is_empty() замість len() > 0
        for user in users {
            // без індексів
            println!("{user}");
        }
    }
    for (i, user) in users.iter().enumerate() {
        // якщо індекс справді потрібен
        println!("{i}: {user}");
    }
}

fn main() {
    php_style();
    let users = ["Іван", "Марія"]; // масив замість vec!
    idiomatic(&users);
}
