//! A `std::sync::Mutex` isn't held across `.await`: close it in a block or use `tokio::sync::Mutex`

use std::sync::Arc;

async fn save() {
    tokio::task::yield_now().await; // "a database query"
}

async fn query() -> u32 {
    tokio::task::yield_now().await;
    42
}

#[tokio::main]
async fn main() {
    let state = Arc::new(std::sync::Mutex::new(Vec::<u32>::new()));

    // GOOD 1: don't hold the lock across await
    let s = Arc::clone(&state);
    tokio::spawn(async move {
        let result = query().await;
        s.lock().unwrap().push(result);
    })
    .await
    .unwrap();

    // GOOD 1b: the lock is closed in a BLOCK before .await (the compiler ignores drop(guard))
    let s = Arc::clone(&state);
    tokio::spawn(async move {
        {
            let mut guard = s.lock().unwrap();
            guard.push(1);
        } // the guard is destroyed here
        save().await; // OK
    })
    .await
    .unwrap();

    // GOOD 2: if you must hold it across await — tokio::sync::Mutex
    let async_state = Arc::new(tokio::sync::Mutex::new(Vec::<u32>::new()));
    let s = Arc::clone(&async_state);
    tokio::spawn(async move {
        let mut guard = s.lock().await; // async lock
        save().await;
        guard.push(7);
    })
    .await
    .unwrap();

    // Even here: `println!("{:?} {:?}", state.lock().unwrap(), async_state.lock().await)`
    // would hold the std lock across .await — clippy catches that with await_holding_lock.
    let async_items = async_state.lock().await.clone();
    println!("{:?} {:?}", state.lock().unwrap(), async_items);
}
