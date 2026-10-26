//! Вправа 1: `count_vowels(&str)` працює і для String, і для літерала

fn count_vowels(s: &str) -> usize {
    // голосні української та англійської (у нижньому регістрі)
    const VOWELS: &str = "аеєиіїоуюяaeiou";
    s.chars()
        .filter(|c| VOWELS.contains(c.to_lowercase().next().unwrap_or(*c)))
        .count()
}

fn main() {
    let owned = String::from("Віталій");
    println!("{}", count_vowels(&owned)); // &String → &str автоматично
    println!("{}", count_vowels("Дніпро")); // літерал — уже &str
    println!("{owned}"); // жодного clone(), owned живий
}
