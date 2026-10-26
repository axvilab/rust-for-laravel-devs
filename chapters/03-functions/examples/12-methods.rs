//! Methods in `impl`: `&self` reads, `&mut self` changes

struct Counter {
    value: u32,
}

impl Counter {
    fn increment(&mut self) {
        // &mut self — the method changes the struct
        self.value += 1;
    }
    fn get(&self) -> u32 {
        // &self — the method only reads
        self.value
    }
}

fn main() {
    let mut c = Counter { value: 0 };
    c.increment();
    c.increment();
    println!("{}", c.get()); // 2
}
