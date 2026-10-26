//! `async fn` у трейтах (Rust 1.75+) — без крейта async_trait

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    id: u64,
    name: String,
}

trait UserRepo {
    async fn find(&self, id: u64) -> Option<User>;
}

struct InMemoryRepo {
    users: HashMap<u64, User>,
}

impl UserRepo for InMemoryRepo {
    async fn find(&self, id: u64) -> Option<User> {
        tokio::task::yield_now().await; // уявний I/O
        self.users.get(&id).cloned()
    }
}

#[tokio::main]
async fn main() {
    let repo = InMemoryRepo {
        users: HashMap::from([(
            1,
            User {
                id: 1,
                name: "Віталій".into(),
            },
        )]),
    };
    println!("{:?}", repo.find(1).await);
    println!("{:?}", repo.find(2).await);
}
