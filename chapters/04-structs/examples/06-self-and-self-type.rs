//! `self` — екземпляр (`$this`), `Self` — тип (`self::`/`static::` у PHP)

struct Counter {
    value: u32,
}

impl Counter {
    const LIMIT: u32 = 3; // константа типу, як const у класі

    fn new() -> Self {
        // Self — тип, тобто Counter
        Self { value: 0 }
    }

    fn bump(&mut self) {
        // self — цей екземпляр, тобто $this
        if self.value < Self::LIMIT {
            // Self:: — як self:: / static:: у PHP
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
