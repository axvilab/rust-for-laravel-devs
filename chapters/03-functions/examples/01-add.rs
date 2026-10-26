//! A plain function: parameter types are mandatory, `->` for the result

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(2, 3)); // 5
}
