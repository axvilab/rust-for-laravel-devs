//! `AppState` — the service container. It is cloned for every request, so
//! everything inside must be cheap to clone: pools are `Arc` inside already,
//! strings and big structs go into `Arc`.

use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{
    config::Config,
    repo::{OrderRepo, UserRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub repo: OrderRepo,
    pub users: UserRepo,
    pub jwt_secret: Arc<str>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(db: PgPool, config: Config) -> Self {
        Self {
            repo: OrderRepo::new(db.clone()),
            users: UserRepo::new(db.clone()),
            jwt_secret: Arc::from(config.jwt_secret.as_str()),
            config: Arc::new(config),
            db,
        }
    }
}

/// Lets a handler ask for `State<PgPool>` instead of the whole `AppState`.
impl FromRef<AppState> for PgPool {
    fn from_ref(s: &AppState) -> PgPool {
        s.db.clone()
    }
}
