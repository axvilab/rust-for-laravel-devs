//! `while` — like in PHP, just without parentheses around the condition

fn main() {
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
}
