//! Вправа 4: свій `macro_rules! hashset` за зразком `hashmap!`

macro_rules! hashset {
    ($($v:expr),* $(,)?) => {{
        let mut s = std::collections::HashSet::new();
        $( s.insert($v); )*   // той самий повтор, тільки одне значення замість пари
        s
    }};
}

fn main() {
    let tags = hashset!["rust", "laravel", "rust", "php",];
    let mut sorted: Vec<_> = tags.into_iter().collect();
    sorted.sort();
    println!("{sorted:?}"); // ["laravel", "php", "rust"] — дубль зник
}
