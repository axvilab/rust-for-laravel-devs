//! Декларативний макрос `hashmap!`: `$k:expr`, повтор `$( ... ),*`, кома в кінці `$(,)?`

macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

fn main() {
    let stock = hashmap! { "ноутбук" => 5, "миша" => 40 };
    let mut keys: Vec<_> = stock.keys().collect();
    keys.sort();
    println!("{keys:?} {}", stock["ноутбук"]);
}
