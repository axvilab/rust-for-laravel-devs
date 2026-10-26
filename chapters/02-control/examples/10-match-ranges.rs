//! `match` with ranges and a guard

fn main() {
    for status in [204, 301, 418, 503, 42] {
        let category = match status {
            200..=299 => "success",
            300..=399 => "redirect",
            400..=499 => "client error",
            n if n >= 500 => "server error", // guard — an extra condition
            _ => "weird",
        };
        println!("{status}: {category}");
    }
}
