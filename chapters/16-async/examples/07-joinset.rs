//! `JoinSet` hands out results as they finish, not in order

use std::time::Duration;
use tokio::task::JoinSet;

async fn fetch(id: u64) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(50 * (5 - id))).await; // the first one is the slowest
    if id == 3 {
        return Err(format!("user {id} not found"));
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
            Ok(Ok(user)) => println!("ok: {user}"),
            Ok(Err(e)) => println!("error: {e}"),
            Err(join_err) => println!("task failed: {join_err}"),
        }
    }
    // order: user4, error 3, user2, user1

    // if the order doesn't matter — just wait for all of them
    let mut set = JoinSet::new();
    for id in [1, 2] {
        set.spawn(fetch(id));
    }
    println!("{:?}", set.join_all().await);
}
