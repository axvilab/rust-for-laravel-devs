//! What `cargo clippy` says about "PHP-style" loops — and how to rewrite them
//!
//! The first half is PHP-style on purpose; three lints are allowed on the function,
//! otherwise CI (`clippy -D warnings`) would fail. Remove `#[allow(...)]` and run
//! `cargo clippy -p ch02-control --example 07-clippy-loops` to see all three hints.

#[allow(clippy::len_zero, clippy::needless_range_loop, clippy::useless_vec)]
fn php_style() {
    let users = vec!["Ivan", "Maria"];
    if users.len() > 0 {
        for i in 0..users.len() {
            println!("{}", users[i]);
        }
    }
}

// The users come in as a slice parameter. With an array literal right here
// clippy would say something else: `!users.is_empty()` on a literal is always
// true (clippy::const_is_empty) — an emptiness check makes sense for outside data.
fn idiomatic(users: &[&str]) {
    if !users.is_empty() {
        // is_empty() instead of len() > 0
        for user in users {
            // no indexes
            println!("{user}");
        }
    }
    for (i, user) in users.iter().enumerate() {
        // if you really need the index
        println!("{i}: {user}");
    }
}

fn main() {
    php_style();
    let users = ["Ivan", "Maria"]; // an array instead of vec!
    idiomatic(&users);
}
