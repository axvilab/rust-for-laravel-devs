//! `while` — як у PHP, тільки без дужок навколо умови

fn main() {
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
}
