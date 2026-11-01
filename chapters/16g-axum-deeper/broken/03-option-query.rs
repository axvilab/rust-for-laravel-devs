//! `Option<Query<T>>`: in axum 0.8 `Query` does not implement `OptionalFromRequestParts`
//!
//! DOES NOT COMPILE — on purpose.
//! Expected error: the trait bound `std::option::Option<Query<Filter>>: FromRequest<(), _>` is not satisfied
//! Fix: make the fields optional instead — `Query<Filter>` with `status: Option<String>`,
//! see src/extractors.rs and the chapter text.
// expected-error: the trait bound `std::option::Option<Query<Filter>>: FromRequest<(), _>` is not satisfied
// check-with: cargo

use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct Filter {
    status: String,
}

#[axum::debug_handler]
async fn list(filter: Option<Query<Filter>>) -> String {
    match filter {
        Some(Query(f)) => f.status,
        None => "all".into(),
    }
}

fn main() {
    let _app: Router = Router::new().route("/orders", get(list));
}
