//! `self` is the instance (`$this`), `Self` is the type (`self::`/`static::` in PHP)

struct Counter {
    value: u32,
}

impl Counter {
    const LIMIT: u32 = 3; // a constant of the type, like a class const

    fn new() -> Self {
        // Self — the type, i.e. Counter
        Self { value: 0 }
    }

    fn bump(&mut self) {
        // self — this instance, i.e. $this
        if self.value < Self::LIMIT {
            // Self:: — like self:: / static:: in PHP
            self.value += 1;
        }
    }
}

fn main() {
    let mut c = Counter::new();
    for _ in 0..5 {
        c.bump();
    }
    println!("{}", c.value); // 3
}
