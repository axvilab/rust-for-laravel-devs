//! `for` over ranges and collections, `&users`, `.iter().enumerate()`

fn main() {
    // With an empty body (`for i in 0..10 { }`) the compiler warns about the
    // unused `i`, so we print the values to see the ranges.
    for i in 0..10 {
        print!("{i} "); // 0,1,...,9  — like range(0, 9)
    }
    println!();
    for i in 0..=10 {
        print!("{i} "); // 0,1,...,10 — inclusive
    }
    println!();
    for i in (0..10).rev() {
        print!("{i} "); // in reverse order
    }
    println!();

    let users = vec!["Ivan", "Maria", "Oleh"];
    for user in &users {
        // & — we borrow, not take (the ownership and borrowing chapters)
        println!("{user}");
    }

    for (i, user) in users.iter().enumerate() {
        // with an index, like foreach with $key
        println!("{i}: {user}");
    }
}
