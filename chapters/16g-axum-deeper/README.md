# Axum, deeper: layers, cookies, files and real time

Code for this chapter of the book. Every example is a separate program in `examples/`.

```bash
cargo run -p ch16g-axum-deeper --example 01-layer-order
# without Rust installed:
docker run --rm -v "$PWD":/w -w /w rust:1-bookworm cargo run -p ch16g-axum-deeper --example 01-layer-order
```

Run the commands from the repository root.

## Examples from the text

| Example | What it shows | Run |
|---|---|---|
| [`01-layer-order`](examples/01-layer-order.rs) | The order of `.layer()`: the last one added is the outermost; `ServiceBuilder` reads top to bottom | `cargo run -p ch16g-axum-deeper --example 01-layer-order` |
| [`02-timing-layer`](examples/02-timing-layer.rs) | A hand-written tower Layer + Service that adds x-response-time-ms | `cargo run -p ch16g-axum-deeper --example 02-timing-layer` |
| [`03-tower-http-stack`](examples/03-tower-http-stack.rs) | request id, TraceLayer, CORS, compression, timeout, body limit — one ServiceBuilder | `cargo run -p ch16g-axum-deeper --example 03-tower-http-stack` |
| [`04-route-layer`](examples/04-route-layer.rs) | `.layer()` vs `.route_layer()`: an auth middleware turns unknown paths into 401 or leaves them 404 | `cargo run -p ch16g-axum-deeper --example 04-route-layer` |
| [`05-load-shed`](examples/05-load-shed.rs) | tower middleware behind HandleErrorLayer: load_shed + concurrency_limit + timeout | `cargo run -p ch16g-axum-deeper --example 05-load-shed` |
| [`06-rate-limit`](examples/06-rate-limit.rs) | Per-IP rate limiting with tower_governor (needs ConnectInfo) | `cargo run -p ch16g-axum-deeper --example 06-rate-limit` |
| [`07-rate-limit-json`](examples/07-rate-limit-json.rs) | tower_governor with a custom JSON 429 response | `cargo run -p ch16g-axum-deeper --example 07-rate-limit-json` |
| [`08-cookies`](examples/08-cookies.rs) | Cookies: plain `CookieJar`, `SignedCookieJar`, `PrivateCookieJar` and the key in the state | `cargo run -p ch16g-axum-deeper --example 08-cookies` |
| [`09-sessions`](examples/09-sessions.rs) | Sessions with tower-sessions: a counter, login with `cycle_id`, flash messages, CsrfLayer | `cargo run -p ch16g-axum-deeper --example 09-sessions` |
| [`10-static-spa`](examples/10-static-spa.rs) | Static files and an SPA: ServeDir, ServeFile fallback, an API with its own 404, cache headers | `cargo run -p ch16g-axum-deeper --example 10-static-spa` |
| [`11-uploads`](examples/11-uploads.rs) | File uploads: Multipart, DefaultBodyLimit::max, streaming to disk, safe file names | `cargo run -p ch16g-axum-deeper --example 11-uploads` |
| [`12-realtime`](examples/12-realtime.rs) | Real time: broadcast channel + SSE (/events) + WebSocket (/ws), with a graceful shutdown that ends the streams | `cargo run -p ch16g-axum-deeper --example 12-realtime` |
| [`13-extractors`](examples/13-extractors.rs) | Extractors, deeper: AppPath/AppJson with JSON rejections, ApiVersion, Option and Result, Extension | `cargo run -p ch16g-axum-deeper --example 13-extractors` |

## Solutions to the “Try it” exercises

| Example | What it shows | Run |
|---|---|---|
| [`exercise-1`](examples/exercise-1.rs) | Exercise 1: SetRequestIdLayer above vs below TraceLayer — whose span gets the id | `cargo run -p ch16g-axum-deeper --example exercise-1` |
| [`exercise-2`](examples/exercise-2.rs) | Exercise 2: two rate limits — /login 5 tries a minute per IP, the rest 20 in a burst | `cargo run -p ch16g-axum-deeper --example exercise-2` |
| [`exercise-3`](examples/exercise-3.rs) | Exercise 3: post a form, flash "order created", redirect (303), show the message exactly once | `cargo run -p ch16g-axum-deeper --example exercise-3` |
| [`exercise-4`](examples/exercise-4.rs) | Exercise 4: accept only real PNG/JPEG uploads — check the first bytes, not the client's content-type | `cargo run -p ch16g-axum-deeper --example exercise-4` |
| [`exercise-5`](examples/exercise-5.rs) | Exercise 5: an SSE clock — one event a second — that TimeoutLayer does not cut off and Ctrl+C ends | `cargo run -p ch16g-axum-deeper --example exercise-5` |
| [`exercise-6`](examples/exercise-6.rs) | Exercise 6: `AppQuery<T>` — query-string errors as JSON, and repeated keys via axum_extra's Query | `cargo run -p ch16g-axum-deeper --example exercise-6` |

## The crate itself

```bash
cargo test -p ch16g-axum-deeper                     # unit, integration and doc tests
```

## Intentionally broken examples

`broken/` holds code that the book shows as a compile error. Cargo does not build these files.
The header of each file states the expected error and the fix.
To check that they really fail, and with exactly that error:

```bash
./scripts/check-broken.sh 16
cp chapters/16g-axum-deeper/broken/01-tower-error-without-handle-error.rs chapters/16g-axum-deeper/examples/zz-broken.rs
cargo build -p ch16g-axum-deeper --example zz-broken          # see the compiler message
rm chapters/16g-axum-deeper/examples/zz-broken.rs
```

| File | What it shows | Expected error |
|---|---|---|
| [`01-tower-error-without-handle-error.rs`](broken/01-tower-error-without-handle-error.rs) | tower's own `TimeoutLayer` straight into `Router::layer`: it returns errors, axum wants `Infallible` | the trait bound `Infallible: From<Box<(dyn std::error::Error + Send + Sync + 'static)>>` is not satisfied |
| [`02-tower-rate-limit-not-clone.rs`](broken/02-tower-rate-limit-not-clone.rs) | tower's `RateLimit` is not `Clone`, and axum clones services for every request | the trait `Clone` is not implemented for `HandleError<RateLimit<Route>, ...>` |
| [`03-option-query.rs`](broken/03-option-query.rs) | `Option<Query<T>>`: in axum 0.8 `Query` does not implement `OptionalFromRequestParts` | the trait bound `std::option::Option<Query<Filter>>: FromRequest<(), _>` is not satisfied |
| [`04-ws-text-from-string.rs`](broken/04-ws-text-from-string.rs) | axum 0.8 WebSocket: `Message::Text` holds `Utf8Bytes`, not `String` | error[E0308]: mismatched types ... expected `Utf8Bytes`, found `String` |
