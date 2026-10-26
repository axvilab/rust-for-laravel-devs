//! Clone — an explicit (and deep) copy

fn main() {
    let a = String::from("hello");
    let b = a.clone(); // allocated new memory, copied the bytes
    println!("{a} {b}"); // ok, both alive

    let tags = vec!["rust".to_string(), "php".to_string()];
    let copy = tags.clone(); // a copy of the vector AND of every string in it
    println!("{tags:?} {copy:?}");
}
