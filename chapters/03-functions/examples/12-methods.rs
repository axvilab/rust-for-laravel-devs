//! Методи в `impl`: `&self` читає, `&mut self` змінює

struct Counter {
    value: u32,
}

impl Counter {
    fn increment(&mut self) {
        // &mut self — метод змінює структуру
        self.value += 1;
    }
    fn get(&self) -> u32 {
        // &self — метод тільки читає
        self.value
    }
}

fn main() {
    let mut c = Counter { value: 0 };
    c.increment();
    c.increment();
    println!("{}", c.get()); // 2
}
