//! Exercise 5: an `Iterator` for Fibonacci; the first 10 with `take(10)`

struct Fib {
    a: u64,
    b: u64,
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.a;
        // checked_add: around the 93rd number u64 overflows — then the iterator ends
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(current)
    }
}

fn main() {
    let first10: Vec<u64> = Fib { a: 0, b: 1 }.take(10).collect();
    println!("{first10:?}"); // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    println!("total in u64: {}", Fib { a: 0, b: 1 }.count());
}
