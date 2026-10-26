//! Your own trait for someone else's type: the method shows up even on `String` (like `Collection::macro`)

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
    let title = String::from("Rust for Laravel Developers");
    println!("{}", title.slug()); // rust-for-laravel-developers
}
