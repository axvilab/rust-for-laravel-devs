//! Extractors, deeper: rejections in your own JSON format via `#[derive(FromRequest)]`,
//! a custom `FromRequestParts`, `Option<T>` and `Result<T, Rejection>`, request extensions.

use axum::{
    Extension, Json, Router,
    extract::{
        FromRequest, FromRequestParts, Request,
        rejection::{JsonRejection, PathRejection},
    },
    http::{StatusCode, request::Parts},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

// --- one error type for everything --------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Path(#[from] PathRejection),
    #[error(transparent)]
    Json(#[from] JsonRejection),
    #[error("unsupported api version: {0}")]
    BadVersion(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::Path(e) => e.status(), // keep axum's code: 400 for a bad id
            Self::Json(e) => e.status(), // 400 / 415 / 422
            Self::BadVersion(_) => StatusCode::NOT_ACCEPTABLE,
        };
        let body = Json(serde_json::json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}

// --- wrappers: the same extractors, our rejection --------------------------------------

/// `Path<T>`, but a failure is `AppError` (JSON) instead of plain text.
#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(AppError))]
pub struct AppPath<T>(pub T);

/// `Json<T>` for input AND output, with `AppError` on bad input.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T: Serialize> IntoResponse for AppJson<T> {
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}

// --- a custom extractor from headers -----------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiVersion {
    V1,
    V2,
}

/// `x-api-version: 2` -> `ApiVersion::V2`; no header -> V1; anything else -> 406.
impl<S: Send + Sync> FromRequestParts<S> for ApiVersion {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, AppError> {
        let raw = parts
            .headers
            .get("x-api-version")
            .map(|v| v.to_str().unwrap_or("?"));
        match raw {
            None | Some("1") => Ok(ApiVersion::V1),
            Some("2") => Ok(ApiVersion::V2),
            Some(other) => Err(AppError::BadVersion(other.to_owned())),
        }
    }
}

// --- handlers --------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct NewOrder {
    pub sku: String,
    pub qty: u32,
}

async fn show(AppPath(id): AppPath<u64>, version: ApiVersion) -> AppJson<serde_json::Value> {
    AppJson(match version {
        ApiVersion::V1 => serde_json::json!({ "id": id }),
        ApiVersion::V2 => serde_json::json!({ "data": { "id": id }, "version": 2 }),
    })
}

async fn create(AppJson(order): AppJson<NewOrder>) -> (StatusCode, AppJson<NewOrder>) {
    (StatusCode::CREATED, AppJson(order))
}

/// `Result<Json<T>, JsonRejection>`: decide in the handler what a bad body means.
async fn lenient(body: Result<Json<NewOrder>, JsonRejection>) -> String {
    match body {
        Ok(Json(o)) => format!("{} x {}", o.qty, o.sku),
        Err(JsonRejection::MissingJsonContentType(_)) => "send JSON, please".into(),
        Err(e) => format!("bad body ({}), using defaults", e.status()),
    }
}

/// `Option<Json<T>>` in axum 0.8: no content-type -> None; broken JSON -> still an error.
async fn optional(body: Option<Json<NewOrder>>) -> String {
    match body {
        Some(Json(o)) => format!("order for {}", o.sku),
        None => "no body, fine".into(),
    }
}

// --- middleware -> handler: request extensions -------------------------------------------

#[derive(Debug, Clone)]
pub struct CurrentTenant(pub String);

/// `$request->attributes->set(...)` from Laravel: the middleware puts, the handler takes.
async fn resolve_tenant(mut req: Request, next: Next) -> Response {
    let host = req
        .headers()
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let tenant = host.split('.').next().unwrap_or("public").to_owned();
    req.extensions_mut().insert(CurrentTenant(tenant));
    next.run(req).await
}

async fn tenant(Extension(CurrentTenant(name)): Extension<CurrentTenant>) -> String {
    format!("tenant: {name}")
}

pub fn app() -> Router {
    Router::new()
        .route("/orders/{id}", get(show))
        .route("/orders", post(create))
        .route("/lenient", post(lenient))
        .route("/optional", post(optional))
        .route(
            "/tenant",
            get(tenant).layer(middleware::from_fn(resolve_tenant)),
        )
}

/// The same handler without the middleware: `Extension` is missing -> 500.
pub fn app_without_tenant_middleware() -> Router {
    Router::new().route("/tenant", get(tenant))
}
