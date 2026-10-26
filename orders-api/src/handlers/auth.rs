//! Register / login. Passwords are hashed and verified in `spawn_blocking`.

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{self, AuthUser},
    domain::Email,
    error::AppError,
    state::AppState,
    validation::ValidatedJson,
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    pub email: Email,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(length(min = 1, max = 254))]
    pub email: String,
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserRes {
    pub id: Uuid,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct TokenRes {
    pub token: String,
    pub token_type: &'static str,
}

pub async fn register(
    State(st): State<AppState>,
    ValidatedJson(b): ValidatedJson<RegisterReq>,
) -> Result<(StatusCode, Json<UserRes>), AppError> {
    let hash = auth::hash_password(b.password).await?;
    let user = st.users.create(&b.email, &hash).await?; // duplicate -> 409
    Ok((
        StatusCode::CREATED,
        Json(UserRes {
            id: user.id,
            email: user.email,
            role: user.role,
        }),
    ))
}

pub async fn login(
    State(st): State<AppState>,
    ValidatedJson(b): ValidatedJson<LoginReq>,
) -> Result<Json<TokenRes>, AppError> {
    // NB: returning early for an unknown email is faster than a wrong password,
    // which leaks whether the email exists (timing). Acceptable for the book;
    // in production verify against a dummy hash here too.
    let user = st
        .users
        .find_by_email(&b.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !auth::verify_password(b.password, user.password_hash).await? {
        return Err(AppError::Unauthorized);
    }

    let token = auth::issue_token(user.id, &user.role, &st.jwt_secret, st.config.jwt_ttl_hours)?;
    Ok(Json(TokenRes {
        token,
        token_type: "Bearer",
    }))
}

/// `GET /me` — `Option<AuthUser>`: anonymous is fine, a bad token is not.
pub async fn me(user: Option<AuthUser>) -> Json<serde_json::Value> {
    Json(match user {
        Some(AuthUser(c)) => serde_json::json!({ "id": c.sub, "role": c.role }),
        None => serde_json::json!({ "id": null, "role": "guest" }),
    })
}
