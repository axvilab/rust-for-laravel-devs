//! Exercise 4: your own `macro_rules! hashset` modelled on `hashmap!`

macro_rules! hashset {
    ($($v:expr),* $(,)?) => {{
        let mut s = std::collections::HashSet::new();
        $( s.insert($v); )*   // the same repetition, just one value instead of a pair
        s
    }};
}

fn main() {
    let tags = hashset!["rust", "laravel", "rust", "php",];
    let mut sorted: Vec<_> = tags.into_iter().collect();
    sorted.sort();
    println!("{sorted:?}"); // ["laravel", "php", "rust"] — the duplicate is gone
}
