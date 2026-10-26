//! Вправа 7: одна змінна стала `_` — її значення знищується одразу

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    {
        let _a = Noisy("перший");
        let _ = Noisy("другий"); // `_` — не змінна: значення без власника, drop негайно
        let _c = Noisy("третій");
        println!("кінець блоку");
    }
    // drop другий / кінець блоку / drop третій / drop перший
}
