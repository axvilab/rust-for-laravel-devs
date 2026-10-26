//! Вправа 4: що `cargo clippy` пропонує найчастіше (уже виправлено) і `cargo fmt`
//!
//! Кожен рядок нижче — «після». У коментарі — що було «до» і назва лінту.
//! Весь цей репозиторій перевіряється в CI командами
//! `cargo fmt --check` і `cargo clippy --all-targets -- -D warnings`.

fn print_users(users: &[&str]) {
    // було &Vec<&str> → ptr_arg
    if !users.is_empty() {
        // було users.len() > 0 → len_zero
        for user in users {
            // було for i in 0..users.len() { users[i] } → needless_range_loop
            println!("{user}");
        }
    }
}

fn main() {
    let users = ["Іван", "Марія"]; // було vec![...] → useless_vec
    print_users(&users);
    let n = Some(-5_i32);
    let abs = n.map(i32::abs); // було .map(|x| x.abs()) → redundant_closure_for_method_calls (pedantic)
    let mut name = String::from("Віталій");
    name.push('!'); // було push_str("!") → single_char_add_str
    println!("{abs:?} {name} {}", users.first().unwrap_or(&"")); // було get(0) → get_first
}
