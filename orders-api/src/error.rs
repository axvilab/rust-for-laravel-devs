//! `AppError` — one place that turns every failure into an HTTP response.
//! This is Laravel's `Handler::render()` as a single `match`.

use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::domain::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("не знайдено")]
    NotFound,
    #[error("потрібна автентифікація")]
    Unauthorized,
    #[error("недостатньо прав")]
    Forbidden,
    #[error("конфлікт: {0}")]
    Conflict(String),
    #[error("невалідний запит: {0}")]
    Validation(String),
    #[error("невалідне тіло запиту: {0}")]
    BadJson(#[from] JsonRejection),
    #[error("помилки валідації")]
    ValidationErrors(#[from] validator::ValidationErrors),
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("помилка бази даних")]
    Db(#[from] sqlx::Error),
    #[error("помилка токена")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("внутрішня помилка")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        use StatusCode as S;
        let (status, body) = match &self {
            Self::NotFound => (S::NOT_FOUND, json!({ "error": self.to_string() })),
            Self::Unauthorized => (S::UNAUTHORIZED, json!({ "error": "unauthorized" })),
            Self::Forbidden => (S::FORBIDDEN, json!({ "error": self.to_string() })),
            Self::Conflict(_) => (S::CONFLICT, json!({ "error": self.to_string() })),
            Self::Validation(_) => (S::BAD_REQUEST, json!({ "error": self.to_string() })),
            // Keep axum's own status: 400 for broken JSON, 415 for a wrong
            // content-type, 422 for JSON that does not match the type.
            Self::BadJson(e) => (e.status(), json!({ "error": e.body_text() })),
            Self::ValidationErrors(e) => (
                S::UNPROCESSABLE_ENTITY,
                json!({ "error": "validation", "fields": e }),
            ),
            Self::Domain(e) => (S::UNPROCESSABLE_ENTITY, json!({ "error": e.to_string() })),
            Self::Db(e) => {
                tracing::error!(error = ?e, "db error");
                (S::INTERNAL_SERVER_ERROR, json!({ "error": "internal" }))
            }
            // Token *decoding* failures are mapped to `Unauthorized` in the
            // extractor; reaching this arm means *encoding* failed — a server bug.
            Self::Jwt(e) => {
                tracing::error!(error = ?e, "jwt error");
                (S::INTERNAL_SERVER_ERROR, json!({ "error": "internal" }))
            }
            Self::Internal(e) => {
                tracing::error!(error = ?e, "internal error");
                (S::INTERNAL_SERVER_ERROR, json!({ "error": "internal" }))
            }
        };
        (status, Json(body)).into_response()
    }
}

/// Unique-violation (`23505`) -> 409, everything else stays a DB error.
pub fn conflict_on_unique(e: sqlx::Error, what: &str) -> AppError {
    match &e {
        sqlx::Error::Database(db) if db.is_unique_violation() => AppError::Conflict(what.into()),
        _ => AppError::Db(e),
    }
}
