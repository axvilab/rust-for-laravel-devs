//! An associated type `type Item`: implement `next` — get all the iterator methods

struct Countdown {
    left: u32,
}

impl Iterator for Countdown {
    type Item = u32; // "my elements are u32"

    fn next(&mut self) -> Option<u32> {
        // the only required method
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some(self.left + 1)
    }
}

fn main() {
    let v: Vec<u32> = Countdown { left: 3 }.collect(); // [3, 2, 1]
    println!("{v:?}");
    let sum: u32 = Countdown { left: 10 }.filter(|n| n % 2 == 1).sum(); // the methods come free
    println!("{sum}"); // 25
}
