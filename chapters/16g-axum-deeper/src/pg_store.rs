//! A Postgres session store for tower-sessions 0.15 on sqlx 0.9.
//!
//! `tower-sessions-sqlx-store` 0.15 still depends on sqlx 0.8 and tower-sessions-core 0.14,
//! so it does not plug into tower-sessions 0.15 + sqlx 0.9. The trait is small enough to
//! implement by hand: four methods.

use async_trait::async_trait;
use sqlx::PgPool;
use tower_sessions::{
    SessionStore,
    session::{Id, Record},
    session_store::{Error, Result},
};

#[derive(Debug, Clone)]
pub struct PgStore {
    pool: PgPool,
}

/// sqlx errors become the store's "backend" error.
fn backend(e: sqlx::Error) -> Error {
    Error::Backend(e.to_string())
}

impl PgStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// In a real project this is a migration file.
    pub async fn migrate(&self) -> sqlx::Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sessions (
                id         text PRIMARY KEY,
                data       text NOT NULL,
                expires_at bigint NOT NULL
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Laravel's session garbage collector; call it from a periodic task.
    pub async fn delete_expired(&self) -> sqlx::Result<u64> {
        let now = time_now();
        let res = sqlx::query("DELETE FROM sessions WHERE expires_at < $1")
            .bind(now)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}

fn time_now() -> i64 {
    let since_epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    i64::try_from(since_epoch.as_secs()).unwrap_or(i64::MAX)
}

#[async_trait]
impl SessionStore for PgStore {
    /// A new session: insert, and pick a new id if this one is somehow taken.
    async fn create(&self, record: &mut Record) -> Result<()> {
        loop {
            let data =
                serde_json::to_string(&record.data).map_err(|e| Error::Encode(e.to_string()))?;
            let inserted = sqlx::query(
                "INSERT INTO sessions (id, data, expires_at) VALUES ($1, $2, $3)
                 ON CONFLICT (id) DO NOTHING",
            )
            .bind(record.id.to_string())
            .bind(data)
            .bind(record.expiry_date.unix_timestamp())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
            if inserted.rows_affected() == 1 {
                return Ok(());
            }
            record.id = Id::default(); // a collision: try another random id
        }
    }

    async fn save(&self, record: &Record) -> Result<()> {
        let data = serde_json::to_string(&record.data).map_err(|e| Error::Encode(e.to_string()))?;
        sqlx::query(
            "INSERT INTO sessions (id, data, expires_at) VALUES ($1, $2, $3)
             ON CONFLICT (id) DO UPDATE SET data = EXCLUDED.data, expires_at = EXCLUDED.expires_at",
        )
        .bind(record.id.to_string())
        .bind(data)
        .bind(record.expiry_date.unix_timestamp())
        .execute(&self.pool)
        .await
        .map_err(backend)?;
        Ok(())
    }

    /// Expired rows are treated as missing, even before `delete_expired` removes them.
    async fn load(&self, id: &Id) -> Result<Option<Record>> {
        let row: Option<(String, i64)> = sqlx::query_as(
            "SELECT data, expires_at FROM sessions WHERE id = $1 AND expires_at > $2",
        )
        .bind(id.to_string())
        .bind(time_now())
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        let Some((data, expires_at)) = row else {
            return Ok(None);
        };
        let data = serde_json::from_str(&data).map_err(|e| Error::Decode(e.to_string()))?;
        let expiry_date =
            tower_sessions::cookie::time::OffsetDateTime::from_unix_timestamp(expires_at)
                .map_err(|e| Error::Decode(e.to_string()))?;
        Ok(Some(Record {
            id: *id,
            data,
            expiry_date,
        }))
    }

    async fn delete(&self, id: &Id) -> Result<()> {
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(())
    }
}
