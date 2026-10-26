//! Вправа 4: порядок знищення — зворотний до порядку оголошення

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    {
        let _a = Noisy("перший");
        let _b = Noisy("другий");
        let _c = Noisy("третій");
        println!("кінець блоку");
    }
    // drop третій / drop другий / drop перший — як стек: останній створений знищується першим
}
