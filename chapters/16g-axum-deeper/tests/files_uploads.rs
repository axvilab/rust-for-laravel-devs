mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use ch16g_axum_deeper::{files, uploads};
use common::{get, send};

#[tokio::test]
async fn spa_routes_get_index_html_with_200() {
    let res = send(files::app(), get("/orders/42")).await;
    assert_eq!(res.status, StatusCode::OK);
    assert!(res.body.contains("The SPA shell"));
    assert_eq!(res.headers[header::CACHE_CONTROL], "no-cache");
}

#[tokio::test]
async fn hashed_assets_are_immutable_but_404_is_not() {
    let res = send(files::app(), get("/assets/app-3f9a1c.js")).await;
    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(
        res.headers[header::CACHE_CONTROL],
        "public, max-age=31536000, immutable"
    );
    assert!(res.headers.contains_key(header::ETAG));

    let res = send(files::app(), get("/assets/nope.js")).await;
    assert_eq!(res.status, StatusCode::NOT_FOUND);
    assert_eq!(res.headers[header::CACHE_CONTROL], "no-cache");
}

#[tokio::test]
async fn api_has_its_own_json_404() {
    let res = send(files::app(), get("/api/nope")).await;
    assert_eq!(res.status, StatusCode::NOT_FOUND);
    assert_eq!(res.body, r#"{"error":"not found"}"#);
}

#[tokio::test]
async fn etag_gives_304() {
    let res = send(files::app(), get("/assets/app-3f9a1c.js")).await;
    let etag = res.headers[header::ETAG].clone();
    let req = Request::get("/assets/app-3f9a1c.js")
        .header(header::IF_NONE_MATCH, etag)
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        send(files::app(), req).await.status,
        StatusCode::NOT_MODIFIED
    );
}

#[tokio::test]
async fn not_found_service_keeps_404() {
    let res = send(files::app_with_404(), get("/orders/42")).await;
    assert_eq!(res.status, StatusCode::NOT_FOUND);
    assert!(res.body.contains("The SPA shell"));
}

#[tokio::test]
async fn traversal_does_not_escape_dist() {
    let res = send(files::app(), get("/../Cargo.toml")).await;
    assert!(!res.body.contains("[package]"));
}

fn multipart(parts: &[(&str, Option<&str>, Vec<u8>)]) -> Request<Body> {
    let boundary = "XBOUNDARYX";
    let mut body = Vec::new();
    for (name, file, data) in parts {
        body.extend(format!("--{boundary}\r\n").bytes());
        match file {
            Some(f) => body.extend(
                format!(
                    "Content-Disposition: form-data; name=\"{name}\"; filename=\"{f}\"\r\n\r\n"
                )
                .bytes(),
            ),
            None => body
                .extend(format!("Content-Disposition: form-data; name=\"{name}\"\r\n\r\n").bytes()),
        }
        body.extend(data);
        body.extend(b"\r\n");
    }
    body.extend(format!("--{boundary}--\r\n").bytes());
    Request::post("/upload")
        .header(
            header::CONTENT_TYPE,
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap()
}

fn tmp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("ch16g-test-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[tokio::test]
async fn upload_streams_to_disk_with_a_safe_name() {
    let dir = tmp_dir("ok");
    let app = uploads::app(dir.clone(), 1024 * 1024);
    let req = multipart(&[
        ("title", None, b"hello".to_vec()),
        ("doc", Some("../../etc/passwd"), b"root:x".to_vec()),
    ]);
    let res = send(app, req).await;
    assert_eq!(res.status, StatusCode::OK, "{}", res.body);
    let saved: serde_json::Value = serde_json::from_str(&res.body).unwrap();
    let stored = saved[0]["stored_as"].as_str().unwrap();
    assert!(
        stored.ends_with(".bin") && !stored.contains('/'),
        "{stored}"
    );
    assert_eq!(std::fs::read(dir.join(stored)).unwrap(), b"root:x");
}

#[tokio::test]
async fn too_big_upload_is_413_and_leaves_no_file() {
    let dir = tmp_dir("big");
    let app = uploads::app(dir.clone(), 1000);
    let res = send(
        app,
        multipart(&[("doc", Some("big.pdf"), vec![b'x'; 5000])]),
    )
    .await;
    assert_eq!(res.status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(res.body, r#"{"error":"Request payload is too large"}"#);
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
}

#[tokio::test]
async fn form_without_files_is_422() {
    let app = uploads::app(tmp_dir("none"), 1000);
    let res = send(app, multipart(&[("title", None, b"hi".to_vec())])).await;
    assert_eq!(res.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[test]
fn safe_extension_whitelist() {
    assert_eq!(uploads::safe_extension("photo.JPEG"), "jpg");
    assert_eq!(uploads::safe_extension("shell.php"), "bin");
    assert_eq!(uploads::safe_extension("no-extension"), "bin");
}
