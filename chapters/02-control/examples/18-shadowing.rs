//! Shadowing: a new variable with the same name (and possibly another type)

fn main() {
    let input = "42"; // &str
    let input: i32 = input.parse().unwrap(); // now i32, the old &str is unreachable
    let input = input * 2; // 84
    println!("{input}");

    // the same parse with the turbofish:
    let n = "7".parse::<i32>().unwrap();
    println!("{n}");
}
