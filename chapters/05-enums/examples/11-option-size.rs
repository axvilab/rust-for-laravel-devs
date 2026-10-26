//! The size of Option: for references and Box the wrapper is free

use std::mem::size_of;

struct User {
    id: u64,
    name: String,
}

fn main() {
    println!("&User         = {} bytes", size_of::<&User>());
    println!("Option<&User> = {} bytes", size_of::<Option<&User>>()); // the same
    println!("Box<User>         = {} bytes", size_of::<Box<User>>());
    println!(
        "Option<Box<User>> = {} bytes",
        size_of::<Option<Box<User>>>()
    ); // the same
    println!("u64         = {} bytes", size_of::<u64>());
    println!("Option<u64> = {} bytes", size_of::<Option<u64>>()); // here a flag is needed
}
