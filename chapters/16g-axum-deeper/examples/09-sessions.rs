//! Sessions with tower-sessions: a counter, login with `cycle_id`, flash messages, CsrfLayer
// long-running: a server on http://127.0.0.1:3000, stop it with Ctrl+C
//   curl -s -c jar -b jar localhost:3000/visits     → visit number 1, 2, 3...

#[tokio::main]
async fn main() {
    ch16g_axum_deeper::serve(ch16g_axum_deeper::sessions::app()).await;
}
