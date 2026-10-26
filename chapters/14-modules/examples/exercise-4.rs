//! Exercise 4: what `cargo clippy` suggests most often (already fixed) and `cargo fmt`
//!
//! Every line below is the "after". The comment says what the "before" was and the lint name.
//! This whole repository is checked in CI with
//! `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.

fn print_users(users: &[&str]) {
    // was &Vec<&str> → ptr_arg
    if !users.is_empty() {
        // was users.len() > 0 → len_zero
        for user in users {
            // was for i in 0..users.len() { users[i] } → needless_range_loop
            println!("{user}");
        }
    }
}

fn main() {
    let users = ["Ivan", "Maria"]; // was vec![...] → useless_vec
    print_users(&users);
    let n = Some(-5_i32);
    let abs = n.map(i32::abs); // was .map(|x| x.abs()) → redundant_closure_for_method_calls (pedantic)
    let mut name = String::from("Vitalii");
    name.push('!'); // was push_str("!") → single_char_add_str
    println!("{abs:?} {name} {}", users.first().unwrap_or(&"")); // was get(0) → get_first
}
