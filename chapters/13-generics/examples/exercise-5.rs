//! Вправа 5: `Iterator` для Фібоначчі; перші 10 через `take(10)`

struct Fib {
    a: u64,
    b: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.a;
        // checked_add: на ~93-му числі u64 переповниться — тоді ітератор закінчиться
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(current)
    }
}

fn main() {
    let first10: Vec<u64> = Fib { a: 0, b: 1 }.take(10).collect();
    println!("{first10:?}"); // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    println!("усього в u64: {}", Fib { a: 0, b: 1 }.count());
}
