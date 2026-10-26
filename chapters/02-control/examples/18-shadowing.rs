//! Затінення: нова змінна з тим самим іменем (і, можливо, іншим типом)

fn main() {
    let input = "42"; // &str
    let input: i32 = input.parse().unwrap(); // тепер i32, старий &str недоступний
    let input = input * 2; // 84
    println!("{input}");

    // той самий parse через турборибу:
    let n = "7".parse::<i32>().unwrap();
    println!("{n}");
}
