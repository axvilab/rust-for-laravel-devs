//! Свій трейт для чужого типу: метод з'являється навіть у `String` (як `Collection::macro`)

trait Slugify {
    fn slug(&self) -> String;
}

impl Slugify for String {
    fn slug(&self) -> String {
        self.to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    }
}

fn main() {
    let title = String::from("Rust для Laravel Розробника");
    println!("{}", title.slug()); // rust-для-laravel-розробника
}
