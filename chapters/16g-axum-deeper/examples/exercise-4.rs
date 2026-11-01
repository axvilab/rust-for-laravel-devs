//! Exercise 4: accept only real PNG/JPEG uploads — check the first bytes, not the client's content-type
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
// `cargo test -p ch16g-axum-deeper --example exercise-4` runs the test below.

use axum::{
    Router,
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    routing::post,
};

/// What the file really is, by its "magic bytes".
fn sniff(first: &[u8]) -> Option<&'static str> {
    if first.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("png")
    } else if first.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("jpg")
    } else {
        None
    }
}

async fn avatar(mut form: Multipart) -> Result<String, (StatusCode, String)> {
    let bad = |e: axum::extract::multipart::MultipartError| (e.status(), e.body_text());
    let mut field = form
        .next_field()
        .await
        .map_err(bad)?
        .ok_or((StatusCode::UNPROCESSABLE_ENTITY, "no file".to_string()))?;
    // in theory a chunk may be shorter than 8 bytes; for a real check, buffer until you have 8
    let first = field.chunk().await.map_err(bad)?.unwrap_or_default();
    let Some(kind) = sniff(&first) else {
        return Err((
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "only PNG or JPEG".into(),
        ));
    };
    let mut size = first.len();
    while let Some(chunk) = field.chunk().await.map_err(bad)? {
        size += chunk.len(); // in a real app: write to disk, as in src/uploads.rs
    }
    Ok(format!("{kind}, {size} bytes"))
}

pub fn app() -> Router {
    Router::new()
        .route("/avatar", post(avatar))
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
}

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(app()).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, header},
    };
    use tower::ServiceExt;

    fn upload(content_type: &str, data: &[u8]) -> Request<Body> {
        let mut body =
            b"--B\r\nContent-Disposition: form-data; name=\"a\"; filename=\"x.png\"\r\n".to_vec();
        body.extend(format!("Content-Type: {content_type}\r\n\r\n").bytes());
        body.extend(data);
        body.extend(b"\r\n--B--\r\n");
        Request::post("/avatar")
            .header(header::CONTENT_TYPE, "multipart/form-data; boundary=B")
            .body(Body::from(body))
            .unwrap()
    }

    #[tokio::test]
    async fn png_passes_fake_png_does_not() {
        let png = upload("image/png", b"\x89PNG\r\n\x1a\nrest-of-file");
        assert_eq!(app().oneshot(png).await.unwrap().status(), StatusCode::OK);
        // the client SAYS image/png, but it's a script
        let fake = upload("image/png", b"<?php system($_GET['c']);");
        assert_eq!(
            app().oneshot(fake).await.unwrap().status(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
    }
}
