//! Exercise 1: `count_vowels(&str)` works for both a String and a literal

fn count_vowels(s: &str) -> usize {
    // vowels (lowercase)
    const VOWELS: &str = "aeiou";
    s.chars()
        .filter(|c| VOWELS.contains(c.to_lowercase().next().unwrap_or(*c)))
        .count()
}

fn main() {
    let owned = String::from("Vitalii");
    println!("{}", count_vowels(&owned)); // &String → &str automatically
    println!("{}", count_vowels("Dnipro")); // a literal is already &str
    println!("{owned}"); // no clone(), owned is alive
}
