//! Exercise 6: `AppQuery<T>` — query-string errors as JSON, and repeated keys via axum_extra's Query
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
// `cargo test -p ch16g-axum-deeper --example exercise-6` runs the test below.

use axum::{
    Json, Router,
    extract::FromRequestParts,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use axum_extra::extract::{Query as MultiQuery, QueryRejection as MultiQueryRejection};
use serde::Deserialize;

pub struct QueryError(MultiQueryRejection);

impl From<MultiQueryRejection> for QueryError {
    fn from(e: MultiQueryRejection) -> Self {
        Self(e)
    }
}

impl IntoResponse for QueryError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({ "error": self.0.body_text() }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

/// axum_extra's `Query` understands `?status=a&status=b`; the wrapper makes errors JSON.
#[derive(FromRequestParts)]
#[from_request(via(MultiQuery), rejection(QueryError))]
pub struct AppQuery<T>(pub T);

#[derive(Debug, Deserialize)]
pub struct Filter {
    #[serde(default)]
    pub status: Vec<String>,
    pub page: Option<u32>,
}

async fn list(AppQuery(f): AppQuery<Filter>) -> String {
    format!("statuses={:?} page={}", f.status, f.page.unwrap_or(1))
}

pub fn app() -> Router {
    Router::new().route("/orders", get(list))
}

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(app()).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn call(uri: &str) -> (StatusCode, String) {
        let res = app()
            .oneshot(Request::get(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = res.status();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn repeated_keys_and_json_errors() {
        let (s, b) = call("/orders?status=paid&status=draft").await;
        assert_eq!(
            (s, b.as_str()),
            (StatusCode::OK, r#"statuses=["paid", "draft"] page=1"#)
        );
        let (s, b) = call("/orders?page=-1").await;
        assert_eq!(s, StatusCode::BAD_REQUEST);
        assert!(
            b.starts_with(r#"{"error":"Failed to deserialize query string"#),
            "{b}"
        );
    }
}
