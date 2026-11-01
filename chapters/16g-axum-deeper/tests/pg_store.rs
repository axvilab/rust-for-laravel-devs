//! Needs Postgres: `DATABASE_URL=postgres://postgres:pg@localhost:5432/postgres cargo test`.
//! `#[sqlx::test]` creates a throwaway database for the test.

use ch16g_axum_deeper::pg_store::PgStore;
use sqlx::PgPool;
use tower_sessions::{
    SessionStore,
    cookie::time::{Duration, OffsetDateTime},
    session::{Id, Record},
};

fn record(expires_in: Duration) -> Record {
    Record {
        id: Id::default(),
        data: [("user".to_string(), serde_json::json!("v@example.com"))].into(),
        expiry_date: OffsetDateTime::now_utc() + expires_in,
    }
}

#[sqlx::test]
async fn create_load_save_delete(pool: PgPool) {
    let store = PgStore::new(pool);
    store.migrate().await.unwrap();

    let mut rec = record(Duration::hours(1));
    store.create(&mut rec).await.unwrap();
    let loaded = store.load(&rec.id).await.unwrap().unwrap();
    assert_eq!(loaded.data["user"], "v@example.com");

    rec.data.insert("visits".into(), serde_json::json!(3));
    store.save(&rec).await.unwrap();
    assert_eq!(
        store.load(&rec.id).await.unwrap().unwrap().data["visits"],
        3
    );

    store.delete(&rec.id).await.unwrap();
    assert!(store.load(&rec.id).await.unwrap().is_none());
}

#[sqlx::test]
async fn expired_sessions_are_invisible_and_collected(pool: PgPool) {
    let store = PgStore::new(pool);
    store.migrate().await.unwrap();
    let mut old = record(-Duration::minutes(1));
    store.create(&mut old).await.unwrap();
    assert!(store.load(&old.id).await.unwrap().is_none());
    assert_eq!(store.delete_expired().await.unwrap(), 1);
}
