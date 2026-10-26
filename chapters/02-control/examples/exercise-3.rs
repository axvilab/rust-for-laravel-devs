//! Exercise 3: numbers in 0..=100 divisible by 3 but not by 5 (`continue`)

fn main() {
    for n in 0..=100 {
        if n % 3 != 0 || n % 5 == 0 {
            continue; // skip everything that doesn't fit
        }
        print!("{n} ");
    }
    println!();
}
