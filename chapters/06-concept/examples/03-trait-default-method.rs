//! Трейт з реалізацією за замовчуванням — це PHP-трейт з перевіркою типів

trait HasTimestamps {
    fn created_at(&self) -> u64;
    fn age_seconds(&self, now: u64) -> u64 {
        // є тіло — дефолтна реалізація
        now - self.created_at()
    }
}

struct User {
    created_at: u64,
}

// Реалізуємо тільки created_at — age_seconds отримуємо безкоштовно.
impl HasTimestamps for User {
    fn created_at(&self) -> u64 {
        self.created_at
    }
}

fn main() {
    let u = User {
        created_at: 1_700_000_000,
    };
    println!("{} с", u.age_seconds(1_700_000_090)); // 90 с
}
