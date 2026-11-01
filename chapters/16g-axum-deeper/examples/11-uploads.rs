//! File uploads: Multipart, DefaultBodyLimit::max, streaming to disk, safe file names
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
//   curl -s -F 'avatar=@photo.png' localhost:3000/upload
// Files go to $TMPDIR/ch16g-uploads; the limit is 1 MB.

#[tokio::main]
async fn main() {
    let dir = std::env::temp_dir().join("ch16g-uploads");
    tokio::fs::create_dir_all(&dir).await.unwrap();
    println!("saving to {}", dir.display());
    ch16g_axum_deeper::serve(ch16g_axum_deeper::uploads::app(dir, 1024 * 1024)).await;
}
