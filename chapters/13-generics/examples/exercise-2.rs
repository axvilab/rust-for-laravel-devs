//! Exercise 2: `Cache<K, V>` — HashMap requires `Eq + Hash` from the key
//!
//! Without bounds the compiler says on `self.map.get(key)`:
//!   error[E0599]: the method `get` exists for struct `HashMap<K, V>`, but its
//!   trait bounds were not satisfied … `K: Eq`, `K: Hash`
//! Hence `impl<K: Eq + Hash, V> Cache<K, V>`.

use std::collections::HashMap;
use std::hash::Hash;

struct Cache<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    fn new() -> Self {
        Cache {
            map: HashMap::new(),
        }
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    fn set(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

fn main() {
    let mut users: Cache<u64, String> = Cache::new();
    users.set(1, "Vitalii".into());
    println!("{:?} {:?}", users.get(&1), users.get(&2));

    let mut rates: Cache<&str, f64> = Cache::new();
    rates.set("USD", 41.5);
    println!("{:?}", rates.get(&"USD"));
}
