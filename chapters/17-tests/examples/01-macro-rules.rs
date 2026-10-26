//! A declarative macro `hashmap!`: `$k:expr`, the repetition `$( ... ),*`, a trailing comma `$(,)?`

macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

fn main() {
    let stock = hashmap! { "laptop" => 5, "mouse" => 40 };
    let mut keys: Vec<_> = stock.keys().collect();
    keys.sort();
    println!("{keys:?} {}", stock["laptop"]);
}
