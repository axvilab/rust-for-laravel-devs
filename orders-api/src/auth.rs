//! Authentication as an extractor: a handler that has `AuthUser` in its
//! signature is protected — you cannot forget it, because without it you do
//! not have `user.sub`.

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppError, state::AppState};

pub const ROLE_ADMIN: &str = "admin";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
    pub role: String,
}

impl Claims {
    pub fn is_admin(&self) -> bool {
        self.role == ROLE_ADMIN
    }
}

#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

fn bearer(parts: &Parts) -> Option<&str> {
    parts
        .headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}

fn decode_claims(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(), // HS256, checks `exp`
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}

// Axum 0.8: `async fn` directly in the impl, no `#[async_trait]`.
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let token = bearer(parts).ok_or(AppError::Unauthorized)?;
        Ok(AuthUser(decode_claims(token, &state.jwt_secret)?))
    }
}

/// Axum 0.8: `Option<AuthUser>` needs this impl explicitly.
/// No header -> `None` (anonymous); a header with a bad token -> 401.
impl OptionalFromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, AppError> {
        match bearer(parts) {
            None => Ok(None),
            Some(token) => Ok(Some(AuthUser(decode_claims(token, &state.jwt_secret)?))),
        }
    }
}

/// `AdminUser` in a signature = `middleware('can:admin')`.
#[derive(Debug, Clone)]
pub struct AdminUser(pub Claims);

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, AppError> {
        let AuthUser(c) =
            <AuthUser as FromRequestParts<AppState>>::from_request_parts(parts, state).await?;
        if !c.is_admin() {
            return Err(AppError::Forbidden);
        }
        Ok(AdminUser(c))
    }
}

pub fn issue_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    ttl_hours: i64,
) -> Result<String, AppError> {
    let exp = chrono::Utc::now() + chrono::Duration::hours(ttl_hours);
    let claims = Claims {
        sub: user_id,
        role: role.to_owned(),
        exp: usize::try_from(exp.timestamp()).unwrap_or(0),
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

// ---------------------------------------------------------------------------
// Passwords: argon2, and ALWAYS off the async runtime threads.
// Hashing takes ~100 ms on purpose; on a runtime thread it would stall every
// other request scheduled on that thread.
// ---------------------------------------------------------------------------

pub async fn hash_password(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|h| h.to_string())
            .map_err(|e| anyhow::anyhow!("argon2 hash: {e}"))
    })
    .await
    .map_err(anyhow::Error::from)?
    .map_err(AppError::from)
}

pub async fn verify_password(password: String, hash: String) -> Result<bool, AppError> {
    let ok = tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash)
            .map(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok()
            })
            .unwrap_or(false)
    })
    .await
    .map_err(anyhow::Error::from)?;
    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_roundtrip_and_wrong_secret() {
        let id = Uuid::new_v4();
        let t = issue_token(id, "user", "secret-a", 1).unwrap();
        let c = decode_claims(&t, "secret-a").unwrap();
        assert_eq!(c.sub, id);
        assert!(!c.is_admin());
        assert!(matches!(
            decode_claims(&t, "secret-b"),
            Err(AppError::Unauthorized)
        ));
    }

    #[test]
    fn expired_token_is_rejected() {
        let t = issue_token(Uuid::new_v4(), "user", "s", -2).unwrap();
        assert!(matches!(
            decode_claims(&t, "s"),
            Err(AppError::Unauthorized)
        ));
    }

    #[tokio::test]
    async fn argon2_roundtrip() {
        let h = hash_password("correct horse".into()).await.unwrap();
        assert!(h.starts_with("$argon2id$"));
        assert!(
            verify_password("correct horse".into(), h.clone())
                .await
                .unwrap()
        );
        assert!(!verify_password("wrong".into(), h).await.unwrap());
        assert!(
            !verify_password("x".into(), "not-a-hash".into())
                .await
                .unwrap()
        );
    }
}
