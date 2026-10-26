//! A trait with a default implementation — a PHP trait with type checking

trait HasTimestamps {
    fn created_at(&self) -> u64;
    fn age_seconds(&self, now: u64) -> u64 {
        // has a body — a default implementation
        now - self.created_at()
    }
}

struct User {
    created_at: u64,
}

// We implement only created_at — age_seconds comes for free.
impl HasTimestamps for User {
    fn created_at(&self) -> u64 {
        self.created_at
    }
}

fn main() {
    let u = User {
        created_at: 1_700_000_000,
    };
    println!("{} s", u.age_seconds(1_700_000_090)); // 90 s
}
