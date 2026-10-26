//! Асоційований тип `type Item`: реалізуй `next` — отримай усі методи ітераторів

struct Countdown {
    left: u32,
}

impl Iterator for Countdown {
    type Item = u32; // «мої елементи — u32»

    fn next(&mut self) -> Option<u32> {
        // єдиний обов'язковий метод
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
    let sum: u32 = Countdown { left: 10 }.filter(|n| n % 2 == 1).sum(); // методи безкоштовно
    println!("{sum}"); // 25
}
