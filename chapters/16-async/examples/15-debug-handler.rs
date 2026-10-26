//! `#[axum::debug_handler]` — human-readable errors for wrong handler signatures
//!
//! If you put `Json` BEFORE `Path`, the usual error is `the trait bound ... Handler<_, _>
//! is not satisfied`. With the attribute: "Json<_> consumes the request body and thus must be
//! the last argument to the handler function". Here the order is already correct.
//! (Needs the `macros` feature: `cargo add axum --features macros`.)

use axum::{Json, Router, extract::Path, routing::post};

#[axum::debug_handler]
async fn hello(Path(id): Path<u64>, Json(body): Json<serde_json::Value>) -> String {
    format!("{id}: {body}")
}

fn main() {
    // We only build the router: this example is about compiling, not running a server.
    let _app: Router = Router::new().route("/hello/{id}", post(hello));
    println!("the handler has a correct signature — it compiles");
}
