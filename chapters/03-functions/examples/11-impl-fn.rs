//! Your own function that takes a closure: `impl Fn(i32) -> i32`

fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    println!("{}", apply_twice(|n| n + 10, 5)); // 25
    println!("{}", apply_twice(|n| n * n, 3)); // 81
}
