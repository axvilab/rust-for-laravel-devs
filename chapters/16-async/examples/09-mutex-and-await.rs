//! `std::sync::Mutex` не тримають через `.await`: закрити блоком або `tokio::sync::Mutex`

use std::sync::Arc;

async fn save() {
    tokio::task::yield_now().await; // «запит у базу»
}

async fn query() -> u32 {
    tokio::task::yield_now().await;
    42
}

#[tokio::main]
async fn main() {
    let state = Arc::new(std::sync::Mutex::new(Vec::<u32>::new()));

    // ДОБРЕ 1: не тримати замок через await
    let s = Arc::clone(&state);
    tokio::spawn(async move {
        let result = query().await;
        s.lock().unwrap().push(result);
    })
    .await
    .unwrap();

    // ДОБРЕ 1б: замок закрито БЛОКОМ до .await (drop(guard) компілятор не врахує)
    let s = Arc::clone(&state);
    tokio::spawn(async move {
        {
            let mut guard = s.lock().unwrap();
            guard.push(1);
        } // guard знищено тут
        save().await; // OK
    })
    .await
    .unwrap();

    // ДОБРЕ 2: якщо треба тримати через await — tokio::sync::Mutex
    let async_state = Arc::new(tokio::sync::Mutex::new(Vec::<u32>::new()));
    let s = Arc::clone(&async_state);
    tokio::spawn(async move {
        let mut guard = s.lock().await; // async lock
        save().await;
        guard.push(7);
    })
    .await
    .unwrap();

    // Навіть тут: `println!("{:?} {:?}", state.lock().unwrap(), async_state.lock().await)`
    // тримав би std-замок через .await — clippy ловить це лінтом await_holding_lock.
    let async_items = async_state.lock().await.clone();
    println!("{:?} {:?}", state.lock().unwrap(), async_items);
}
