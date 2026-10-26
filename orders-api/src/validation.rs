//! `ValidatedJson<T>` = `Json<T>` + `validate()` — a `FormRequest` you write once.

use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, AppError> {
        // `?` turns `JsonRejection` into `AppError::BadJson` via `#[from]`
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?; // -> AppError::ValidationErrors -> 422 with field list
        Ok(ValidatedJson(value))
    }
}
