//! `JoinSet` віддає результати в міру готовності, а не по порядку

use std::time::Duration;
use tokio::task::JoinSet;

async fn fetch(id: u64) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(50 * (5 - id))).await; // перша — найповільніша
    if id == 3 {
        return Err(format!("користувача {id} нема"));
    }
    Ok(format!("user{id}"))
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    for id in 1..=4 {
        set.spawn(fetch(id));
    }
    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(user)) => println!("ок: {user}"),
            Ok(Err(e)) => println!("помилка: {e}"),
            Err(join_err) => println!("задача впала: {join_err}"),
        }
    }
    // порядок: user4, помилка 3, user2, user1

    // якщо порядок не важливий — просто дочекатися всіх
    let mut set = JoinSet::new();
    for id in [1, 2] {
        set.spawn(fetch(id));
    }
    println!("{:?}", set.join_all().await);
}
