//! File uploads: `Multipart`, `DefaultBodyLimit`, streaming each field to disk chunk by chunk.

use std::path::{Path, PathBuf};

use axum::{
    Json, Router,
    extract::{
        DefaultBodyLimit, Multipart, State,
        multipart::{Field, MultipartError},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use serde::Serialize;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Clone)]
pub struct Uploads {
    pub dir: PathBuf,
}

#[derive(Debug, Serialize)]
pub struct Saved {
    pub field: String,
    pub original_name: String,
    pub stored_as: String,
    pub bytes: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error(transparent)]
    Multipart(#[from] MultipartError),
    #[error("disk error")]
    Io(#[from] std::io::Error),
    #[error("no file in the request")]
    NoFile,
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let status = match &self {
            // 400 for a broken form, 413 when a size limit was hit
            Self::Multipart(e) => e.status(),
            Self::Io(e) => {
                tracing::error!(error = %e, "upload io");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NoFile => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let msg = match &self {
            Self::Multipart(e) => e.body_text(),
            other => other.to_string(),
        };
        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}

/// Never trust the client's file name: keep only the extension, and only a safe one.
pub fn safe_extension(name: &str) -> &'static str {
    let ext = Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    match ext.to_ascii_lowercase().as_str() {
        "png" => "png",
        "jpg" | "jpeg" => "jpg",
        "pdf" => "pdf",
        "txt" => "txt",
        _ => "bin",
    }
}

/// Copies one field to `path` chunk by chunk: a 1 GB file never sits in memory.
async fn save_field(field: &mut Field<'_>, path: &Path) -> Result<usize, UploadError> {
    let mut file = File::create(path).await?;
    let mut bytes = 0;
    while let Some(chunk) = field.chunk().await? {
        bytes += chunk.len();
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    Ok(bytes)
}

async fn upload(
    State(st): State<Uploads>,
    mut form: Multipart,
) -> Result<Json<Vec<Saved>>, UploadError> {
    let mut saved = Vec::new();
    while let Some(mut field) = form.next_field().await? {
        let field_name = field.name().unwrap_or("").to_owned();
        let Some(original) = field.file_name().map(str::to_owned) else {
            continue; // a plain text field, not a file
        };
        let stored_as = format!("{}.{}", uuid::Uuid::new_v4(), safe_extension(&original));
        let path = st.dir.join(&stored_as);
        match save_field(&mut field, &path).await {
            Ok(bytes) => saved.push(Saved {
                field: field_name,
                original_name: original,
                stored_as,
                bytes,
            }),
            Err(e) => {
                let _ = tokio::fs::remove_file(&path).await; // no half-written files left behind
                return Err(e);
            }
        }
    }
    if saved.is_empty() {
        return Err(UploadError::NoFile);
    }
    Ok(Json(saved))
}

pub fn app(dir: PathBuf, max_bytes: usize) -> Router {
    Router::new()
        .route("/upload", post(upload))
        // axum's default is 2 MB for the whole body; for uploads we set our own
        .layer(DefaultBodyLimit::max(max_bytes))
        .with_state(Uploads { dir })
}
