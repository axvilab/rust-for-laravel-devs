//! `unwrap_or` рахує дефолт завжди, `unwrap_or_else` — лише при None
//
// clippy тут (як і пише книга) радить коротший запис `unwrap_or_else(load_default_name)`
// — лінт redundant_closure. Форму з `||` залишено навмисно, тому лінт вимкнено.
#![allow(clippy::redundant_closure)]

fn find_name() -> Option<String> {
    Some("Віталій".to_string())
}

fn load_default_name() -> String {
    println!("(дорогий запит за дефолтом...)");
    "Гість".to_string()
}

fn main() {
    let a = find_name().unwrap_or(load_default_name()); // викликається ЗАВЖДИ
    let b = find_name().unwrap_or_else(|| load_default_name()); // тільки якщо None
    let c = find_name().unwrap_or_default(); // None → ""
    println!("{a} {b} {c}");
}
